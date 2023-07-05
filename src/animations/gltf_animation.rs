use std::collections::HashMap;
use crate::objects::gltf_mesh::{KeyFrame, Animation};
use crate::animations::{skeleton::{Skeleton}};
use crate::typedef::V3;
use std::rc::Rc;

pub type MeshName = String;
pub type AnimationName = String;

#[derive(Debug)]
struct ActiveAnimation {
    anim: Rc::<Animation>,
    repeat: bool,
    frame: usize,
    elapsed: f32,
    root_motion: V3,
    speed: f32
}


pub struct Start<AnimationId> {
    pub anim: Rc::<Animation>,
    pub repeat: bool,
    pub id: AnimationId,
    pub speed: f32,
}


pub type Animations = HashMap::<String, Animation>;


/*
pub struct Animations {
    next_id: AnimationId,
    id_to_name: HashMap::<AnimationId, AnimationName>,
    animations: HashMap::<AnimationId, Animation>,
    mesh_to_animations: HashMap::<AnimationName, Vec::<AnimationId>>
}
*/

#[derive(Debug, Default)]
pub struct AnimationPlayer<AnimationId> where AnimationId : Clone + Copy + Eq + std::hash::Hash + std::default::Default {
    animations: HashMap::<AnimationId, ActiveAnimation>,
    clear_buffer: Vec::<AnimationId>,
    tmp_keyframe: KeyFrame
}

impl<AnimationId: Clone + Copy + Eq + std::hash::Hash + std::default::Default> AnimationPlayer<AnimationId> {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, dt: f32) {
        // just update dt for each active animation
        // and clear non repeating finished animations

        self.clear_buffer.clear();

        for (id, active) in &mut self.animations {
            active.elapsed += dt * active.speed;

            //println!("Total ,  elapsed{:?}", (active.anim.total_secs, active.elapsed));

            if active.anim.total_secs > active.elapsed {
                // set active.frame until we are at end, or current frame end after currently elapsed time
                while active.frame < (active.anim.frames.len() - 1) &&
                    active.anim.frames[active.frame].end_time() < active.elapsed {
                        //println!("{:.2?}", (active.anim.frames[active.frame].end_time(), active.elapsed));
                    active.frame = usize::min(active.anim.frames.len() - 1 , active.frame  + 1);
                }
            } else {
                if !active.repeat {
                    self.clear_buffer.push(*id);
                }
                else {
                    active.elapsed = active.anim.total_secs - active.elapsed;
                    // this might not be correct if a frame is so short we skip the first keyframe in animation
                    active.frame = 0;
                }
            }


            // update root motion
            if let Some(rm) = &active.anim.root_motion {

                let next = usize::min(active.frame + 1, active.anim.frames.len()-1);

                let frame = &active.anim.frames[active.frame];
                // for take next, or last, no cyclic, can be implemented to either take last or cyclic so first using %
                let next_frame = &active.anim.frames[next];
                let motion = &rm[active.frame];
                // for take next, or last, no cyclic, can be implemented to either take last or cyclic so first using %
                let next_motion = &rm[next];

                let t = (active.elapsed - frame.start_sec) / frame.length_sec;
                active.root_motion = motion.lerp(next_motion, t);
            }
        }

        for id in &self.clear_buffer {
            self.animations.remove(id);
        }
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }


    pub fn start(&mut self, start: Start<AnimationId>) -> AnimationId {
        let id = start.id;
        // make sure tmp keyframes always has enought joints
        if self.tmp_keyframe.joints.len() < start.anim.frames[0].joints.len() {
            for _i in 0..(start.anim.frames[0].joints.len() - self.tmp_keyframe.joints.len()) {
                self.tmp_keyframe.joints.push(Default::default());
            }
        }

        self.animations.insert(id,
                               ActiveAnimation {
                                   anim: start.anim,
                                   repeat: start.repeat,
                                   frame: 0,
                                   elapsed: 0.0,
                                   root_motion: V3::new(0.0, 0.0, 0.0),
                                   speed: start.speed
                               });
        id
    }


    pub fn root_motion(&self, id: &AnimationId) -> V3 {
        if let Some(active) = self.animations.get(id) {
            return active.root_motion;
        }
        V3::new(0.0, 0.0, 0.0)
    }

    pub fn expired(&self, id: &AnimationId) -> bool {
        !self.animations.contains_key(id)
    }

    pub fn change_speed(&mut self, id: &AnimationId, speed: f32) {
        if let Some(active) = self.animations.get_mut(&id) {
            active.speed = speed;
        }
    }

    pub fn update_skeleton(&mut self, anim_id: AnimationId, skeleton: &mut Skeleton) {
        if let Some(active) = self.animations.get(&anim_id) {
            // update skeleton

            let frame = &active.anim.frames[active.frame];
            // for take next, or last, no cyclic, can be implemented to either take last or cyclic so first using %
            let next_frame = &active.anim.frames[usize::min(active.frame + 1, active.anim.frames.len()-1)];

            // get how far into the frame we are, divided bt frame length
            let t = (active.elapsed - frame.start_sec) / frame.length_sec;

            frame.interpolate(next_frame, t, &mut self.tmp_keyframe);

            //println!("elapsed, length, frame {:?}", (active.elapsed, frame.length_sec, active.frame));

            update_skeleton_to_key_frame(skeleton, &self.tmp_keyframe);
        }
    }
}

pub fn update_skeleton_to_key_frame(skeleton: &mut Skeleton, key_frame: &KeyFrame) {
    // interpolate joints new transformation
    for i in 0..skeleton.joints.len() {
        let rotation = key_frame.joints[i].rotation;
        let translation = key_frame.joints[i].translation;
        skeleton.update_joint_matrices(i, rotation, translation);
    }
}

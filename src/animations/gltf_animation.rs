use std::collections::HashMap;
use crate::objects::gltf_mesh::{KeyFrame, Animation};
use crate::animations::{skeleton::{Skeleton, Bones}};
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
    speed: f32,
    transition: Option<Transition>,
    expired: bool
}

#[derive(Debug)]
struct Transition {
    elapsed: f32,
    time: f32, //how long transition from start_frame to first frame in animaiton should be
    start_frame: KeyFrame
}


pub struct Start<AnimationId> {
    pub anim: Rc::<Animation>,
    pub repeat: bool,
    pub id: AnimationId,
    pub speed: f32,
    pub transition: Option<StartTransition>
}

pub struct StartTransition {
    pub start_frame: KeyFrame, // maybe make this Rc if we can?
    pub time: f32, //how long transition from start_frame to first frame in animaiton should be
}


pub type Animations = HashMap::<String, Animation>;


#[derive(Debug, Default)]
pub struct AnimationPlayer<AnimationId>
where AnimationId : Clone + Copy + Eq + std::hash::Hash + std::default::Default + std::fmt::Debug
{
    animations: HashMap::<AnimationId, ActiveAnimation>,
    tmp_keyframe: KeyFrame
}

impl<AnimationId> AnimationPlayer<AnimationId>
where AnimationId: Clone + Copy + Eq + std::hash::Hash + std::default::Default + std::fmt::Debug {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, dt: f32) {
        // just update dt for each active animation
        // and set non repeating animations as expiredclear non repeating finished animations

        for (_id, active) in &mut self.animations {
            // we don't want to remove expired animation automaticly, since we might still need the
            // keyframe info for transitioning
            if active.expired {
                continue;
            }

            // first do transition;
            if let Some(trans) = &mut active.transition {

                trans.elapsed += dt;
                // check if transition is done
                if trans.elapsed > trans.time {
                    active.elapsed += trans.time - trans.elapsed;
                    active.transition = None;
                } else {
                    // TODO: transition - handle root motion some how? Old animation should be done,
                    // so root motion should be updated be aniumation
                    continue;
                }
            }

            active.elapsed += dt * active.speed;

            if active.anim.total_secs > active.elapsed {
                // set active.frame until we are at end, or current frame end after currently elapsed time
                while active.frame < (active.anim.frames.len() - 1) &&
                    active.anim.frames[active.frame].end_time() < active.elapsed {
                    active.frame = usize::min(active.anim.frames.len() - 1 , active.frame  + 1);
                }
            } else {
                if !active.repeat {
                    active.expired = true;
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
                let _next_frame = &active.anim.frames[next];
                let motion = &rm[active.frame];
                // for take next, or last, no cyclic, can be implemented to either take last or cyclic so first using %
                let next_motion = &rm[next];

                let t = (active.elapsed - frame.start_sec) / frame.length_sec;
                active.root_motion = motion.lerp(next_motion, t);
            }
        }
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }

    pub fn key_frame(&mut self, id: &AnimationId) -> Option::<KeyFrame> {
        if let Some(active) = self.animations.get(id) {
            let mut frame = &active.anim.frames[active.frame];
            // for take next, or last, no cyclic, can be implemented to either take last or cyclic so first using %
            let mut next_frame = &active.anim.frames[usize::min(active.frame + 1, active.anim.frames.len()-1)];
            let mut t = (active.elapsed - frame.start_sec) / frame.length_sec;

            if let Some(trans) = &active.transition {
                frame = &trans.start_frame;
                next_frame = &active.anim.frames[0];
                t = trans.elapsed / trans.time;
            }


            frame.interpolate(next_frame, t, &mut self.tmp_keyframe);
            return Some(self.tmp_keyframe.clone());
        }

        None

    }


    pub fn start(&mut self, start: Start<AnimationId>) -> AnimationId {
        let id = start.id;
        // make sure tmp keyframes always has enought joints
        if self.tmp_keyframe.joints.len() < start.anim.frames[0].joints.len() {
            for _i in 0..(start.anim.frames[0].joints.len() - self.tmp_keyframe.joints.len()) {
                self.tmp_keyframe.joints.push(Default::default());
            }
        }

        let trans = if let Some(t) = start.transition {
            Some(Transition {
                elapsed: 0.0,
                start_frame: t.start_frame,
                time: t.time
            })
        } else {
            None
        };
        self.animations.insert(id,
                               ActiveAnimation {
                                   anim: start.anim,
                                   repeat: start.repeat,
                                   frame: 0,
                                   elapsed: 0.0,
                                   root_motion: V3::new(0.0, 0.0, 0.0),
                                   speed: start.speed,
                                   transition: trans,
                                   expired: false
                               });
        id
    }


    pub fn root_motion(&self, id: &AnimationId) -> V3 {
        if let Some(active) = self.animations.get(id) {
            return active.root_motion;
        }
        V3::new(0.0, 0.0, 0.0)
    }


    pub fn removed(&self, id: &AnimationId) -> bool {
        !self.animations.contains_key(id)
    }

    pub fn expired(&self, id: &AnimationId) -> bool {
        if let Some(active) = self.animations.get(id) {
            return active.expired;
        }
        true
    }

    pub fn change_speed(&mut self, id: &AnimationId, speed: f32) {
        if let Some(active) = self.animations.get_mut(&id) {
            active.speed = speed;
        }
    }

    pub fn update_skeleton_and_bones(&mut self, anim_id: AnimationId, skeleton: &mut Skeleton, bones: &mut Bones) {
        if let Some(active) = self.animations.get(&anim_id) {
            let mut frame = &active.anim.frames[active.frame];
            // for take next, or last, no cyclic, can be implemented to either take last or cyclic so first using %
            let mut next_frame = &active.anim.frames[usize::min(active.frame + 1, active.anim.frames.len()-1)];
            let mut t = (active.elapsed - frame.start_sec) / frame.length_sec;

            if let Some(trans) = &active.transition {
                frame = &trans.start_frame;
                next_frame = &active.anim.frames[0];
                t = trans.elapsed / trans.time;
            }

            frame.interpolate(next_frame, t, &mut self.tmp_keyframe);
            update_skeleton_to_key_frame(skeleton, &self.tmp_keyframe);
            skeleton.set_all_bones_from_skeleton(bones);
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

use std::collections::HashMap;
use crate::objects::gltf_mesh::{KeyFrame, Animation};
use crate::animations::{clamp01, skeleton::{self, Skeleton, Bones}};


pub type MeshName = String;
pub type AnimationName = String;
pub type AnimationId = usize;

#[derive(Debug)]
struct ActiveAnimation<'a> {
    anim: &'a Animation,
    repeat: bool,
    frame: usize,
    elapsed: f32,
}


pub struct Start<'a> {
    pub anim: &'a Animation,
    pub repeat: bool,
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
pub struct AnimationPlayer<'a> {
    animations: HashMap::<AnimationId, ActiveAnimation<'a>>,
    next_id: AnimationId,
    clear_buffer: Vec::<AnimationId>,
    tmp_keyframe: KeyFrame
}

impl<'a> AnimationPlayer<'a> {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, dt: f32) {
        // just update dt for each active animation
        // and clear non repeating finished animations

        self.clear_buffer.clear();

        for (id, active) in &mut self.animations {
            active.elapsed += dt;

            //println!("Total ,  elapsed{:?}", (active.anim.total_secs, active.elapsed));

            if active.anim.total_secs > active.elapsed {
                // set active.frame until we are at end, or current frame end after currently elapsed time
                while(active.frame < (active.anim.frames.len() - 1) &&
                      active.anim.frames[active.frame].end_time() < active.elapsed) {
                    //println!("{:.2?}", (active.anim.frames[active.frame].end_time(), active.elapsed));
                    active.frame = usize::min(active.anim.frames.len() - 1 , active.frame  + 1);
                }
            } else {
                if !active.repeat {
                    self.clear_buffer.push(*id);
                }
                else {
                    active.elapsed = 0.0;
                    active.frame = 0;
                }
            }

            //println!("{:.2?}", (active.anim.total_secs, active.elapsed, active.frame));
        }

        for id in &self.clear_buffer {
            self.animations.remove(id);
        }
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }


    pub fn start(&mut self, start: Start<'a>) -> AnimationId {
        let id = self.next_id;

        // make sure tmp keyframes always has enought joints
        if self.tmp_keyframe.joints.len() < start.anim.frames[0].joints.len() {
            for i in 0..(start.anim.frames[0].joints.len() - self.tmp_keyframe.joints.len()) {
                self.tmp_keyframe.joints.push(Default::default());
            }
        }

        self.animations.insert(id,
                               ActiveAnimation {
                                   anim: start.anim,
                                   repeat: start.repeat,
                                   frame: 0,
                                   elapsed: 0.0,
                               });
        self.next_id += 1;
        id
    }


    pub fn expired(&self, id: AnimationId) -> bool {
        !self.animations.contains_key(&id)
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

/*
    pub fn get(&self, id: AnimationId) -> Option<&Animation> {
        self.animations.get(&id)
    }

    fn add_animation(&mut self, anim: Animation, name: String) -> AnimationId {
        self.next_id += 1;
        self.id_to_name.insert(self.next_id, name);
        self.animations.insert(self.next_id, anim);

        self.next_id
    }


    fn mesh_to_animation(&mut self, mesh_name: &str, animation_id: AnimationId) {
        if !self.mesh_to_animations.contains_key(mesh_name) {
            self.mesh_to_animations.insert(mesh_name.to_string(), vec![]);
        }

        let anims: &mut Vec::<AnimationId> = self.mesh_to_animations.get_mut(mesh_name).unwrap();
        anims.push(animation_id);
    }


    pub fn get_mesh_animations(&self, mesh_name: &str) -> Option<&Vec::<AnimationId>> {
        self.mesh_to_animations.get(mesh_name)
    }

    pub fn get_mesh_name_animation(&self, mesh_name: &str, animation_name: &str) -> Option::<AnimationId> {

        if let Some(ids) = self.mesh_to_animations.get(mesh_name) {
            for id in ids {
                if let Some(id_name) = self.id_to_name.get(id) {
                    if id_name == animation_name {
                        return Some(*id);
                    }
                }
            }
        }

        None
    }

    pub fn get_animations(&self) -> &HashMap::<AnimationName, Vec::<AnimationId>> {
        &self.mesh_to_animations
    }

    pub fn id_to_name(&self) -> &HashMap::<AnimationId, AnimationName> {
        &self.id_to_name
    }


}


impl Animation {

    pub fn update_bones(&self, bones: &mut Bones, skeleton: &mut skeleton::Skeleton, elapsed: f32) {

        let progress = self.next_keyframe(elapsed);

        let t =  progress.t;

        for i in 0..bones.len() {

            // current transfor
            let start_transform = self.frames[progress.cur_frame].joints[i];
            let end_transform = self.frames[progress.next_frame].joints[i];

            // Slerp the rotation
            let rotation = start_transform.rotation.slerp(&end_transform.rotation, t);

            // Linear interpolate the translation
            let translation = start_transform.translation * (1.0 - progress.t) + end_transform.translation * t;

            // we have to update the skeleton to update the

            skeleton.update_joint_matrices(i, rotation, translation);


            // update the bones from skeleton world and inverse bind matrices

            bones[i] = skeleton.joints[i].world_matrix * skeleton.joints[i].inverse_bind_pose;
        }
    }

    /// Maps time to [0;1[ where 0 is start of current frame and 0.999 is almost at next frame
    fn next_keyframe(&self, t: f32) -> FrameProgress {

        let frames_len = self.frames.len();
        // time per frame
        let frame_time = self.total_secs / frames_len as f32;


        let cur_frame = ((t / frame_time).floor() as usize) % frames_len;

        // make sure we don't index out of bounds. Maybe if we want to cycle % with len, so we wrap around to 0
        let next_frame = (cur_frame + 1) % frames_len;

        let min = frame_time * cur_frame as f32;
        let max = frame_time * next_frame as f32;

        let cur_t = clamp01(t, min, max);

        FrameProgress {
            t: cur_t,
            cur_frame,
            next_frame
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct FrameProgress {
    t: f32,
    cur_frame: usize,
    next_frame: usize
}


#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_clamp() {

        let min = 23.0;
        let max = 24.0;
        let v = 23.5;

        let mapped = clamp01(v, min, max);

        let expected = 0.5;
        assert!(f32::abs(mapped - expected) < 0.001);

    }


    #[test]
    fn test_clamp2() {

        let min = 0.2;
        let max = 0.6;
        let v = 0.4;

        let mapped = clamp01(v, min, max);

        let expected = 0.5;
        println!("{:?}", (mapped, expected));
        assert!(f32::abs(mapped - expected) < 0.001);

    }
}
*/

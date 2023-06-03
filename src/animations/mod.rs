#![allow(for_loops_over_fallibles)]
use crate::na;
use crate::math::clamp01;
use crate::animations::skeleton::*;
use std::collections::{HashMap, HashSet};

pub mod skeleton;
mod types;

pub use self::types::*;

pub type MeshName = String;
pub type AnimationName = String;
pub type AnimationId = usize;

#[derive(Debug, Default, Clone)]
pub struct Animations {
    next_id: AnimationId,
    id_to_name: HashMap::<AnimationId, AnimationName>,
    animations: HashMap::<AnimationId, Animation>,
    mesh_to_animations: HashMap::<AnimationName, Vec::<AnimationId>>
}

impl Animations {

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

#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: Vec::<KeyFrame>,
    pub seconds: f32,
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
        let frame_time = self.seconds / frames_len as f32;


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


pub fn load_animations(file_path: &str, skins: &Skins, animations: &mut Animations) {


    let (gltf, buffers, _) = gltf::import(file_path).unwrap();

    for ani in gltf.animations() {

        let name = match ani.name() {
            Some(n) => n.to_string(),
            _ => continue
        };


        let mut skeleton_option = None;
        let mut mesh_names = HashSet::<String>::new();

        // First use the channels to get a node, map that node to a skeleton
        // TODO: Multiple meshes should be allowed here
        for channel in ani.channels() {
            let node_index = channel.target().node().index();

            if let Some(skin_id) = skins.node_index_to_skin.get(&node_index) {
                skeleton_option = skins.skeletons.get(&skin_id);
                mesh_names.insert(skins.skin_to_mesh.get(&skin_id).unwrap().to_string());
            }
        }


        // if we could not map channel target node to skeleton skip animation
        let skeleton = match skeleton_option {
            Some(skele) => skele,
            None => {
                println!("Skipping {:?} could not match skeleton.", name);
                continue;
            }
        };

        let mut joints_indexes: HashMap::<String, usize> = HashMap::new();
        for i in 0..skeleton.joints.len() {
            joints_indexes.insert(skeleton.joints[i].name.clone(), i);
        }

        let mut frames = Vec::new();
        let mut max_frame_count = 0;

        for channel in ani.channels() {

            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let mut frame_count = 0;

            // Just iterate to get the frame_count
            for read_outputs in reader.read_outputs() {
                match read_outputs {
                    gltf::animation::util::ReadOutputs::Translations(ts) => {
                        frame_count = ts.len();
                    },
                    _=> {}
                }
            }

            max_frame_count = usize::max(max_frame_count, frame_count);
        }

        // fill frames with joint data

        for _ in 0..max_frame_count {

            frames.push(KeyFrame {
                joints: skeleton.joints.iter().map(|joint| {
                    Transformation {
                        translation: joint.translation,
                        rotation: joint.rotation
                    }
                }).collect()
            });
        }


        fill_frames(&buffers, &ani, &mut frames, &joints_indexes);

        // TODO: Figure out this fps, playback, using samplers
        // assume 25 fps is animation speed. take number of frames and divide 25

        let dur = frames.len() as f32 / 25.0;
        let animation =  Animation {
            frames,
            seconds: dur
        };


        let id = animations.add_animation(animation, name);

        for mesh_name in &mesh_names {
            animations.mesh_to_animation(mesh_name, id);
        }
    }

    println!("Animations loaded: {:#?}", animations.id_to_name.values());
}


fn fill_frames(buffers: &Vec::<gltf::buffer::Data>, ani: &gltf::Animation, frames: &mut Vec::<KeyFrame>, joints_indexes: &HashMap::<String, usize>) {

    // Each channel
    for channel in ani.channels() {
        let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
        let target = channel.target();

        let joints_index = match joints_indexes.get(target.node().name().unwrap()) {
            Some(i) => *i,
            _ => {
                //println!("Skipping joint {:#?}", target.node().name().unwrap());
                continue;
            }
        };


        for read_outputs in reader.read_outputs() {
            match read_outputs {
                gltf::animation::util::ReadOutputs::Translations(ts) => {
                    let mut i = 0;
                    assert_eq!(frames.len(), ts.len());

                    for t in ts {
                        frames[i].joints[joints_index].translation = na::Vector3::new(t[0], t[1], t[2]);
                        i += 1;
                    }

                },
                gltf::animation::util::ReadOutputs::Rotations(rs) => {
                    let mut i = 0;
                    let rs_f32 = rs.into_f32();
                    assert_eq!(frames.len(), rs_f32.len());

                    for r in rs_f32 {

                        let q = na::Quaternion::from(na::Vector4::new(r[0], r[1], r[2], r[3]));

                        frames[i].joints[joints_index].rotation = na::UnitQuaternion::from_quaternion(q);
                        i += 1 ;
                    }

                },
                gltf::animation::util::ReadOutputs::Scales(ss) => {
                    for s in ss {
                        let diff = f32::abs(3.0 - (s[0] + s[1] + s[2]));
                        if diff > 0.01 {
                            panic!("Scale was more that 0.01 it might be important\n scale was {}", diff)
                        }
                    }
                },
                gltf::animation::util::ReadOutputs::MorphTargetWeights(mtws) => {
                    println!("{:#?}", mtws);
                }
            }
        }
    }
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

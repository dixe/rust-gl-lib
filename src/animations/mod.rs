use crate::na;
use crate::animations::skeleton::*;
use crate::animations::types::*;
use std::collections::HashMap;

pub mod skeleton;
mod types;

pub use self::types::*;


pub type MeshName = String;
pub type AnimationName = String;


pub type Animations = HashMap::<AnimationName, Animation>;

pub type MeshAnimations = HashMap::<MeshName, Animations>;



#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: Vec::<KeyFrame>,
    pub seconds: f32,
}

pub fn load_animations(file_path: &str, skins: &Skins) -> MeshAnimations {

    let (gltf, buffers, _) = gltf::import(file_path).unwrap();


    let mut res = MeshAnimations::new();

    for ani in gltf.animations() {


        let name = match ani.name() {
            Some(n) => n.to_string(),
            _ => continue
        };


        let mut skeleton_option = None;
        let mut mesh_name = "".to_string();

        // First use the channels to get a node, map that node to a skeleton
        for channel in ani.channels() {
            let node_index = channel.target().node().index();

            if let Some(skin_id) = skins.node_index_to_skin.get(&node_index) {
                skeleton_option = skins.skeletons.get(&skin_id);
                mesh_name = skins.skin_to_mesh.get(&skin_id).unwrap().to_string();
            }

            break;
        }


        // if we could not map channel target node to skeleton skip animation
        let skeleton = match skeleton_option {
            Some(skele) => skele,
            None => {
                println!("Skipping {:?} could not match skeleton.", name);
                continue;
            }
        };

        if !res.contains_key(&mesh_name) {
            res.insert(mesh_name.clone(), HashMap::default());
        }


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

        let animations : &mut Animations = res.get_mut(&mesh_name).unwrap();

        animations.insert(name, Animation {
            frames,
            seconds: dur
        });
    }

    res
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

use crate::na;
use failure::err_msg;


#[derive(Debug, Clone)]
pub struct Skeleton {
    pub name: String,
    pub joints: Vec<Joint>,
}

#[derive(Debug, Clone)]
pub struct Joint {
    pub name: String,
    pub parent_index: usize,

    pub inverse_bind_pose: na::Matrix4::<f32>,
    pub world_matrix: na::Matrix4::<f32>,

    pub rotation: na::UnitQuaternion::<f32>,
    pub translation: na::Vector3::<f32>

}

impl Joint {

    pub fn empty() -> Joint {
        Joint {
            name: "Empty".to_string(),
            parent_index: 0,
            inverse_bind_pose: na::Matrix4::identity(),
            world_matrix: na::Matrix4::identity(),
            rotation: na::UnitQuaternion::identity(),
            translation: na::Vector3::identity(),
        }
    }

    fn get_local_matrix(&self) -> na::Matrix4::<f32> {
        let rot_mat = self.rotation.to_homogeneous();

        let trans_mat = na::Matrix4::new_translation(&self.translation);

        trans_mat * rot_mat
    }

    pub fn get_local_matrix_data(&self, rotation: na::UnitQuaternion::<f32>, translation: na::Vector3::<f32>) -> na::Matrix4::<f32> {
        let rot_mat = rotation.to_homogeneous();

        let trans_mat = na::Matrix4::new_translation(&translation);

        trans_mat * rot_mat

    }

    pub fn transformation(&self) -> Transformation {
        Transformation {
            rotation: self.rotation,
            translation: self.translation
        }
    }
}


impl Skeleton {

    fn calc_t_pose(&mut self) {
        for i in 0..self.joints.len() {
            self.set_t_pose_joint(i);
        }
    }

    fn set_t_pose_joint(&mut self, index: usize) {

        let joint = &self.joints[index];

        let local_matrix = joint.get_local_matrix();

        let mut world_matrix = local_matrix;

        if joint.parent_index != 255 {
            world_matrix = self.joints[joint.parent_index].world_matrix * local_matrix;
        }

        if joint.parent_index >= index && joint.parent_index != 255 {
            panic!("Bones are not in correct order. All children should be after parent current {}, parent {}", index, joint.parent_index);
        }

        self.joints[index].world_matrix = world_matrix;
        self.joints[index].inverse_bind_pose = world_matrix.try_inverse().unwrap();

    }



    pub fn from_gltf(file_path: &str) -> Result<(Skeleton, std::collections::HashMap<u16,usize>), failure::Error> {
        let (gltf, _, _) = gltf::import(file_path)?;

        for skin in gltf.skins() {

            //println!("SKING {:#?}", skin.name());

            let mut joints_data = std::collections::HashMap::new();

            // fill the array with joints data
            let mut hip_index = 0;

            for node in skin.joints() {

                let index = node.index();
                let (translation, rotation) = match node.transform() {
                    gltf::scene::Transform::Decomposed {translation, rotation, .. } => {
                        let q = na::Quaternion::from(
                            na::Vector4::new(rotation[0], rotation[1], rotation[2], rotation[3]));
                        let rot = na::UnitQuaternion::from_quaternion(q);
                        (na::Vector3::new(translation[0], translation[1], translation[2]), rot)

                    },
                    _ => { panic!("Non decomposed joints info")}
                };

                if node.name().unwrap() == "hip" {
                    hip_index = index;
                }



                let children: Vec::<usize> = node.children().map(|c| c.index()).collect();


                joints_data.insert(index, (children, node.name().unwrap(), Transformation {
                    translation,
                    rotation
                }));
            }


            // start from hip index and create skeleton from there

            let mut skeleton = Skeleton {
                name: skin.name().unwrap().to_string(),
                joints: Vec::new(),
            };

            let mut index_map = std::collections::HashMap::<u16,usize>::new();
            load_joints(&mut skeleton, &joints_data, hip_index, 255, &mut index_map);

            for i in 0..skeleton.joints.len() {
                println!("{:?} {}", skeleton.joints[i].name, i);
            }

            if !index_map.contains_key(&0) {
                index_map.insert(0, 0);
            }

            let mut walk_target = joints_data[&23].2.translation;
            walk_target.x += 1.0;
            walk_target.z += 0.0;

            //TODO: remove this hardcoded, and find a way to do it more generally

            skeleton.calc_t_pose();

            return Ok((skeleton, index_map));

        }

        Err(err_msg("NO SKELETON LOADED"))
    }

    pub fn set_bones_from_skeleton(&self, bones: &mut [na::Matrix4::<f32>]) {
        for i in 0..self.joints.len() {
            bones[i] = self.joints[i].world_matrix * self.joints[i].inverse_bind_pose;
        }
    }

    pub fn update_joint_matrices(joints: &mut Vec::<Joint>, joint: usize, rotation: na::UnitQuaternion::<f32>, translation: na::Vector3::<f32>) {
        joints[joint].rotation = rotation;
        joints[joint].translation = translation;

        joints[joint].world_matrix = joints[joint].get_local_matrix();

        let parent_index = joints[joint].parent_index;
        if parent_index != 255 {
            joints[joint].world_matrix = joints[parent_index].world_matrix * joints[joint].world_matrix;
        }
    }

}


fn load_joints(skeleton: &mut Skeleton, joints: &std::collections::HashMap<usize, (Vec<usize>, &str, Transformation)>, index: usize, parent_index: usize, index_map: &mut std::collections::HashMap<u16,usize>) {

    let mut joint = Joint::empty();

    joint.rotation = joints[&index].2.rotation;
    joint.translation = joints[&index].2.translation;
    joint.name = joints[&index].1.to_string();

    joint.parent_index = parent_index;

    skeleton.joints.push(joint);

    let this_idx = skeleton.joints.len() - 1;
    index_map.insert(index as u16, this_idx);
    for child_index in &joints[&index].0 {
        load_joints(skeleton, joints, *child_index, this_idx, index_map);
    }

}



#[derive(Debug, Copy, Clone)]
pub struct Transformation {
    pub translation: na::Vector3::<f32>,
    pub rotation: na::UnitQuaternion::<f32>,
}

impl Transformation {

    pub fn identity(joint: &Joint) -> Self {
        Transformation {
            translation: joint.translation,
            rotation: joint.rotation,
        }
    }

    pub fn rotation_euler(joint: &Joint, roll: f32, pitch: f32, yaw: f32) -> Self {
        Transformation {
            translation: joint.translation,
            rotation: na::UnitQuaternion::from_euler_angles(roll, pitch, yaw),
        }
    }
}

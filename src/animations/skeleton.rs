use crate::na;
use crate::animations::types::*;
use crate::collision3d;
use std::collections::HashMap;
use crate::typedef::*;
use crate::collision3d::CollisionBox;

pub type Bones = Vec::<na::Matrix4::<f32>>;

pub type Joints = Vec<Joint>;

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub joints: Joints,
}

pub type SkinId = usize;


#[derive(Default, Debug, Clone)]
pub struct Skins {
    pub mesh_to_skin: HashMap::<String, SkinId>,
    pub skin_to_mesh: HashMap::<SkinId, String>,
    pub skeletons: HashMap::<SkinId, Skeleton>,
    pub index_maps: HashMap::<SkinId, HashMap::<u16, usize>>,
    pub node_index_to_skin: HashMap::<usize, SkinId>, // for use when mapping animations to skin
}


impl Skeleton {

    pub fn calc_t_pose(&mut self) {
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


    pub fn set_all_bones_from_skeleton(&self, bones: &mut [na::Matrix4::<f32>]) {
        for i in 0..self.joints.len() {
            bones[i] = self.joints[i].world_matrix * self.joints[i].inverse_bind_pose;
        }
    }

    /// Generate a new vec with the bones
    pub fn create_bones(&self) -> Bones {
        let mut bones = vec![];
        for _ in 0..self.joints.len() {
            bones.push(na::Matrix4::<f32>::identity());
        }

        self.set_all_bones_from_skeleton(&mut bones);

        bones
    }

    pub fn update_joint_matrices(&mut self, joint_index: usize, rotation: na::UnitQuaternion::<f32>, translation: na::Vector3::<f32>) {
        update_joint_matrices(&mut self.joints, joint_index, rotation, translation);
    }

    pub fn generate_bone_collision_boxes(&self) -> Vec::<collision3d::CollisionBox> {

        let mut res: Vec::<CollisionBox> = vec![];
        for joint in &self.joints {
            // skip root
            if joint.parent_index == 255 {
                continue;
            }

            let to = joint.world_pos();
            let from = self.joints[joint.parent_index].world_pos();

            let bb = CollisionBox::from_end_centers(to, from, 0.1);
            res.push(bb);
        }

        res
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

fn load_skin_nodes(gltf: &gltf::Document, name: &str) -> Vec::<usize> {

    let mut res = vec![];
    for node in gltf.nodes() {
        if node.name() == Some(name) {
            for child in node.children() {
                res.push(child.index());
            }
        }
    }

    res
}

pub fn load_skins(gltf: &gltf::Document) -> Result<Skins, failure::Error> {


    let mut skins : Skins = Default::default();
    // Map mesh names to skin names and nodes to skin id
    for node in gltf.nodes() {
        if let Some(mesh) = node.mesh() {
            if let Some(skin) = node.skin() {
                let mesh_name = mesh.name().unwrap().to_string();

                skins.mesh_to_skin.insert(mesh_name.clone(), skin.index());
                skins.skin_to_mesh.insert(skin.index(), mesh_name);

                let node_indexes = load_skin_nodes(&gltf, skin.name().unwrap());
                for &node_index in &node_indexes {
                    skins.node_index_to_skin.insert(node_index, skin.index());
                }
            }
        }
    }


    for skin in gltf.skins() {

        let mut joints_data = std::collections::HashMap::new();

        // fill the array with joints data
        let mut root_index = None;
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

            if node.name().unwrap() == "root" {
                root_index = Some(index);
            }



            let children: Vec::<usize> = node.children().map(|c| c.index()).collect();


            joints_data.insert(index, (children, node.name().unwrap(), Transformation {
                translation,
                rotation
            }));
        }


        // start from root index and create skeleton from there

        let mut skeleton = Skeleton {
            //name: skin.name().unwrap().to_string(),
            joints: Vec::new(),
        };

        let mut index_map = std::collections::HashMap::<u16,usize>::new();
        if let Some(root) = root_index {
            load_joints(&mut skeleton, &joints_data, root, 255, &mut index_map);
        } else {
            panic!("Could not find any bones with name root. Atm we assume the root is named root");
        }


        // This create a mapping from hip bone (1) to index 0, so first bones in skeleton
        // why is this not added in load bones??
        if !index_map.contains_key(&0) {
            index_map.insert(1, 0);
        }

        //TODO: remove this hardcoded, and find a way to do it more generally

        skeleton.calc_t_pose();

        skins.skeletons.insert(skin.index(), skeleton);
        skins.index_maps.insert(skin.index(), index_map);
    }

    Ok(skins)
}

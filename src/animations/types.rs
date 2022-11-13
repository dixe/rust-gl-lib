use crate::na;

#[derive(Debug, Clone)]
pub struct KeyFrame {
    pub joints: Vec<Transformation>,
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

    pub fn get_local_matrix(&self) -> na::Matrix4::<f32> {
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

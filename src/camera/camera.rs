use crate::na;


/// A general 3d camera
/// Default is right hand coordinate system with z as up, x horizontal and y going into the screen
#[derive(Debug, Clone)]
pub struct Camera {
    pos: na::Vector3::<f32>,
    target: na::Vector3::<f32>,
    look_dir: na::Vector3::<f32>,
    up: na::Vector3::<f32>,
    world_up: na::Vector3::<f32>,
    right: na::Vector3::<f32>,
    width: f32,
    height: f32,
    fov: f32,
    zfar: f32,
    znear: f32,
}

impl Camera {

    pub fn new(width: f32, height: f32) -> Camera {


        let pos = na::Vector3::new(0.0, 0.0, 5.0);
        let target = na::Vector3::new(0.0, 0.0, 0.0);
        let look_dir = na::Vector3::new(1.0, 0.0, 0.0);
        let up = na::Vector3::new(0.0, 0.0, 1.0);
        let right = na::Vector3::new(1.0, 0.0, 0.0);


        Camera {
            pos,
            target,
            look_dir,
            up,
            world_up: na::Vector3::new(0.0, 0.0, 1.0),
            right,
            width,
            height,
            fov: 60.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }


    pub fn pos(&self) -> na::Vector3::<f32> {
        self.pos
    }

    pub fn update_pos(&mut self, new_pos: na::Vector3::<f32>) {
        self.pos = new_pos;
        self.update_camera_vectors();
    }


    pub fn update_target(&mut self, new_target: na::Vector3::<f32>) {
        self.target = new_target;
        self.update_camera_vectors();
    }


    pub fn projection(&self) -> na::Matrix4::<f32> {
        na::Matrix4::new_perspective(self.width / self.height, self.fov.to_radians(), self.znear, self.zfar)
    }

    pub fn orthographic(&self, left: f32, right: f32, bottom:f32, top: f32) -> na::Matrix4::<f32> {
        na::Matrix4::new_orthographic(left, right, bottom, top, self.znear, self.zfar)
    }


    pub fn view(&self) -> na::Matrix4::<f32> {
        let target_vec = self.pos + self.look_dir;

        let target = na::Point3::new(target_vec.x, target_vec.y, target_vec.z);

        let pos = self.pos();
        let point_pos = na::Point3::new(pos.x, pos.y, pos.z);

        na::Matrix::look_at_rh(&point_pos, &target, &self.up)
    }


    fn update_camera_vectors(&mut self) {
        self.look_dir = (self.target - self.pos).normalize();
        self.right = self.look_dir.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.look_dir).normalize();
    }

}

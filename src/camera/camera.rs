use crate::na;


/// A general 3d camera
/// Default is right hand coordinate system with z as up, x horizontal and y going into the screen
#[derive(Debug, Clone)]
pub struct Camera {
    pub(crate)pos: na::Vector3::<f32>,
    pub(crate) front: na::Vector3::<f32>,
    pub(crate) up: na::Vector3::<f32>,
    pub(crate) world_up: na::Vector3::<f32>,
    pub(crate) right: na::Vector3::<f32>,
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) fov: f32,
    pub(crate) zfar: f32,
    pub(crate) znear: f32,
}

impl Camera {

    pub fn new(width: f32, height: f32) -> Camera {


        let pos = na::Vector3::new(0.0, 0.0, 5.0);
        let front = na::Vector3::new(0.0, 0.0, 0.0);
        let up = na::Vector3::new(0.0, 0.0, 1.0);
        let right = na::Vector3::new(1.0, 0.0, 0.0);


        Camera {
            pos,
            front,
            up,
            right,
            world_up: na::Vector3::new(0.0, 0.0, 1.0),
            width,
            height,
            fov: 60.0,
            znear: 0.1,
            zfar: 100.0,
            yaw: (90.0_f32).to_radians(), // point along positive Y axis
            pitch: (-5.0_f32).to_radians(),
        }
    }


    pub fn pos(&self) -> na::Vector3::<f32> {
        self.pos
    }

    pub fn move_to(&mut self, new_pos: na::Vector3::<f32>) {
        self.pos = new_pos;
        self.update_camera_vectors();
    }

    pub fn look_at(&mut self, target: na::Vector3::<f32>) {

        // calc yaw and pitch, only worth because we don't have roll.

        let diff = target - self.pos;
        let height = diff.z; // always height
        let horizontal_len = (diff.x*diff.x + diff.y * diff.y).sqrt();
        self.pitch = (height / horizontal_len).atan();


        let base = na::Vector2::new(1.0, 0.0);
        let new = diff.xy().normalize();
        self.yaw =  new.y.signum() * base.dot(&new).acos();

        self.update_camera_vectors();
    }


    pub fn projection(&self) -> na::Matrix4::<f32> {
        na::Matrix4::new_perspective(self.width / self.height, self.fov.to_radians(), self.znear, self.zfar)
    }

    pub fn orthographic(&self, left: f32, right: f32, bottom:f32, top: f32) -> na::Matrix4::<f32> {
        na::Matrix4::new_orthographic(left, right, bottom, top, self.znear, self.zfar)
    }


    pub fn view(&self) -> na::Matrix4::<f32> {
        let target_vec = self.pos + self.front;

        let target = na::Point3::new(target_vec.x, target_vec.y, target_vec.z);

        let pos = self.pos();
        let point_pos = na::Point3::new(pos.x, pos.y, pos.z);

       na::Matrix::look_at_rh(&point_pos, &target, &self.up)
    }

    pub fn set_zfar(&mut self, zfar: f32) {

        self.zfar = zfar;
    }


    fn update_camera_vectors(&mut self) {
        self.front = na::Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
        ).normalize();

        self.right = self.front.cross(&self.world_up).normalize();

    }

}

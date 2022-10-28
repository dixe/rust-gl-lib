use crate::na;


/// A general 3d camera
/// Default is right hand coordinate system with z as up, x horizontal and y going into the screen
#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: na::Vector3::<f32>,
    pub front: na::Vector3::<f32>,
    pub up: na::Vector3::<f32>,
    pub world_up: na::Vector3::<f32>,
    pub right: na::Vector3::<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub width: f32,
    pub height: f32,
    pub fov: f32,
    pub zfar: f32,
    pub znear: f32,
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

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn right(&self) -> na::Vector3::<f32> {
        self.right
    }

    pub fn up(&self) -> na::Vector3::<f32> {
        self.up
    }

    pub fn front(&self) -> na::Vector3::<f32> {
        self.front
    }

    pub fn move_to(&mut self, new_pos: na::Vector3::<f32>) {
        self.pos = new_pos;
        self.update_camera_vectors();
    }

    pub fn look_at(&mut self, target: na::Vector3::<f32>) {

        // calc yaw and pitch, only works because we don't have roll.
        let diff = target - self.pos;
        let height = diff.z; // always height
        let horizontal_len = (diff.x*diff.x + diff.y * diff.y).sqrt();
        self.pitch = (height / horizontal_len).atan();


        let base = na::Vector3::new(1.0, 0.0, 0.0);
        let new = diff.normalize();
        self.yaw = new.y.signum() * base.dot(&new).acos();

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

    pub fn target(&self) -> na::Vector3::<f32> {
        self.pos + self.front
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

    /// Given a screen x,y return the ray shooting fom camera.pos(origin), throught that pixel, into the world.
    /// Returned ray direction is in world space
    /// 0,0 in screen space is lower left, 1,1 is upper right
    pub fn screen_to_ray(&self, screen_x: f32, screen_y: f32) -> na::Vector3::<f32> {

        // first transform to clip space

        //first from [0;1] then to [-1;]
        let clip_x = ((screen_x / self.width) - 0.5) * 2.0;
        let clip_y = ((screen_y / self.height) - 0.5) * 2.0;

        let mut clip_space = na::Vector3::new(clip_x, clip_y, 1.0).to_homogeneous();
        clip_space.w = 1.0;

        let transform = (self.projection() * self.view() ).try_inverse().unwrap();

        let transformed = transform * clip_space;

        (transformed.xyz() / transformed.w - self.pos).normalize()
    }


    /// Given a world position, return the screen position
    pub fn world_pos_to_screen(&self, world_pos: na::Vector3::<f32>) -> na::Vector2::<f32> {
        let transform = self.projection() * self.view();
        let mut homo_pos = world_pos.to_homogeneous();

        homo_pos.w = 1.0;
        let mut sp = transform * homo_pos;
        sp = sp / sp.w;

        let mut xy = sp.xy();

        xy.x = (xy.x + 1.0) * self.width / 2.0;
        xy.y = self.height - ((xy.y + 1.0) * self.height / 2.0);

        xy
    }

    pub fn to_clip_space(&self, top_left: na::Vector2::<f32>, w: f32, h: f32) -> ClipSpace {

        ClipSpace {
            left: (-0.5 + top_left.x / self.width) * 2.0,
            right: (-0.5 + (top_left.x + w) / self.width) * 2.0,
            top:  (-0.5 + (top_left.y) / self.height) * -2.0,
            bottom:  (-0.5 + (top_left.y + h) / self.height) * -2.0
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct ClipSpace {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

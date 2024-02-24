use crate::camera::*;
use crate::typedef::*;
use crate::na;

#[derive(Clone, Debug)]
pub struct Controller {
    target: na::Vector3::<f32>,
    pub desired_distance: f32,
    pub desired_pitch: f32,
    pub yaw_change: f32
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            target: V3::new(0.0, 0.0, 0.0),
            desired_distance: 7.0,
            desired_pitch: 0.5,
            yaw_change: 0.0,
        }
    }
}


impl Controller {

    pub fn update_camera_target(&mut self, pos: V3) {
        self.target = pos;
    }


    pub fn update_dist(&mut self, dist_delta: f32) {
        self.desired_distance = f32::max(1.0, f32::min(15.0, self.desired_distance + dist_delta));
    }

    pub fn update_camera(&mut self, camera: &mut Camera, _dt: f32){


        // camera is always target

        camera.pos =
        let diff_xy = camera.pos.xy() - self.target.xy();
        let yaw = (diff_xy.y).atan2(diff_xy.x) - self.yaw_change;

        self.desired_pitch = f32::min(f32::max(0.0, self.desired_pitch), std::f32::consts::PI/2.0);

        // find target z
        let sin_p = self.desired_pitch.sin();
        let mut target_z = camera.pos.z;
        if sin_p != 0.0 {
            target_z = self.desired_distance * sin_p;
        }

        let d = V3::new(yaw.cos(), yaw.sin(), 0.0);

        // not acctually correct in the sense that we get desired distance, since we get that in xy plane, and then we add z, which make the distance longer
        // but it is stable, so what does it matter
        camera.pos = d * self.desired_distance + self.target;
        camera.pos.z = target_z;

        camera.look_at(self.target);


    }

}

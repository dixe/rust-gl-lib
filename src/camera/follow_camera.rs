use crate::camera::*;
use crate::typedef::*;
use crate::na;

#[derive(Clone, Debug)]
pub struct Controller {
    target: na::Vector3::<f32>,
    desired_distance: f32,
    pub desired_pitch: f32
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            target: V3::new(0.0, 0.0, 0.0),
            desired_distance: 10.0,
            desired_pitch: 0.5
        }
    }
}


impl Controller {


    pub fn update_camera_target(&mut self, pos: V3) {
        self.target = pos;
    }


    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32){

        self.desired_pitch = f32::min(f32::max(0.0, self.desired_pitch), std::f32::consts::PI/2.0);

        // update camera z fist;
        let sin_p = self.desired_pitch.sin();
        let mut target_z = camera.pos.z;
        if sin_p != 0.0 {
            target_z = self.desired_distance * sin_p;
        }

        camera.pos.z = target_z;

        //println!("Desired, sin, target_z {:?}", (self.desired_pitch, sin_p, target_z));

        let diff = camera.pos - self.target;
        let dist = (camera.pos - self.target).magnitude();

        // if we are too close keep pitch, so move along xy plane
        let change = dist - self.desired_distance;
        if change.abs() > 0.5 {
            let dir = diff.xy().normalize();
            // TODO: make this smooth using dt, since now we just skip, jump like we stutter
            let target_pos = self.target.xy() + dir * self.desired_distance;

            // when far away zoom in faster, this ensure the distance to camera is decreasing,
            // even when char is fast. Ohterwise char can keep getting furhter and further away
            let zoom_speed = dist * 1.7;
            let delta = -change.signum() * dir * dt * zoom_speed;

            camera.pos.x += delta.x;
            camera.pos.y += delta.y;

        }

        camera.look_at(self.target);

        //println!("{:?}", (camera.pitch, dist))

/*
        //println!("Font: {:.2?}", camera.front);

        // do the actual update
        let mut pos = camera.pos();
        let dir = self.movement;

        //println!("Dir: {:.2?}", dir);
        let dt_speed = dt * self.speed;

        pos += camera.front * dir.x * dt_speed;
        pos += camera.right * dir.y * dt_speed;
        pos += camera.up * dir.z * dt_speed;



        camera.yaw -= self.mouse_movement.xrel * self.sens * dt * self.inverse_x;

        camera.pitch = f32::max((-70.0_f32).to_radians(), f32::min((70.0_f32).to_radians(), camera.pitch + self.mouse_movement.yrel* self.sens * self.inverse_y *  dt));

        //camera.pitch = camera.pitch + self.mouse_movement.yrel* self.sens * self.inverse_y *  dt;

        self.mouse_movement.xrel = 0.0;
        self.mouse_movement.yrel = 0.0;

         */



    }

}

use crate::camera::*;
use crate::movement::Inputs;


pub fn update_camera(camera: &mut Camera, dt: f32, input: &Inputs){

    //println!("Font: {:.2?}", camera.front);

    // do the actual update
    let mut pos = camera.pos();
    let dir = input.movement;

    //println!("Dir: {:.2?}", dir);
    let dt_speed = dt * input.speed;

    pos += camera.front * dir.x * dt_speed;
    pos += camera.right * dir.y * dt_speed;
    pos += camera.up * dir.z * dt_speed;



    camera.yaw -= input.mouse_movement.xrel * input.sens * dt * input.inverse_x;

    camera.pitch = f32::max((-70.0_f32).to_radians(), f32::min((70.0_f32).to_radians(), camera.pitch + input.mouse_movement.yrel* input.sens * input.inverse_y *  dt));

    camera.pitch = camera.pitch + input.mouse_movement.yrel* input.sens * input.inverse_y *  dt;

    camera.move_to(pos);

}

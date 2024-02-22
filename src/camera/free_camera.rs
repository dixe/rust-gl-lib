use crate::camera::*;
use crate::movement::Inputs;
use crate::na;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::{self, * };
use std::collections::HashMap;

/// Use this for the most part, fx used in scene
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

///Free  camera controller. Can fly in all directions. forward is where camera is looking
#[derive(Debug, Clone)]
pub struct Controller {
    movement: na::Vector3::<f32>,
    mapping: KeyMapping,
    pub speed: f32,
    mouse_movement: MouseMovement,
    pub sens: f32,
    inverse_y: f32, // should be -1 or 1. 1 for normal, -1 for inverse
    inverse_x: f32,
}

impl Controller {

    pub fn update_events(&mut self, event: &Event) {
        // update state based on event


        match event {
            Event::KeyDown{keycode: Some(kc), .. } => {
                if let Some(dir) = self.mapping.move_mapping.get(&kc) {
                    // Clamp x,y,z in dependently to discrete [-1, 0, 1]
                    self.movement.x = f32::max(-1.0, f32::min(1.0, self.movement.x + dir.x));
                    self.movement.y = f32::max(-1.0, f32::min(1.0, self.movement.y + dir.y));
                    self.movement.z = f32::max(-1.0, f32::min(1.0, self.movement.z + dir.z));
                }
            },
            Event::KeyUp{keycode: Some(kc), .. } => {
                 if let Some(dir) = self.mapping.move_mapping.get(&kc) {
                    self.movement -= dir;
                }
            },
            Event::MouseMotion{mousestate, xrel, yrel, .. } => {
                if mousestate.right() {
                    //println!("{:?}", (xrel, yrel));
                    self.mouse_movement.xrel = *xrel as f32;
                    self.mouse_movement.yrel = *yrel as f32;
                }
            },
            _ => {}
        };

    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32){

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


        camera.move_to(pos);

    }

}



impl Default for Controller {
    fn default() -> Self {
        Controller {
            movement: na::Vector3::new(0.0, 0.0, 0.0),
            mapping: Default::default(),
            speed: 10.0,
            mouse_movement: Default::default(),
            sens: 0.15,
            inverse_y : 1.0,
            inverse_x : -1.0,
        }
    }
}


#[derive(Debug, Clone, Default)]
struct MouseMovement {
    xrel: f32,
    yrel: f32
}


#[derive(Debug,Clone)]
pub struct KeyMapping {
    pub(crate) move_mapping: HashMap::<Keycode, na::Vector3::<f32>>
}

impl Default for KeyMapping {
    fn default() -> Self {

        let mut map = HashMap::new();
        // Forward
        map.insert(W, na::Vector3::new(1.0, 0.0, 0.0));
        // Backward
        map.insert(S, na::Vector3::new(-1.0, 0.0, 0.0));
        // Left
        map.insert(A, na::Vector3::new(0.0, -1.0, 0.0));
        //Right
        map.insert(D, na::Vector3::new(0.0, 1.0, 0.0));
        // Up
        map.insert(Space, na::Vector3::new(0.0, 0.0, 1.0));
        // Down
        map.insert(LShift, na::Vector3::new(0.0, 0.0, -1.0));


        Self {
            move_mapping: map
        }
    }
}

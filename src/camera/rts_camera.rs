use crate::camera::*;
use crate::na;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::{self, * };
use std::collections::HashMap;


///Rts like camera, up/down moves Y rihgt left moves x. Looks down at scene
#[derive(Debug, Clone)]
pub struct Controller {
    movement: na::Vector3::<f32>,
    mapping: KeyMapping,
    pub speed: f32,
    mouse_movement: MouseMovement,
    pub sens: f32,
    inverse_y: f32, // should be -1 or 1. 1 for normal, -1 for inverse
}

impl Controller {

    pub fn update_events(&mut self, event: Event) {
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
                    self.mouse_movement.xrel = xrel as f32;
                    self.mouse_movement.yrel = self.inverse * yrel as f32;
                }
            },
            _ => {}
        };

    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32){

        // do the actual update
        let dt_speed = dt * self.speed;
        let new_pos = camera.pos() + self.movement * dt_speed;
        let new_target = camera.target() + self.movement * dt_speed;
        camera.move_to(new_pos);
        camera.look_at(new_target);

    }

}



impl Default for Controller {
    fn default() -> Self {
        Controller {
            movement: na::Vector3::new(0.0, 0.0, 0.0),
            mapping: Default::default(),
            speed: 2.0,
            mouse_movement: Default::default(),
            sens: 0.2,
            inverse_y : -1.0,
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
        map.insert(W, na::Vector3::new(0.0, 1.0, 0.0));
        // Backward
        map.insert(S, na::Vector3::new(0.0, -1.0, 0.0));
        // Left
        map.insert(A, na::Vector3::new(-1.0, 0.0, 0.0));
        //Right
        map.insert(D, na::Vector3::new(1.0, 0.0, 0.0));
         // Up
        map.insert(Space, na::Vector3::new(0.0, 0.0, 1.0));
        // Down
        map.insert(LShift, na::Vector3::new(0.0, 0.0, -1.0));
        Self {
            move_mapping: map
        }
    }
}

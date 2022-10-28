use crate::camera::*;
use crate::na;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::{self, * };
use std::collections::HashMap;


///Rts like camera, up/down moves Y rihgt left moves x. Looks down at scene
#[derive(Debug, Clone)]
pub struct Controller {
    //movement: na::Vector3::<f32>,
    mapping: KeyMapping,
    pub speed: f32,
    pub sens: f32,
    inverse_y: f32,// should be -1 or 1. 1 for normal, -1 for inverse
    pub screen_w: i32,
    pub screen_h: i32,
    // movement is left, right up down
    keyboard_movement: [bool; 4],
    mouse_movement: [bool; 4],
    zoom: f32,
    zoom_speed: f32

}


#[derive(Default, Debug, Clone, Copy)]
struct Movement {
    // movement is left, right up down
    movement: [bool; 4],
}

impl Controller {

    pub fn update_events(&mut self, event: Event) {

        // update state based on event
        match event {
            Event::KeyDown{keycode: Some(kc), .. } => {
                if let Some(dir) = self.mapping.move_mapping.get(&kc) {

                    // Set which movement keys are pressed
                    self.keyboard_movement[0] = dir[0] || self.keyboard_movement[0];
                    self.keyboard_movement[1] = dir[1] || self.keyboard_movement[1];

                    self.keyboard_movement[2] = dir[2] || self.keyboard_movement[2];
                    self.keyboard_movement[3] = dir[3] || self.keyboard_movement[3];

                }
            },
            Event::KeyUp{keycode: Some(kc), .. } => {
                if let Some(dir) = self.mapping.move_mapping.get(&kc) {
                    self.keyboard_movement[0] = !dir[0] && self.keyboard_movement[0];
                    self.keyboard_movement[1] = !dir[1] && self.keyboard_movement[1];
                    self.keyboard_movement[2] = !dir[2] && self.keyboard_movement[2];
                    self.keyboard_movement[3] = !dir[3] && self.keyboard_movement[3];

                }
            },
            Event::MouseMotion{mousestate, x, y, .. } => {
                // set mouse move direction
                self.mouse_movement[0] = x == 0;
                self.mouse_movement[1] = x + 1 == self.screen_w;
                self.mouse_movement[2] = y == 0;
                self.mouse_movement[3] = y + 1 == self.screen_h;
            },

            Event::MouseWheel{y, .. } => {
                let y_signum = (y as f32).signum();
                if self.zoom.signum() != y_signum {
                    self.zoom = y as f32;
                } else {
                    self.zoom = y_signum * self.zoom_speed;
                }

            },
            _ => {}
        };

    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32){

        // do the actual update
        let dt_speed = dt * self.speed;

        // cast bool to float and multiple with the direction. x is left * -1.0 + right * 1.0
        let x = -1.0 * (self.keyboard_movement[0] || self.mouse_movement[0]) as i32 as f32 +
            1.0 * (self.keyboard_movement[1] || self.mouse_movement[1]) as i32 as f32;

        let y = 1.0 * (self.keyboard_movement[2] || self.mouse_movement[2]) as i32 as f32 +
            -1.0 * (self.keyboard_movement[3] || self.mouse_movement[3]) as i32 as f32;


        let dir = na::Vector3::new(x,y,0.0);

        let mut new_pos = camera.pos();
        let mut forward = camera.right;

        new_pos += camera.right * dir.x * dt_speed;

        new_pos += na::Vector3::new(-camera.right.y, camera.right.x, camera.right.z) * dir.y * dt_speed;

        new_pos += camera.front * dt * self.zoom * self.zoom_speed;


        camera.move_to(new_pos);


        if self.zoom.abs() < 2.0 {
            self.zoom = 0.0;
        }
    }

}



impl Default for Controller {
    fn default() -> Self {
        Controller {
            //movement: na::Vector3::new(0.0, 0.0, 0.0),
            mapping: Default::default(),
            speed: 2.0,
            sens: 0.2,
            inverse_y : -1.0,
            screen_w: 1200,
            screen_h: 700,
            keyboard_movement: Default::default(),
            mouse_movement: Default::default(),
            zoom: 0.0,
            zoom_speed: 3.0
        }
    }
}



#[derive(Debug,Clone)]
pub struct KeyMapping {
    pub(crate) move_mapping: HashMap::<Keycode, [bool;4]>
}

impl Default for KeyMapping {
    fn default() -> Self {

        let mut map = HashMap::new();
        map.insert(Left, [true, false, false, false]);
        map.insert(Right, [false, true, false, false]);
        map.insert(Up, [false, false, true, false]);
        map.insert(Down, [false, false, false, true]);

        Self {
            move_mapping: map
        }
    }
}

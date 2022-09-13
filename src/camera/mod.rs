mod camera;
pub use self::camera::*;
use crate::na;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::{self, * };
use std::collections::HashMap;



#[derive(Debug, Clone)]
pub struct Controller {
    movement: na::Vector3::<f32>,
    mapping: KeyMapping,
    pub speed: f32
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
            _ => {}
        };

    }

    pub fn update_camera(&self, camera: &mut Camera, dt: f32){

        // do the actual update
        let mut pos = camera.pos();
        let dir = self.movement;


        let dt_speed = dt * self.speed;

        pos += camera.front * dir.x * dt_speed;
        pos += camera.right * dir.y * dt_speed;

        camera.move_to(pos);

    }

}


impl Default for Controller {
    fn default() -> Self {
        Controller {
            movement: na::Vector3::new(0.0, 0.0, 0.0),
            mapping: Default::default(),
            speed: 1.0
        }
    }
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


        Self {
            move_mapping: map
        }
    }
}


use crate::na;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode::{self, * };
use std::collections::HashMap;


///Free  camera controller. Can fly in all directions. forward is where camera is looking
#[derive(Debug, Clone)]
pub struct Inputs {
    pub movement: na::Vector3::<f32>,
    pub mapping: KeyMapping,
    pub speed: f32,
    pub mouse_movement: MouseInputs,
    pub sens: f32,
    pub inverse_y: f32, // should be -1 or 1. 1 for normal, -1 for inverse
    pub inverse_x: f32,
    pub mouse_wheel: f32,
    pub middle_mouse: bool,
    pub left_mouse: bool,
    pub animation_expired: bool
}



impl Inputs {

    pub fn frame_start(&mut self) {
        self.mouse_movement.xrel = 0.0;
        self.mouse_movement.yrel = 0.0;
        self.mouse_wheel = 0.0;
        self.middle_mouse = false;
        self.left_mouse = false;
    }

    pub fn update_events(&mut self, event: &Event) {

        match event {
            Event::MouseWheel {y, .. } => {
                self.mouse_wheel -= *y as f32;
            },
            Event::MouseButtonDown {mouse_btn, .. } => {
                if *mouse_btn == sdl2::mouse::MouseButton::Middle {
                    self.middle_mouse = true;
                } else if *mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.left_mouse = true;
                }
            },
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
            Event::MouseMotion{ xrel, yrel, .. } => {
                self.mouse_movement.xrel = *xrel as f32;
                self.mouse_movement.yrel = *yrel as f32;
            },
            _ => {}
        };

    }
}


#[derive(Debug, Clone, Default)]
pub struct MouseInputs {
    pub xrel: f32,
    pub yrel: f32
}


#[derive(Debug,Clone)]
pub struct KeyMapping {
    pub move_mapping: HashMap::<Keycode, na::Vector3::<f32>>
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



impl Default for Inputs {
    fn default() -> Self {
        Self {
            movement: na::Vector3::new(0.0, 0.0, 0.0),
            mapping: Default::default(),
            speed: 10.0,
            mouse_movement: Default::default(),
            sens: 0.25,
            inverse_y : -1.0,
            inverse_x : 1.0,
            mouse_wheel: 0.0,
            middle_mouse: false,
            left_mouse: false,
            animation_expired: false
        }
    }
}

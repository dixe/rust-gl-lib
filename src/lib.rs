//! A libaray that aim to make use of opengl eaiser to start with, or add
//! to project where graphics is not the main focus.
//! While still allowing all the low level access needed for performance
//! and customizability.
//!
//! # Examples
//!
//! See [examples](https://github.com/dixe/rust-gl-lib/tree/master/examples)
//!

use std::default::Default;

pub use nalgebra as na;

pub mod gl;
pub mod shader;
pub mod buffer;
pub mod objects;
pub mod camera;
pub mod text_rendering;
pub mod texture;



/// Defines point in ScreenBox x,y in \[0; 1.0\]
/// Top left corner is x=0, y=0
#[derive(Debug, Copy, Clone)]
pub struct BoxCoords {
    pub x: f32,
    pub y: f32
}

/// Defines point in ScreenBox x,y in \[0.0; 1.0\]
/// Top left corner is x=0, y=0
#[derive(Debug, Copy, Clone)]
pub struct ScreenCoords {
    pub x: f32,
    pub y: f32
}

/// Defines box on the screen starting at x,y width width and height
/// Defeault to the whole screen. Staring at (0,0) with height going downwards
/// With height of 1 being whole screen and widht of 1 being whole screen
#[derive(Debug, Copy, Clone)]
pub struct ScreenBox {
    coords: ScreenCoords,
    width: f32,
    height: f32,
}



impl ScreenBox {

    pub fn new(pixel_x: f32, pixel_y: f32, pixel_w: f32, pixel_h: f32, screen_w: f32, screen_h: f32) -> Self {
        ScreenBox {
            coords: ScreenCoords {
                x: pixel_x / screen_w,
                y: pixel_y / screen_h,
            },

            width:  pixel_w / screen_w,
            height: pixel_h / screen_h,
        }
    }

    pub fn left(&self) -> f32 {
        self.coords.x
    }

    pub fn right(&self) -> f32 {
        self.coords.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.coords.y
    }

    pub fn bottom(&self) -> f32 {
        self.coords.y + self.height
    }

    pub fn width(&mut self, w: f32) {
        self.width = w;
    }

    pub fn create_child(&self, coords: BoxCoords, w:f32, h: f32) -> Self {
        ScreenBox {
            coords: ScreenCoords {
                x: self.coords.x + self.width * coords.x,
                y: self.coords.y + self.height * coords.y,
            },

            height: h,
            width: w,
        }
    }
}

impl Default for ScreenBox {

    fn default() -> Self {
        ScreenBox {
            coords: ScreenCoords {
                x: 0.0,
                y: 0.0,
            },

            height: 1.0,
            width: 1.0,
        }
    }
}

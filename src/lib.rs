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
pub use failure as failure;
pub use sdl2;

pub mod gl;
pub mod shader;
pub mod buffer;
pub mod objects;
pub mod camera;
pub mod text_rendering;
pub mod texture;
pub mod color;
pub mod deltatime;
pub mod helpers;
pub mod typedef;
pub mod scene_3d;
pub mod math;

pub mod widget_gui;

pub mod image;

pub mod sdl_gui;

pub mod controller;

pub mod animations;

pub mod imode_gui;

pub mod collision2d;

pub mod particle_system;

pub mod general_animation;

pub mod audio;

/// Defines point in ScreenBox x,y in \[0.0; 1.0\]
/// Top left corner is x=0, y=0
#[derive(Debug, Copy, Clone)]
pub struct ScreenCoords {
    pub x: f32,
    pub y: f32
}

/// Defines box on the screen starting at x,y width width and height
/// Defeault to the whole screen. Staring at (0,0) with height going downwards
/// With height of 1 being whole screen and width of 1 being whole screen
#[derive(Debug, Copy, Clone)]
pub struct ScreenBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub screen_w: f32,
    pub screen_h: f32
}



impl ScreenBox {

    pub fn new(pixel_x: f32, pixel_y: f32, pixel_w: f32, pixel_h: f32, screen_w: f32, screen_h: f32) -> Self {
        ScreenBox {
            x: pixel_x,
            y: pixel_y,
            width:  pixel_w,
            height: pixel_h,
            screen_w,
            screen_h
        }
    }


    pub fn full_screen(screen_w: f32, screen_h: f32) -> Self {
        ScreenBox {
            x: 0.0,
            y: 0.0,
            width: screen_w,
            height: screen_h,
            screen_w,
            screen_h
        }
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

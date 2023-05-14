#![allow(warnings)]
use crate::na::{self, Translation3};
use crate::text_rendering::{text_renderer::TextRenderer};
use crate::{gl::{self, viewport}, ScreenBox, ScreenCoords};
use crate::shader::{ TransformationShader, rounded_rect_shader::{self as rrs, RoundedRectShader}, circle_shader::{self as cs, CircleShader}};
use crate::objects::square;
use sdl2::event;
use crate::color::Color;


pub mod ui;
pub use ui::*;

pub mod drawer2d;
use drawer2d::*;

pub mod widgets;

pub mod numeric;

pub mod style;

#[derive(Debug, Copy, Clone, Default)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}


pub type Id = u64;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

#[derive(Debug, Copy, Clone)]
pub enum WidgetStatus {
    Inactive,
    Hot,
    Active
}
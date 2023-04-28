#![allow(warnings)]
use crate::na::{self, Translation3};
use crate::widget_gui::*;
use crate::widget_gui::layout::Size;
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

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32
}


pub type Id = u64;

pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

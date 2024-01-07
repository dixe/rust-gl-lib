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

pub mod style;

pub type Pos = na::Vector2::<i32>;

pub trait ToPos {
    fn to_pos(&self) -> Pos;

    fn from_pos(&mut self, p: Pos);
}

impl ToPos for Pos {

    fn to_pos(&self) -> Pos {
        *self
    }

    fn from_pos(&mut self, p: Pos) {
        *self = p;
    }
}



impl ToPos for na::Vector2::<f32> {

    fn to_pos(&self) -> Pos {
        Pos::new(self.x as i32, self.y as i32)
    }

    fn from_pos(&mut self, p: Pos) {
        self.x = p.x as f32;
        self.y = p.y as f32;
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Hash, Eq)]
pub struct Id {
    widget_id: u64,
    window_id: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WidgetStatus {
    Inactive,
    Hot,
    Active
}

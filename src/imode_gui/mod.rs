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


pub mod drawer2d;
use drawer2d::*;
pub mod widgets;

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32
}


pub type Id = u64;

pub struct Ui<'a> {
    pub drawer2D: Drawer2D<'a>,
    mouse_pos: Pos,
    next_id: u64,
    mouse_down: bool,
    mouse_up: bool,
    /// Reset after each frame. Widget should use hot instead of just checking in the mouse is inside them
    /// since hot only gets set when there is not active widget or we out self are the active elemnt
    /// use to check fx release of mouse happens inside button.
    hot: Option<Id>,
    /// Persisted between frames
    active: Option<Id>,
    draw_offset: Pos

}

pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}
impl<'a> Ui<'a> {

    pub fn new(drawer2D: Drawer2D<'a>) -> Self {

        Self {
            drawer2D,
            mouse_pos: Pos{x:0, y: 0},
            hot: None,
            active: None,
            next_id: 0,
            mouse_down: false,
            mouse_up: false,
            draw_offset: Pos {x: 0, y: 0}
        }

    }

    pub fn set_hot(&mut self, id: Id) {
        if self.active == None || self.active == Some(id) {
            self.hot = Some(id);
        }
    }

    pub fn is_hot(&self, id: Id) -> bool {
        self.hot == Some(id)
    }

    pub fn is_active(&self, id: Id) -> bool {
        self.active == Some(id)
    }
    pub fn set_active(&mut self, id: Id) {
        // TODO: Should this also check if we are already hot?
        self.active = Some(id)
    }

    pub fn set_not_active(&mut self) {
        self.active = None
    }

    pub fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn mouse_in_rect(&self, rect: &Rect) -> bool {
        self.mouse_pos.x >= rect.x
            && self.mouse_pos.x <= rect.x + rect.w
            && self.mouse_pos.y >= rect.y
            && self.mouse_pos.y <= rect.y + rect.h
    }

    pub fn layout_rect(&mut self, mut rect: Rect) -> Rect {
        // TODO: Figure out good way to handle spacing/margin
        // TODO: Handle vertical and horizontal and overflow
        rect.x += self.draw_offset.x;


        self.draw_offset.x = rect.x + rect.w + 5;
        self.draw_offset.y = rect.y + rect.w + 5;

        // TODO:
        rect

    }


    // TODO: Either return unused events only. Or return all events along with bool to indicate if the event is used/consumed by gui
    pub fn consume_events(&mut self, event_pump: &mut sdl2::EventPump) {

        self.mouse_down = false;
        self.mouse_up = false;
        self.next_id = 0;
        self.hot = None;
        self.draw_offset = Pos{ x: 0, y: 0 };


        use event::Event::*;

        for event in event_pump.poll_iter() {
            match event {
                MouseButtonDown {x, y, ..} => {
                    self.mouse_down = true;
                },
                MouseButtonUp {x, y, ..} => {
                    self.mouse_up = true;
                },
                MouseMotion {x,y, .. } => {
                    self.mouse_pos = Pos{x,y};
                },
                Window {win_event: event::WindowEvent::Resized(x,y), ..} => {
                    self.drawer2D.update_viewport(x, y);
                },
                other => {
                    // pass along to program
                }
            }
        }
    }

}


pub trait Numeric : Clone + Copy + PartialEq + PartialOrd + 'static {

    fn to_f64(self) -> f64 ;

    fn from_f64(num: f64) -> Self;
}

macro_rules! impl_numeric {
    ($t: ident) => {
        impl Numeric for $t {
            fn to_f64(self) -> f64 {
                self as f64
            }

            fn from_f64(num: f64) -> Self {
                num as Self
            }
        }
    }
}


impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);


impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);

impl_numeric!(f32);
impl_numeric!(f64);

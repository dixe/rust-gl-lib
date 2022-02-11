use crate::gl::viewport;

mod row;
pub use self::row::*;

mod column;
pub use self::column::*;

mod attributes;
pub use self::attributes::*;

mod button;
pub use self::button::*;

mod element;
pub use self::element::*;

mod node;
pub use self::node::*;




#[derive(Debug,Clone, Copy)]
pub struct RealizedSize {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}


impl From<&viewport::Viewport> for RealizedSize {

    fn from(viewport: &viewport::Viewport) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: viewport.w as f32,
            height: viewport.h as f32
        }
    }
}


pub mod engine;


impl From<engine::Size> for RealizedSize {
    fn from(size: engine::Size) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.w,
            height: size.h
        }

    }
}



#[derive(Debug, Clone, Copy)]
pub enum OnFill {
    Expand,
    Shrink
}

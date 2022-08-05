use crate::text_rendering::text_renderer::TextRenderer;
use crate::objects::square;
use crate::ScreenCoords;
use crate::na::{self, Translation3};
use crate::sdl_gui::components::button::Button;
use crate::gl;
use std::fmt::Debug;
use crate::sdl_gui::layout;

#[derive(Debug,Clone,Copy)]
pub enum ComponentEvent {
    Clicked(ClickType, na::Vector2::<i32>),
    Hover,
    HoverEnd,
    KeyboardInput(KeyInfo)
}

#[derive(Debug,Clone,Copy)]
pub struct KeyInfo {
    pub keycode: sdl2::keyboard::Keycode,
    pub keymod: sdl2::keyboard::Mod,
}


#[derive(Debug,Clone,Copy, PartialEq)]
pub enum ClickType {
    Left,
    Right,
}

pub type Level = u32;
#[derive(Debug, Default,  Clone, Copy)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ComponentBase {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub level: Level,
    pub hover: bool,
    pub disabled: bool
}


impl ComponentBase {
    pub fn new(x: f32, y: f32, width: f32, height: f32, disabled: bool) -> Self {
        Self {
            width,
            height,
            x,
            y,
            hover: false,
            level: 0,
            disabled
        }
    }

    pub fn set_width(&mut self, w_pixels: f32) {
        self.width = w_pixels;
    }


    pub fn set_height(&mut self, h_pixels: f32) {
        self.height = h_pixels;
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn color_scale(&self) -> f32 {
        let mut cs = if self.hover { 0.6 } else { 1.0 };
        if self.disabled {
            cs = 0.4;
        }
        cs
    }

    pub fn unit_square_transform_matrix(&self, screen_w: f32, screen_h: f32) -> na::Matrix4::<f32> {

        let sc_top_left = Self::window_to_screen_coords(self.x, self.y, screen_w, screen_h);

        let x_scale = self.width / screen_w  * 2.0;
        let y_scale = self.height / screen_h * 2.0;

        let mut model = na::Matrix4::<f32>::identity();

        // Scale to size
        model[0] = x_scale;
        model[5] = y_scale;

        // move to position

        let x_move = sc_top_left.x + x_scale * 0.5;
        let y_move = sc_top_left.y - y_scale * 0.5;

        let trans = Translation3::new(x_move, y_move, 0.0);

        model = trans.to_homogeneous() * model;

        model
    }

    pub fn window_to_screen_coords(x: f32, y: f32, w: f32, h: f32) -> ScreenCoords {
        ScreenCoords {x : x *2.0/ w  - 1.0, y: -y *2.0 / h + 1.0 }
    }


}


impl From<layout::RealizedSize> for ComponentBase {
    fn from(rs: layout::RealizedSize) -> Self {
        Self::new(rs.x, rs.y, rs.width, rs.height, rs.disabled)
    }
}


#[derive(Debug,Clone,Copy)]
pub enum OnTop {
    No,
    OnTop(Level)
}


pub type Component<Message> = Box<dyn ComponentTrait<Message>>;

pub trait ComponentTrait<Message>: Debug where Message: Clone {

    fn base(&self) -> &ComponentBase;

    fn base_mut(&mut self) -> &mut ComponentBase;

    fn set_base(&mut self, base: ComponentBase);

    fn on_top(&self, x: f32, y: f32) -> OnTop {

        if x >= self.base().x && x <= self.base().x + self.base().width && y >= self.base().y && y <= self.base().y + self.base().height {
            return OnTop::OnTop(self.base().level)
        }

        OnTop::No
    }

    fn focus_on_click(&self) -> bool {
        false
    }

    fn disabled(&self) -> bool {
        self.base().disabled
    }

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32);

    fn update_content(&mut self, content: String);

    fn on_event(&self, event: ComponentEvent) -> Option<Message>;

}

#[derive(Debug,Clone)]
pub enum ComponentType<Message> {
    Btn(Button<Message>)
}

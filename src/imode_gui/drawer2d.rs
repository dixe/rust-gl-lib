use super::*;

use crate::na::{self, Translation3};
use crate::widget_gui::*;

use crate::widget_gui::layout::Size;
use crate::text_rendering::{text_renderer::TextRenderer};
use crate::{gl::{self, viewport}, ScreenBox, ScreenCoords};
use crate::text_rendering::text_renderer::{TextAlignment, TextAlignmentX,TextAlignmentY};
use crate::shader::{ TransformationShader, rounded_rect_shader::{self as rrs, RoundedRectShader}, circle_shader::{self as cs, CircleShader}};
use crate::objects::square;
use crate::color::Color;




pub struct Drawer2D<'a> {
    pub gl: &'a gl::Gl,
    pub tr: &'a mut TextRenderer,
    pub viewport: &'a viewport::Viewport,
    pub render_square: &'a square::Square,
    pub rounded_rect_shader: &'a RoundedRectShader,
    pub circle_shader: &'a CircleShader,
}

impl<'a> Drawer2D<'a> {

    pub fn rounded_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        self.rounded_rect_color(x, y, w, h, Color::Rgb(100, 100, 100));
    }


    pub fn rounded_rect_color(&self, x: i32, y: i32, w: i32, h: i32, color: Color) {

        self.rounded_rect_shader.shader.set_used();

        let geom = Geometry {
            pos: Position { x, y },
            size: Size {
                pixel_w: w,
                pixel_h: h

            }
        };

        let transform = unit_square_transform_matrix(&geom, self.viewport);

        self.rounded_rect_shader.set_transform(transform);


        self.rounded_rect_shader.set_uniforms(rrs::Uniforms { color,
                                                             pixel_height: geom.size.pixel_h as f32,
                                                             pixel_width: geom.size.pixel_w as f32,
                                                             radius: 0.0
        });

        self.render_square.render(self.gl);
    }


    pub fn render_text(&mut self, text: &str, x: i32, y: i32) {

        let rect = Rect {
            x,
            y,
            w: 1000,
            h: 1000
        };

        let sb = transform_to_screen_space(&rect, &self.viewport);


        let alignment = TextAlignment {
            x: TextAlignmentX::Left,
            y: TextAlignmentY::Top
        };

        self.tr.render_text(self.gl, text, alignment, sb, 1.0);
    }
}



pub fn transform_to_screen_space(rect: &Rect, viewport: &viewport::Viewport) -> ScreenBox {
    ScreenBox::new(rect.x as f32,
                   rect.y as f32,
                   rect.w as f32,
                   rect.h as f32,
                   viewport.w as f32,
                   viewport.h as f32)
}



pub fn unit_square_transform_matrix(geom: &Geometry, viewport: &viewport::Viewport) -> na::Matrix4::<f32> {

    let sc_top_left = window_to_screen_coords(geom.pos.x as f32, geom.pos.y as f32, viewport.w as f32, viewport.h as f32);

    let x_scale = geom.size.pixel_w as f32 / viewport.w as f32 * 2.0;
    let y_scale = geom.size.pixel_h as f32  / viewport.h as f32 * 2.0;

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

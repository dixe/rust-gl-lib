use super::*;
use crate::na::{self, Point3, Vector3, Translation3, geometry::Rotation, Vector2, Orthographic3};
use crate::widget_gui::*;
use crate::widget_gui::layout::Size;
use crate::text_rendering::text_renderer::{TextRenderer, TextRenderBox};
use crate::{gl::{self, viewport}, ScreenBox, ScreenCoords};
use crate::text_rendering::text_renderer::{TextAlignment, TextAlignmentX,TextAlignmentY};
use crate::shader::{ Shader, TransformationShader, rounded_rect_shader::{self as rrs, RoundedRectShader}, circle_shader::{self as cs, CircleShader}};
use crate::objects::{square, color_square};
use crate::color::Color;
use crate::helpers::SetupError;
use crate::text_rendering::font::Font;

pub struct Drawer2D {
    pub gl: gl::Gl,
    pub tr: TextRenderer,
    pub viewport: viewport::Viewport,
    pub square: square::Square,
    pub color_square: color_square::ColorSquare,
    pub rounded_rect_shader: RoundedRectShader,
    pub color_square_shader: Box::<Shader>,
    pub color_square_h_line_shader: Box::<Shader>,
    pub circle_shader: CircleShader,
}

impl Drawer2D {

    pub fn new(gl: &gl::Gl, viewport: viewport::Viewport) -> Result<Self, SetupError> {

        let inner_font = Default::default();
        let font = Font::Msdf(inner_font);
        let text_renderer = TextRenderer::new(gl, font);
        text_renderer.setup_blend(gl);
        let rrs = RoundedRectShader::new(gl)?;
        let cs = CircleShader::new(gl)?;
        let color_square_shader = Box::new(color_square::ColorSquare::default_shader(&gl)?);

        let color_square_h_line_shader = Box::new(color_square::ColorSquare::h_line_shader(&gl)?);

        let square = square::Square::new(gl);
        let color_square = color_square::ColorSquare::new(gl);

        Ok(Self {
            gl: (*gl).clone(),
            tr: text_renderer,
            viewport,
            rounded_rect_shader: rrs,
            square,
            circle_shader: cs,
            color_square_shader,
            color_square,
            color_square_h_line_shader
        })


    }

    pub fn update_viewport(&mut self, w: i32, h: i32) {
        self.viewport.w = w;
        self.viewport.h = h;

        self.viewport.set_used(&self.gl);

    }

    pub fn line(&self, x: i32, y: i32, x1: i32, y1: i32, thickness: i32) {

        let v = Vector2::<f32>::new(x as f32, y as f32) - Vector2::<f32>::new(x1 as f32, y1 as f32);


        let mut angle = -std::f32::consts::PI / 2.0;

        if v.x == 0.0 || v.y == 0.0 {
            if v.x == 0.0 { // angle is 90 or -90 (270) degrees
                angle = if y < y1 { -std::f32::consts::PI / 2.0 } else { -std::f32::consts::PI * 3.0/4.0};
            }
            else { // angle is 0 og 180
                angle = if x < x1 { 0.0 } else { std::f32::consts::PI};
            }
        }
        else  {
            angle = -f32::atan(v.y / v.x);
            if v.y *v.x <= 0.0 {
                angle -= std::f32::consts::PI;
            }
        }

        self.rounded_rect_shader.shader.set_used();

        let l = v.magnitude();

        let transform = unit_line_transform(x, y, l as f32,  thickness as f32, angle, &self.viewport);

        self.rounded_rect_shader.set_transform(transform);


        let color = Color::Rgb(0,0,0);

        self.rounded_rect_shader.set_uniforms(rrs::Uniforms { color,
                                                              pixel_height: 1.0 as f32,
                                                              pixel_width: 1.0 as f32,
                                                              radius: 0.0
        });

        self.square.render(&self.gl);
    }

    pub fn color_square(&self, x: i32, y: i32, w: i32, h: i32) {

        self.color_square_shader.set_used();

        let geom = Geometry {
            pos: Position { x, y },
            size: Size {
                pixel_w: w,
                pixel_h: h
            }
        };


        let transform = unit_square_transform_matrix(&geom, &self.viewport);
        self.color_square_shader.set_mat4(&self.gl, "transform", transform);

        self.color_square.render(&self.gl);
    }

    pub fn circle(&self, center_x: i32, center_y: i32, r: i32) {
        self.circle_shader.shader.set_used();

        let geom = Geometry {
            pos: Position { x: center_x, y: center_y },
            size: Size {
                pixel_w: r,
                pixel_h: r
            }
        };

        let transform = unit_square_transform_matrix(&geom, &self.viewport);

        self.circle_shader.set_transform(transform);

        let color_scale = 0.0;
        self.circle_shader.set_uniforms(cs::Uniforms { color_scale,
                                                      pixel_height: geom.size.pixel_h as f32,
                                                      pixel_width: geom.size.pixel_w as f32,
                                                      radius: r as f32
        });


        self.square.render(&self.gl);
    }

    pub fn rounded_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        self.rounded_rect_color(x, y, w, h, Color::Rgb(100, 100, 100));
    }


    pub fn hsv_h_line(&self, x: i32, y: i32, w: i32, h: i32) {
        self.color_square_h_line_shader.set_used();

        let geom = Geometry {
            pos: Position { x, y },
            size: Size {
                pixel_w: w,
                pixel_h: h

            }
        };

        let transform = unit_square_transform_matrix(&geom, &self.viewport);

        self.color_square_h_line_shader.set_mat4(&self.gl, "transform", transform);

        self.square.render(&self.gl);

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

        let transform = unit_square_transform_matrix(&geom, &self.viewport);


        self.rounded_rect_shader.set_transform(transform);


        self.rounded_rect_shader.set_uniforms(rrs::Uniforms { color,
                                                             pixel_height: geom.size.pixel_h as f32,
                                                             pixel_width: geom.size.pixel_w as f32,
                                                             radius: 0.0
        });

        self.square.render(&self.gl);
    }


    pub fn text_render_box(&self, text: &str, scale: f32) -> TextRenderBox {
        TextRenderer::render_box(self.tr.font(), text, 1920.0, scale)
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

        self.tr.render_text(&self.gl, text, alignment, sb, 1.0);
    }

    pub fn render_text_scaled(&mut self, text: &str, x: i32, y: i32, scale:  f32) {

        let rect = Rect {
            x,
            y,
            w: 1200,
            h: 1200
        };

        let sb = transform_to_screen_space(&rect, &self.viewport);


        let alignment = TextAlignment {
            x: TextAlignmentX::Left,
            y: TextAlignmentY::Top
        };

        self.tr.render_text(&self.gl, text, alignment, sb, scale);
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



pub fn unit_line_transform(x0: i32, y0: i32, w: f32, h: f32, angle: f32, viewport: &viewport::Viewport) -> na::Matrix4::<f32> {

    // Out target render square is defines with upper left (-0.5, 0.5) lower right (0.5,-0.5)

    // First we want to scale it to the target size
    // its a unit square, so just multiply with w and h
    let mut scale = na::Matrix4::<f32>::identity();
    scale[0] = w;
    scale[5] = h;

    // now we want to offset is so that the left edge is at 0 instead of w /2
    // so we rotate the while line, around 0.0
    let offset = Translation3::new(w / 2.0, 0.0, 0.0);

    // we want to rotate the specified angle
    let rot = Rotation::from_euler_angles(0.0, 0.0, angle);

    // translate so that our start pos is at x0, y0
    // Invert y since sdl is (0,0) in top left. Our Mapping has (0,0) in lower left
    let t = Translation3::new(x0 as f32, (viewport.h - y0) as f32, 0.0);

    // Create projectionmmatrix to go into ndc
    let proj = Orthographic3::new(0.0, viewport.w as f32, 0.0, viewport.h as f32, -10.0, 100.0);

    proj.to_homogeneous() * t.to_homogeneous() * rot.to_homogeneous() * offset.to_homogeneous() * scale

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

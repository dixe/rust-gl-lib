use super::*;
use crate::na::{self, Point3, Vector3, Translation3, geometry::Rotation, Vector2, Orthographic3};
use crate::text_rendering::text_renderer::{TextRenderer, TextRenderBox};
use crate::{gl::{self, viewport}, ScreenBox, ScreenCoords};
use crate::text_rendering::text_renderer::{TextAlignment, TextAlignmentX,TextAlignmentY};
use crate::shader::{ Shader, TransformationShader, rounded_rect_shader::{self as rrs, RoundedRectShader}, circle_shader::{self as cs, CircleShader}, texture_shader::{self as ts, TextureShader}};
use crate::objects::{square, color_square, texture_quad};
use crate::color::Color;
use crate::helpers::SetupError;
use crate::text_rendering::font::Font;
use crate::text_rendering::font_cache::FontCache;
use crate::imode_gui::numeric::Numeric;
use crate::texture::TextureId;
use crate::imode_gui::viewport::Viewport;

pub struct Drawer2D {
    // general
    pub gl: gl::Gl,
    pub tr: TextRenderer,
    pub viewport: viewport::Viewport,

    // render objects
    pub square: square::Square,
    pub texture_square: texture_quad::TextureQuad,

    //shdaers
    pub color_square: color_square::ColorSquare,
    pub rounded_rect_shader: RoundedRectShader,
    pub color_square_shader: Box::<Shader>,
    pub color_square_h_line_shader: Box::<Shader>,
    pub circle_shader: CircleShader,
    pub texture_shader: TextureShader,

    // fonts
    pub font_cache: FontCache,
}

impl Drawer2D {

    pub fn new(gl: &gl::Gl, viewport: viewport::Viewport) -> Result<Self, SetupError> {

        let inner_font = Default::default();
        let font = Font::msdf(gl, inner_font);

        let font_cache = FontCache::new(gl.clone(), font.clone(), None);

        let text_renderer = TextRenderer::new(gl, font);
        TextRenderer::setup_blend(gl);

        let texture_square = texture_quad::TextureQuad::new(gl);
        let rrs = RoundedRectShader::new(gl)?;
        let cs = CircleShader::new(gl)?;
        let color_square_shader = Box::new(color_square::ColorSquare::default_shader(&gl)?);

        let texture_shader = TextureShader::new(gl)?;

        let color_square_h_line_shader = Box::new(color_square::ColorSquare::h_line_shader(&gl)?);

        let square = square::Square::new(gl);
        let color_square = color_square::ColorSquare::new(gl);

        Ok(Self {
            font_cache,
            gl: (*gl).clone(),
            tr: text_renderer,
            viewport,
            texture_square,
            texture_shader,
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

    pub fn line<T1: Numeric, T2: Numeric, T3: Numeric, T4: Numeric, T5: Numeric>(
        &self, x_t: T1, y_t: T2, x1_t: T3, y1_t: T4, thickness_t: T5) {

        let x = x_t.to_f64();
        let y = y_t.to_f64();
        let x1 = x1_t.to_f64();
        let y1 = y1_t.to_f64();
        let thickness = thickness_t.to_f64();


        let mut v = Vector2::<f64>::new(x1.to_f64(), y1.to_f64()) - Vector2::<f64>::new(x.to_f64(), y.to_f64());


        let angle = f64::atan2(-v.y, v.x);

        self.rounded_rect_shader.shader.set_used();

        let l = v.magnitude();

        let transform = unit_line_transform(x, y, l,  thickness, angle, &self.viewport);

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

        let geom = Geom { x, y, w, h };

        let rot = na::Matrix4::<f32>::identity();
        let transform = unit_square_transform_matrix(&geom, rot, &self.viewport);
        self.color_square_shader.set_mat4(&self.gl, "transform", transform);

        self.color_square.render(&self.gl);
    }

    pub fn circle<T1: Numeric, T2: Numeric, T3: Numeric >(&self, center_x_t: T1, center_y_t: T2, r_t: T3, color: Color) {
        let center_x = center_x_t.to_f64();
        let center_y = center_y_t.to_f64();
        let r = r_t.to_f64();

        self.circle_shader.shader.set_used();

        let geom = Geom {
            x: center_x - r,
            y: center_y - r,
            w: r * 2.0,
            h: r * 2.0,
        };

        let rot = na::Matrix4::<f32>::identity();
        let transform = unit_square_transform_matrix(&geom, rot, &self.viewport);


        self.circle_shader.set_transform(transform);


        self.circle_shader.set_uniforms(cs::Uniforms { color,
                                                      pixel_height: geom.h.to_f32(),
                                                      pixel_width: geom.w.to_f32(),
                                                      radius: r as f32
        });


        self.square.render(&self.gl);
    }


    pub fn rounded_rect<T1: Numeric, T2: Numeric, T3: Numeric, T4: Numeric>(&self, x: T1, y: T2, w: T3, h: T4) {
        self.rounded_rect_color(x, y, w, h, Color::Rgb(100, 100, 100));
    }


    pub fn hsv_h_line(&self, x: i32, y: i32, w: i32, h: i32) {
        self.color_square_h_line_shader.set_used();

        let geom = Geom { x, y, w, h };

        let rot = na::Matrix4::<f32>::identity();
        let transform = unit_square_transform_matrix(&geom, rot, &self.viewport);

        self.color_square_h_line_shader.set_mat4(&self.gl, "transform", transform);

        self.square.render(&self.gl);

    }

    pub fn rounded_rect_color<T1: Numeric, T2: Numeric, T3: Numeric, T4: Numeric>(&self, x: T1, y: T2, w: T3, h: T4, color: Color) {

        self.rounded_rect_shader.shader.set_used();

        let geom = Geom { x, y, w, h };

        let rot = na::Matrix4::<f32>::identity();
        let transform = unit_square_transform_matrix(&geom, rot,  &self.viewport);


        self.rounded_rect_shader.set_transform(transform);


        self.rounded_rect_shader.set_uniforms(rrs::Uniforms { color,
                                                             pixel_height: geom.w.to_f32(),
                                                             pixel_width: geom.w.to_f32(),
                                                             radius: 0.0
        });

        self.square.render(&self.gl);
    }


    /// Get render box with the default font in text renderer
    pub fn text_render_box(&self, text: &str, pixel_size: i32) -> TextRenderBox {
        TextRenderer::render_box(self.tr.font(), text, 1920.0, pixel_size)
    }

    /// Get render box with the supplied font
    pub fn text_render_box_with_font(&self, text: &str, pixel_size: i32, font: &Font) -> TextRenderBox {
        TextRenderer::render_box(font, text, 1920.0, pixel_size)
    }

    /// Get render box with the supplied font name, maybe load the supplied font with pixel size, if pressent in font
    /// load path
    pub fn text_render_box_with_font_name(&mut self, text: &str, pixel_size: i32, font_name: &str) -> TextRenderBox {
        let font = self.font_cache.get_or_default(pixel_size, font_name);
        TextRenderer::render_box(font, text, 1920.0, pixel_size)
    }

    /// Render at x,y with default font
    pub fn render_text(&mut self, text: &str, x: i32, y: i32, pixel_size: i32) {
        let font = self.font_cache.default(pixel_size);
        render_text(&self.gl, &mut self.tr, text, x, y, &self.viewport, pixel_size, font);
    }

    /// Render at x,y with default font and given color
    pub fn render_text_with_color(&mut self, text: &str, x: i32, y: i32, pixel_size: i32, color: Color) {
        let font = self.font_cache.default(pixel_size);
        render_text(&self.gl, &mut self.tr, text, x, y, &self.viewport, pixel_size, font);
    }



    /// Render at x,y with given font name, or default font
    pub fn render_text_from_font_name(&mut self, text: &str, x: i32, y: i32, pixel_size: i32, font_name: &str) {
        let font = self.font_cache.get_or_default(pixel_size, font_name);
        render_text(&self.gl, &mut self.tr, text, x, y, &self.viewport, pixel_size, font);
    }

    /// Render at x,y with given font
    pub fn render_text_with_font(&mut self, text: &str, x: i32, y: i32, pixel_size: i32, font: &Font) {
        render_text(&self.gl, &mut self.tr, text, x, y, &self.viewport, pixel_size, font);
    }

    /// render the texture in texture_id, at x,y with size
    pub fn render_img(&mut self, texture_id: TextureId, x: i32, y: i32, size: na::Vector2::<f32>) {

        self.texture_shader.shader.set_used();

        let geom = Geom {
            x,
            y,
            w: size.x,
            h: size.y
        };

        let rot = na::Matrix4::<f32>::identity();
        let transform = unit_square_transform_matrix(&geom, rot, &self.viewport);
        self.texture_shader.setup(ts::Uniforms { texture_id, transform });

        self.texture_square.render(&self.gl);
    }

    /// render the texture in texture_id, at x,y with size and rotation angle in radians
    pub fn render_img_rot(&mut self, texture_id: TextureId, x: i32, y: i32, radians: f32, size: na::Vector2::<f32>) {

        self.texture_shader.shader.set_used();

        let geom = Geom {
            x,
            y,
            w: size.x,
            h: size.y
        };


        // Out target render square is defines with upper left (-0.5, 0.5) lower right (0.5,-0.5)

        // First we want to scale it to the target size
        // its a unit square, so just multiply with w and h
        let mut scale = na::Matrix4::<f32>::identity();
        scale[0] = size.x;
        scale[5] = size.y;

        // we want to rotate the specified angle
        let rot = Rotation::from_euler_angles(0.0, 0.0, radians);

        // translate so that our start pos is at x0, y0
        // Invert y since sdl is (0,0) in top left. Our Mapping has (0,0) in lower left
        let t = Translation3::new(x as f32 + size.x/2.0, self.viewport.h as f32 - y as f32 - size.y/2.0, 0.0);

        // Create projectionmmatrix to go into ndc
        let proj = Orthographic3::new(0.0, self.viewport.w as f32, 0.0, self.viewport.h as f32, -10.0, 100.0);

        let transform = proj.to_homogeneous() * t.to_homogeneous() * rot.to_homogeneous()  * scale;

        self.texture_shader.setup(ts::Uniforms { texture_id, transform });

        self.texture_square.render(&self.gl);

        self.texture_square.render(&self.gl);

    }
}


fn render_text(gl: &gl::Gl, tr: &mut TextRenderer, text: &str, x: i32, y: i32, viewport: &Viewport, pixel_size: i32, font: &Font) {
    let rect = Rect {
        x,
        y,
        w: 1200,
        h: 1200
    };

    let sb = transform_to_screen_space(&rect, viewport);


    let alignment = TextAlignment {
        x: TextAlignmentX::Left,
        y: TextAlignmentY::Top
    };

    tr.render_text_with_font(font, gl, text, alignment, sb, pixel_size);
}


pub fn transform_to_screen_space(rect: &Rect, viewport: &viewport::Viewport) -> ScreenBox {
    ScreenBox::new(rect.x as f32,
                   rect.y as f32,
                   rect.w as f32,
                   rect.h as f32,
                   viewport.w as f32,
                   viewport.h as f32)
}



pub fn unit_line_transform<T1: Numeric, T2: Numeric, T3: Numeric, T4: Numeric, T5: Numeric>(x0_t: T1, y0_t: T2, w_t: T3, h_t: T4, angle_t: T5, viewport: &viewport::Viewport) -> na::Matrix4::<f32> {

    let x0 = x0_t.to_f32();
    let y0 = y0_t.to_f32();
    let w = w_t.to_f32();
    let h = h_t.to_f32();
    let angle = angle_t.to_f32();

    // Out target render square is defines with upper left (-0.5, 0.5) lower right (0.5,-0.5)

    // First we want to scale it to the target size
    // its a unit square, so just multiply with w and h
    let mut scale = na::Matrix4::<f32>::identity();
    scale[0] = w;
    scale[5] = h;

    // now we want to offset is so that the left edge is at 0 instead of w /2
    // so we rotate the whole line, around 0.0
    let offset = Translation3::new(w / 2.0, 0.0, 0.0);

    // we want to rotate the specified angle
    let rot = Rotation::from_euler_angles(0.0, 0.0, angle);

    // translate so that our start pos is at x0, y0
    // Invert y since sdl is (0,0) in top left. Our Mapping has (0,0) in lower left
    let t = Translation3::new(x0 as f32, viewport.h as f32 - y0, 0.0);

    // Create projectionmmatrix to go into ndc
    let proj = Orthographic3::new(0.0, viewport.w as f32, 0.0, viewport.h as f32, -10.0, 100.0);

    proj.to_homogeneous() * t.to_homogeneous() * rot.to_homogeneous() * offset.to_homogeneous() * scale

}

fn unit_square_transform_matrix<T1: Numeric, T2: Numeric, T3: Numeric, T4: Numeric>(geom: &Geom<T1, T2, T3, T4>, rotation: na::Matrix4::<f32>, viewport: &viewport::Viewport) -> na::Matrix4::<f32> {

    let x = geom.x.to_f64() * 2.0 / viewport.w as f64- 1.0;
    let y = -geom.y.to_f64() * 2.0 / viewport.h as f64 + 1.0;
    let h = geom.h.to_f64();
    let w = geom.w.to_f64();

    let x_scale = w  / viewport.w as f64 * 2.0;
    let y_scale = h  / viewport.h as f64 * 2.0;

    let mut model = na::Matrix4::<f32>::identity();

    // Scale to size
    model[0] = x_scale as f32;
    model[5] = y_scale as f32;

    // move to position

    let x_move = x + x_scale * 0.5;
    let y_move = y - y_scale * 0.5;

    let trans = Translation3::new(x_move as f32, y_move as f32, 0.0);

    model = trans.to_homogeneous() * rotation * model;

    model
}


struct Geom<T1: Numeric, T2: Numeric, T3: Numeric, T4: Numeric> {
    x: T1,
    y: T2,
    w: T3,
    h: T4
}

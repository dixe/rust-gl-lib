//! An struct that can be used for easy text rendering

use crate::na;
use crate::text_rendering::{font::{Font, PageChar}};
use crate::shader::Shader;
use crate::texture::{self, TextureId};
use crate::gl;
use crate::objects;
use crate::*;


/// A collections of a font, shader and a texture that can render text using open.
pub struct TextRenderer {
    font: Font,
    shader: Shader,
    texture_id: TextureId,
    color: na::Vector3::<f32>,
    char_quad: Box<objects::char_quad::CharQuad>
}


impl TextRenderer {

    /// Create a new text renderer given a path to a signed distance field font
    pub fn new(gl: &gl::Gl, font: Font) -> Self {

        let shader = create_shader(gl);

        let texture_id = texture::gen_texture_rgba(&gl, &font.image);


        let char_quad = Box::new(objects::char_quad::CharQuad::new(gl));
        Self {
            font,
            shader,
            texture_id,
            char_quad,
            color: Default::default()
        }
    }

    /// Enalbes Gl_BLEND and calls BlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    pub fn setup_blend(&self, gl: &gl::Gl) {
        unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    /// Set the color that [render_text](Self::render_text) uses.
    /// The input should be is rgb and each component should be in the range \[0;1\]
    pub fn set_text_color(&mut self, color: na::Vector3::<f32>) {
        self.color = color;
    }

    fn setup_shader(&mut self, gl: &gl::Gl, scale: f32) {

        self.shader.set_used();

        self.shader.set_vec3(gl, "color", self.color);

        self.shader.set_f32(gl, "base_scale", scale);

        self.shader.set_i32(gl, "text_map", (self.texture_id - 1) as i32);

        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            texture::set_texture(gl, self.texture_id);
        }


    }


    /// user this to get size info on how a text will be rendered. Can be used in layout phase, to get side of
    /// fx a text box
    pub fn render_box(font: &Font, text: &str, max_width: f32, input_scale: f32) -> TextRenderBox {
        Self::calc_char_info(font, text, max_width, input_scale, &mut Vec::new())
    }


    fn  calc_char_info(font: &Font, text: &str, max_width: f32, _input_scale: f32, chars_info: &mut Vec::<CharPosInfo>) -> TextRenderBox {

        let mut prev_char: Option<PageChar> = None;
        let mut pixel_x = 0.0;

        for c in text.chars() {

            let chr = font.page_char(c as u32).unwrap();

            if let Some(prev) = prev_char {
                // Lookup potential kerning and apply to x
                let kern = font.kerning(prev.id, chr.id);
                pixel_x += kern;
            }

            prev_char = Some(chr);

            chars_info.push(CharPosInfo {
                x: pixel_x,
                y: 0.0,
                chr,
                is_newline: c == '\n',

            });

            pixel_x += chr.x_advance;
        }


        let mut x_offset = 0.0;
        let mut y_offset = 0.0;

        let mut total_height = 0.0;
        let mut total_width = 0.0;
        // Process chars to wrap newlines, on whole word if possible
        let mut current_max_h = font.info.line_height;


        let mut current_w = 0.0;
        for info in chars_info.iter_mut() {
            // Update x before checking to wrap correctly
            info.x -= x_offset;

            if ( info.x + info.chr.x_advance) > max_width  || info.is_newline {
                x_offset += info.x;
                y_offset += font.info.line_height;
                info.x = 0.0;

                total_height += font.info.line_height;
                total_width = f32::max(current_w, total_width);
                current_max_h = 0.0;
            }

            info.y += y_offset;

            current_max_h = f32::max(current_max_h, info.chr.height + info.chr.y_offset);
            current_w = info.x + info.chr.x_advance;
        }

        total_height += current_max_h;
        total_width = f32::max(total_width, current_w);

        TextRenderBox {
            total_width,
            total_height,
        }
    }

    /// Render text with char wrapping give screen space start x and y. The scale is how big the font is rendered.
    /// Also sets the current color, default is black. See [set_text_color](Self::set_text_color) for how to change the color.
    pub fn render_text(&mut self, gl: &gl::Gl, text: &str, alignment: TextAlignment, screen_box: ScreenBox, input_scale: f32) {

        let scale_x = 2.0 / screen_box.screen_w * input_scale;

        let scale_y = 2.0 / screen_box.screen_h * input_scale;
        let base_scale = 8.0;
        // TODO: fix this so we always render with correct scale
        self.setup_shader(gl, base_scale);

        let mut chars_info = Vec::new();

        let render_box = Self::calc_char_info(&self.font, text, screen_box.width, input_scale, &mut chars_info);

        // map from pixel space into screen space so we are ready to draw
        for info in chars_info.iter_mut() {

            let x_offset = match alignment.x {
                TextAlignmentX::Left => screen_box.x,
                TextAlignmentX::Center => screen_box.x + (screen_box.width - render_box.total_width) / 2.0,
                TextAlignmentX::Right => screen_box.x + screen_box.width - render_box.total_width,
            };

            let y_offset = match alignment.y {
                TextAlignmentY::Top => screen_box.y,
                TextAlignmentY::Center => screen_box.y + (screen_box.height - render_box.total_height) / 2.0,
                TextAlignmentY::Bottom =>  screen_box.y + screen_box.height - render_box.total_height,
            };



            info.x = info.x + x_offset;
            info.y = info.y + y_offset;
        }

        // Draw the chars
        let draw_info = DrawInfo {
            chars_info: &chars_info,
            scale: Scale { x: scale_x, y: scale_y },
            bottom: screen_box.bottom(),
            screen_w: screen_box.screen_w,
            screen_h: screen_box.screen_h,

        };


        self.render_text_quads(gl, &draw_info);
    }


    fn render_text_quads(&mut self , gl: &gl::Gl, draw_info: &DrawInfo) {

        // Draw the chars
        let buffer_size = self.char_quad.buffer_size();
        let mut i = 0;
        for info in draw_info.chars_info.iter() {
            if info.y > draw_info.bottom {
                break;
            }

            let x = smoothstep(0.0, draw_info.screen_w, info.x) * 2.0 - 1.0;
            let y = -1.0 * (smoothstep(0.0, draw_info.screen_h, info.y) * 2.0 - 1.0);

            self.char_quad.update_char(i, x, y, draw_info.scale.x, draw_info.scale.y, &info.chr, (&self.font.image).into());
            i += 1;

            if i >= buffer_size {
                self.char_quad.render(gl, i);
                i = 0;
            }
        }

        self.char_quad.render(gl, i);


    }

    pub fn font(&self) -> &Font {
        &self.font
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    f32::clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0)
}

#[derive(Debug)]
struct DrawInfo<'a>{
    bottom: f32,
    scale: Scale,
    chars_info: &'a Vec::<CharPosInfo>,
    screen_w: f32,
    screen_h: f32,

}

#[derive(Debug)]
struct Scale {
    pub x:f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct TextAlignment {
    pub x: TextAlignmentX,
    pub y: TextAlignmentY,
}

impl Default for TextAlignment {
    fn default() -> Self {

        Self {
            x: TextAlignmentX::Center,
            y: TextAlignmentY::Center
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextAlignmentX {
    Left, Center, Right
}

#[derive(Debug, Clone, Copy)]
pub enum TextAlignmentY {
    Top, Center, Bottom
}


#[derive(Debug, Clone, Copy)]
struct CharPosInfo {
    x: f32,
    y: f32,
    chr: PageChar,
    is_newline: bool,

}


#[derive(Debug, Clone)]
pub struct TextRenderBox {
    pub total_width: f32,
    pub total_height: f32,
}


fn create_shader(gl: &gl::Gl) -> Shader {
    let vert_source = r"#version 330 core
        layout (location = 0) in vec2 pos;
        layout (location = 1) in vec2 aTexCoord;


        out VS_OUTPUT {
        vec2 TexCoords;
    } OUT;


        void main()
        {
        gl_Position = vec4(pos, 0.0, 1.0);

        OUT.TexCoords = aTexCoord;
    }";

    let frag_source = r"#version 330 core
        out vec4 FragColor;
        uniform vec3 color;
        uniform float base_scale;

        uniform sampler2D text_map;


        in VS_OUTPUT {
        vec2 TexCoords;
    } IN;

        void main()
        {

        // Distance from the edge.
        // [0.0, 0.5[ is outside
        // ]0.5;1] is inside
        // And 0.5 is right on the edge
        float dist = texture(text_map, IN.TexCoords).a;


        // Just scale everything below 0.5 (ouside) to 0 and everything inside to 1s
        float u_buffer = 0.5;
        float smoothing = 1.0/base_scale;

        float alpha = smoothstep(u_buffer - smoothing, u_buffer + smoothing, dist);

        if( alpha == 0.0)
        {
            //alpha = 0.3;
            discard;
        }

        FragColor = vec4(color, alpha);
    }";


    Shader::new(gl, vert_source, frag_source).unwrap()
}

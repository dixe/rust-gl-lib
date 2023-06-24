//! An struct that can be used for easy text rendering
use crate::shader::{BaseShader, Shader};
use crate::na;
use crate::text_rendering::{font::*};
use crate::gl;
use crate::objects;
use crate::*;

/// A collections of a font, shader and a texture that can render text using open.
pub struct TextRenderer {
    font: Font,
    color: na::Vector3::<f32>,
    char_quad: Box<objects::char_quad::CharQuad>,
    smoothness: f32
}

pub enum TextSize {
    Small,
    Scaled(i32)
}


impl TextRenderer {

    /// Create a new text renderer given a path to a signed distance field font
    pub fn new(gl: &gl::Gl, font: Font) -> Self {

        let char_quad = Box::new(objects::char_quad::CharQuad::new(gl));
        Self {
            font,
            char_quad,
            color: Default::default(),
            smoothness: 2.0
        }
    }

    /// Enalbes Gl_BLEND and calls BlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    pub fn setup_blend(gl: &gl::Gl) {
        unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    /// Set the color that [render_text](Self::render_text) uses.
    /// The input should be is rgb and each component should be in the range \[0;1\]
    pub fn set_text_color(&mut self, color: na::Vector3::<f32>) {
        self.color = color;
    }

    pub fn set_smoothness(&mut self, at: f32) {
        self.smoothness = at;
    }


    /// user this to get size info on how a text will be rendered. Can be used in layout phase, to get side of
    /// fx a text box
    pub fn render_box(font: &Font, text: &str, max_width: f32, size: i32) -> TextRenderBox {
        let input_scale = size as f32 / font.size();

        let mut chars_info = Vec::new();
        let res = calc_char_info(font, text, max_width, input_scale, &mut chars_info);

        res
    }


    /// Render text with char wrapping give screen space start x and y. The scale is how big the font is rendered.
    /// Also sets the current color, default is black. See [set_text_color](Self::set_text_color) for how to change the color.
    pub fn render_text(&mut self, gl: &gl::Gl, text: &str, alignment: TextAlignment, screen_box: ScreenBox, pixel_size: i32) {

        let projection = na::geometry::Orthographic3::new(0.0, screen_box.screen_w, screen_box.screen_h, 0.0, 0.0, 10.0);
        let texture_id = self.font.texture_id();
        setup_shader(self.font.shader(), gl, projection, texture_id, self.color);
        render_text_with_font(&mut self.char_quad, &self.font, gl, text, alignment, screen_box, pixel_size);
    }

    pub fn render_text_with_font(&mut self, font: &Font, gl: &gl::Gl, text: &str, alignment: TextAlignment, screen_box: ScreenBox, pixel_size: i32) {

        let projection = na::geometry::Orthographic3::new(0.0, screen_box.screen_w, screen_box.screen_h, 0.0, 0.0, 10.0);
        let texture_id = font.texture_id();
        setup_shader(font.shader(), gl, projection, texture_id, self.color);
        render_text_with_font(&mut self.char_quad, font, gl, text, alignment, screen_box, pixel_size);
    }

    pub fn change_font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn font_mut(&mut self) -> &mut Font {
        &mut self.font
    }
}



fn render_text_quads_pixel(char_quad: &mut objects::char_quad::CharQuad, gl: &gl::Gl, draw_info: &DrawInfo) {

    // Draw the chars
    let buffer_size = char_quad.buffer_size();
    let mut i = 0;
    for info in draw_info.chars_info.iter() {
        if info.y > draw_info.bottom {
            break;
        }

        char_quad.update_char_pixels(i, info.x, info.y, draw_info.input_scale, &info.chr, draw_info.font.image().into());


        //char_quad.render_full_texture(i);

        i += 1;

        if i >= buffer_size {
            char_quad.render(gl, i);
            i = 0;
        }
    }
    char_quad.render(gl, i);
}


fn calc_char_info(font: &Font, text: &str, max_width: f32, input_scale: f32, chars_info: &mut Vec::<CharPosInfo>) -> TextRenderBox {

    let mut prev_char: Option<PageChar> = None;
    let mut pixel_x = 0.0;

    for c in text.chars() {
        let chr = match font.page_char(c as u32) {
            Some(chr) => chr,
            None => {
                // TODO: maybe in release just use unicode replacement char instead of panic
                // TODO: Also maybe replace \t with a space, so we can render tabs
                // panic!("No char with code '{}' ({}) found in font", c, c as u32)
                font.default_page_char()

            }
        };

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

        pixel_x += chr.x_advance * input_scale;
    }


    let mut x_offset = 0.0;
    let mut y_offset = 0.0;

    let mut total_height = 0.0;
    let mut total_width = 0.0;
    // Process chars to wrap newlines, on whole word if possible
    let line_height =  font.line_height() * input_scale;
    let mut current_max_h = line_height;


    let mut current_w = 0.0;
    for info in chars_info.iter_mut() {
        // Update x before checking to wrap correctly
        info.x -= x_offset;
        let x_advance = info.chr.x_advance * input_scale;

        if ( info.x + x_advance) > max_width  || info.is_newline {
            x_offset += info.x;
            y_offset += line_height;
            info.x = 0.0;

            total_height += line_height;
            total_width = f32::max(current_w, total_width);
            current_max_h = 0.0;
        }

        info.y += y_offset;

        current_max_h = f32::max(current_max_h, (info.chr.height + info.chr.y_offset) * input_scale);
        current_w = info.x + x_advance;
    }

    total_height += current_max_h;
    total_width = f32::max(total_width, current_w);

    TextRenderBox {
        total_width,
        total_height,
    }
}


pub fn render_text_with_font(char_quad: &mut objects::char_quad::CharQuad, font: &Font, gl: &gl::Gl, text: &str, alignment: TextAlignment, screen_box: ScreenBox, pixel_size: i32) {


    let input_scale = pixel_size as f32 / font.size();

    let mut chars_info = Vec::new();

    let render_box = calc_char_info(font, text, screen_box.width, input_scale, &mut chars_info);


    // higher than 1 means the text does NOT fit in the assigned box. And we have to account for scroll input
    //let text_to_window_ratio = dbg!(render_box.total_height / screen_box.screen_h);


    // maybe scroll bar should not be between 0 and 1
    // if we know the actual ration like 5.0,
    // scroll 0.0 means no offset, first word is shown in to left
    // scroll 1.0 means last word is shows on last line


    // for 0: offset with 0

    // for 1: offset with length of text - screen_box heigh

    let _scroll = 0.5;
    //let offset = dbg!(render_box.total_height * scroll - screen_box.screen_h);
    // map from pixel space into screen space so we are ready to draw
    for info in chars_info.iter_mut() {

        let x_offset = match alignment.x {
            TextAlignmentX::Left => screen_box.x,
            TextAlignmentX::Center => screen_box.x + (screen_box.width - render_box.total_width) / 2.0,
            TextAlignmentX::Right => screen_box.x + screen_box.width - render_box.total_width,
        };

        let y_offset = match alignment.y {
            TextAlignmentY::Top => screen_box.y,
            TextAlignmentY::Center => screen_box.y + (screen_box.height - f32::min(screen_box.height, render_box.total_height)) / 2.0,
            TextAlignmentY::Bottom =>  screen_box.y + screen_box.height - render_box.total_height,
        };

        info.x = info.x + x_offset;
        info.y = info.y + y_offset;
    }

    // Draw the chars
    let draw_info = DrawInfo {
        chars_info: &chars_info,
        bottom: screen_box.bottom(),
        input_scale: input_scale,
        font
    };

    render_text_quads_pixel(char_quad, gl, &draw_info);
}


fn setup_shader(shader: &BaseShader, gl: &gl::Gl, projection: na::geometry::Orthographic3::<f32>, texture_id: u32, color: na::Vector3::<f32>) {

    shader.set_used();

    shader.set_vec3(gl, "color", color);

    shader.set_i32(gl, "text_map", 0);

    shader.set_mat4(gl, "projection", projection.to_homogeneous());

    TextRenderer::setup_blend(gl);

    unsafe {
        gl.ActiveTexture(gl::TEXTURE0);
        texture::set_texture(gl, texture_id);
    }
}



#[derive(Debug)]
struct DrawInfo<'a>{
    bottom: f32,
    chars_info: &'a Vec::<CharPosInfo>,
    input_scale: f32,
    font: &'a Font
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TextAlignment {
    pub x: TextAlignmentX,
    pub y: TextAlignmentY,
}


#[derive(Debug, Clone, Copy)]
pub enum TextAlignmentX {
    Left, Center, Right
}

impl Default for TextAlignmentX {
    fn default() -> Self {
        TextAlignmentX::Center
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextAlignmentY {
    Top, Center, Bottom
}

impl Default for TextAlignmentY {
    fn default() -> Self {
        TextAlignmentY::Center
    }
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

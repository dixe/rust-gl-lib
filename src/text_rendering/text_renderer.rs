//! An struct that can be used for easy text rendering

use std::path::Path;
use crate::na;
use crate::text_rendering::{font::{Font, PageChar}};
use crate::shader::Shader;
use crate::texture::{self, TextureId};
use crate::gl;
use crate::objects;


/// A collections of a font, shader and a texture that can render text using open.
pub struct TextRenderer {
    font: Font,
    shader: Shader,
    texture_id: TextureId,
}


impl TextRenderer {

    /// Create a new text renderer given a path to a signed distance field font
    pub fn new(gl: &gl::Gl, font_path: &Path) -> Self {
        // TODO: return result from font load

        // TODO maybe take optional font as parameter
        let font = Font::load_fnt_font(font_path).unwrap();

        unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let shader = create_shader(gl);


        let texture_id = texture::gen_texture_rgba(&gl, &font.image);


        Self {
            font,
            shader,
            texture_id
        }
    }

    /// Render text with char wrapping give screen space start x and y. The scale is how big the font is rendered
    pub fn render_text(&self, gl: &gl::Gl, text: &str, screen_x: f32, screen_y: f32, input_scale: f32) {
        let scale = input_scale/self.font.info.scale.w;

        self.shader.set_used();


        let color = na::Vector3::new(0.0, 0.0, 0.0);
        // set color
        self.shader.set_vec3(gl, "color", &color);

        let base_scale = 8.0;
        self.shader.set_f32(gl, "scale", input_scale * base_scale);
        self.shader.set_i32(gl, "text_map", (self.texture_id - 1) as i32);

        // Most basic way is to generate a new char_quad for each char in text and render it

        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            texture::set_texture(gl, self.texture_id);
        }



        let mut chars_info = Vec::new();


        let mut x = screen_x;
        let mut prev_char: Option<PageChar> = None;

        for c in text.chars() {

            let chr = self.font.page_char(c as u32).unwrap();



            if let Some(prev) = prev_char {
                // Lookup potential kerning and apply to x
                let kern = self.font.kerning(prev.id, chr.id) * scale;
                x += kern;
            }

            prev_char = Some(chr);

            chars_info.push(CharPosInfo {
                x,
                y: screen_y,
                is_whitespace: c.is_whitespace(),
                chr,
            });
            x += chr.x_advance * scale;
        }

        // Process chars to wrap newlines, on whole word if possible
        let mut x_offset = 0.0;
        let mut y_offset = 0.0;

        for info in chars_info.iter_mut() {
            // Update x before checking to
            info.x -= x_offset;

            if (info.x + info.chr.width * scale) >= 1.0 {
                x_offset +=  info.x - screen_x;
                y_offset += self.font.info.line_height * scale;
                info.x = screen_x;

            }

            info.y -= y_offset;

        }


        // Draw the chars
        for info in chars_info.iter() {
            let char_quad = objects::char_quad::CharQuad::new(gl, info.x, info.y, scale, &info.chr, (&self.font.image).into());
            char_quad.render(gl);
        }

    }
}


struct CharPosInfo {
    is_whitespace: bool,
    x: f32,
    y: f32,
    chr: PageChar
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
uniform float scale;

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
    float smoothing = 1.0/scale;

    float alpha = smoothstep(u_buffer - smoothing, u_buffer + smoothing, dist);

    if(alpha == 0.0)
    {
        discard;
    }
    FragColor = vec4(color, alpha);
}";


    Shader::new(gl, vert_source, frag_source).unwrap()
}

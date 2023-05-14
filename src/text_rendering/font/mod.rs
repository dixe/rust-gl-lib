use crate::shader::{BaseShader};
use crate::gl;
use crate::texture::{self, TextureId};

mod fnt_font;
pub use self::fnt_font::*;

mod msdf_font;
pub use self::msdf_font::*;

#[derive(Debug, Clone)]
pub struct Font {
    texture_id: TextureId,
    shader: BaseShader,
    inner_font: InnerFont
}

#[derive(Debug, Clone)]
pub enum InnerFont {
    Fnt(FntFont),
    Msdf(MsdfFont)
}


pub enum FontType {
    Fnt,
    Msdf
}

impl Font {

    pub fn msdf(gl: &gl::Gl, inner: MsdfFont) -> Self {
        let id = texture::gen_texture_rgba(gl, &inner.image);
        let shader = inner.create_shader(gl);
        Self {
            texture_id: id,
            shader,
            inner_font: InnerFont::Msdf(inner)
        }
    }

    pub fn fnt(gl: &gl::Gl, inner: FntFont) -> Self {
        let id = texture::gen_texture_rgba(gl, &inner.image);
        let shader = inner.create_shader(gl);
        Self {
            texture_id: id,
            shader,
            inner_font: InnerFont::Fnt(inner)
        }
    }

    pub fn image(&self) -> &image::RgbaImage {

        match &self.inner_font {
            InnerFont::Fnt(fnt) => &fnt.image,
            InnerFont::Msdf(msdf) => &msdf.image,
        }
    }

    pub fn page_char(&self, c: u32) -> Option<PageChar> {
        match &self.inner_font {
            InnerFont::Fnt(fnt) => fnt.page_char(c),
            InnerFont::Msdf(msdf) => msdf.page_char(c)
        }
    }

    pub fn font_type(&self) -> FontType {
         match &self.inner_font {
            InnerFont::Fnt(fnt) => FontType::Fnt,
            InnerFont::Msdf(msdf) => FontType::Msdf
        }
    }

    pub fn name(&self) -> &str {
        match &self.inner_font {
            InnerFont::Fnt(fnt) => &fnt.info.face,
            InnerFont::Msdf(msdf) => &msdf.info.name
        }
    }

    pub fn default_page_char(&self) -> PageChar {
        match &self.inner_font {
            InnerFont::Fnt(fnt) => fnt.page.chars[0],
            InnerFont::Msdf(msdf) => msdf.chars[0]
        }
    }

    /// Return the kerning between a left and a right char. Defaults to 0.0
    pub fn kerning(&self, left: u32, right: u32) -> f32 {
         match &self.inner_font {
             InnerFont::Fnt(fnt) => fnt.kerning(left, right),
             InnerFont::Msdf(msdf) => msdf.kerning(left, right)
        }
    }

    pub fn line_height(&self) -> f32 {
        match &self.inner_font {
            InnerFont::Fnt(fnt) => fnt.info.line_height,
            InnerFont::Msdf(msdf) => msdf.line_height
        }
    }

    pub fn size(&self) -> f32 {
        match &self.inner_font {
            InnerFont::Fnt(fnt) => fnt.info.size as f32,
            InnerFont::Msdf(msdf) => msdf.pixel_size
        }
    }


    pub fn create_shader(&self, gl: &gl::Gl) -> BaseShader {
        match &self.inner_font {
            InnerFont::Fnt(fnt) => fnt.create_shader(gl),
            InnerFont::Msdf(msdf) => msdf.create_shader(gl),
        }
    }

    pub fn change_shader(&mut self, shader: BaseShader) {
        self.shader = shader
    }

    pub fn shader(&self) -> &BaseShader {
        &self.shader
    }

    pub fn texture_id(&self, gl: &gl::Gl) -> TextureId {
        self.texture_id
    }
}


#[derive(Debug, Default, Clone, Copy)]
pub struct PageChar {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
    pub y_advance: f32,
}

mod sdf_font;
pub use self::sdf_font::*;
use crate::shader::{BaseShader,Shader};
use crate::gl;


mod msdf_font;
pub use self::msdf_font::*;

#[derive(Debug)]
pub enum Font {
    Sdf(SdfFont),
    Msdf(MsdfFont)
}


impl Font {

    pub fn image(&self) -> &image::RgbaImage {

        match self {
            Font::Sdf(sdf) => &sdf.image,
            Font::Msdf(msdf) => &msdf.image,
        }
    }

    pub fn page_char(&self, c: u32) -> Option<PageChar> {
        match self {
            Font::Sdf(sdf) => sdf.page_char(c),
            Font::Msdf(msdf) => msdf.page_char(c)
        }
    }

    pub fn default_page_char(&self) -> PageChar {
        match self {
            Font::Sdf(sdf) => panic!("No default set for sdf font"),
            Font::Msdf(msdf) => msdf.chars[0]
        }
    }

    /// Return the kerning between a left and a right char. Defaults to 0.0
    pub fn kerning(&self, left: u32, right: u32) -> f32 {
         match self {
             Font::Sdf(sdf) => sdf.kerning(left, right),
             Font::Msdf(msdf) => msdf.kerning(left, right)
        }
    }

    pub fn line_height(&self) -> f32 {
        match self {
            Font::Sdf(sdf) => sdf.info.line_height,
            Font::Msdf(msdf) => msdf.line_height
        }
    }


    pub fn create_shader(&self, gl: &gl::Gl) -> BaseShader {
        match self {
            Font::Sdf(sdf) => sdf.create_shader(gl),

            Font::Msdf(msdf) => msdf.create_shader(gl),
        }
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

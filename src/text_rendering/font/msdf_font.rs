#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use serde_json::Result;

use crate::shader::{BaseShader};
use crate::gl;
use super::*;


#[derive(Debug, Clone)]
pub struct MsdfFont {
    pub image: image::RgbaImage, // TODO: only store height and width, and not full image, for each font
    pub info: FontInfo,
    pub chars: Vec::<PageChar>,
    pub line_height: f32,
    pub pixel_size: f32
}


static FONT_JSON: &str = include_str!("../../../assets/fonts/msdf_consolas.json");
static FONT_IMG: &[u8] = include_bytes!("../../../assets/fonts/msdf_consolas.png");


impl Default for MsdfFont {
    fn default() -> Self {
        let loaded_img = match image::load_from_memory(FONT_IMG) {
            Ok(img) => img,
            Err(err) => panic!("Load default font, creating image failed with: {}", err)
        };

        let image = loaded_img.into_rgba8();

        match MsdfFont::load_font(FONT_JSON, image) {
            Ok(font) => font,
            Err(err) => panic!("Load default font failed with: {}", err)
        }
    }
}



impl MsdfFont {

    pub fn load_from_paths(json_p: &str, img_p: &str) -> Result<MsdfFont> {

        let jp = std::fs::canonicalize(json_p).unwrap();
        let ip = std::fs::canonicalize(img_p).unwrap();

        let json = std::fs::read_to_string(jp).unwrap();
        let img = std::fs::read(ip).unwrap();

        let loaded_img = image::load_from_memory(&img).unwrap();

        let image = loaded_img.into_rgba8();

        Self::load_font(&json, image)

    }

    pub fn load_font(text: &str, image: image::RgbaImage) -> Result<MsdfFont> {

        let info : FontInfo = serde_json::from_str(text)?;

        let line_height = info.metrics.lineHeight * info.atlas.size;

        let mut chars = vec![];

        // Add newline
        chars.push(PageChar {
            id: 10,
            height: 0.0,
            width: 0.0,
            x: 0.0,
            y: 0.0,
            x_advance: 0.0,
            x_offset: 0.0,
            y_advance: 0.0,
            y_offset: 0.0,

        });

        for g in &info.glyphs {

            let mut chr = PageChar {
                id: g.unicode,
                height: 0.0,
                width: 0.0,
                x: 0.0,
                y: 0.0,
                x_advance: info.atlas.size * g.advance,
                x_offset: 0.0,
                y_advance: 0.0,
                y_offset: 0.0,
            };

            if let Some(atlas) = &g.atlasBounds {

                let plane_b = g.planeBounds.unwrap_or_default();

                let h = atlas.top - atlas.bottom;
                let w = atlas.right - atlas.left;

                chr.height = h;
                chr.width = w;
                chr.x = atlas.left;
                chr.y = atlas.top;
                chr.x_advance = info.atlas.size * g.advance;
                chr.x_offset = 0.0;
                chr.y_advance = 0.0;
                // default with y_offset = 0 , then the top of every char is aligned.
                // so take size, which is like square around char, fx 32
                // first subtract height, this will align everything by the bottom of the char, fx bottom of 'p' is at the same level
                // as as 'a'
                // to push fx 'p' down and subtract h * plane_b.bottom);
                chr.y_offset = info.atlas.size - h - (h * plane_b.bottom);
            }

            chars.push(chr);
        }


        let pixel_size = info.atlas.size;
        let font = MsdfFont {
            image,
            chars,
            info,
            line_height,
            pixel_size
        };

        Ok(font)
    }

    /// Return the page char if it exists in the font
    pub fn page_char(&self, char_id: u32) -> Option<PageChar> {
        for c in &self.chars {
            if c.id == char_id {
                return Some(*c);
            }
        }

        None
    }

    pub fn kerning(&self, _left: u32, _right: u32) -> f32 {
        0.0
    }

    pub fn create_shader(&self, gl: &gl::Gl) -> BaseShader {

        let vert_source = include_str!("../../../assets/shaders/msdf_text_render.vert");
        let frag_source = include_str!("../../../assets/shaders/msdf_text_render.frag");

        BaseShader::new(gl, vert_source, frag_source).unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FontInfo {
    pub atlas: Atlas,
    pub name: String,
    metrics: Metrics,
    glyphs: Vec::<Glyph>,
    kerning: Vec::<Kerning>
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Atlas {
    #[serde(alias = "type")]
    type_: String,
    pub size: f32,
    width: u32,
    height: u32,
    #[allow(non_snake_case)]
    yOrigin: String
}


#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct Metrics {
    #[allow(non_snake_case)]
    emSize: u32,
    #[allow(non_snake_case)]
    lineHeight: f32,
    ascender: f32,
    descender: f32,
    #[allow(non_snake_case)]
    underlineY: f32,
    #[allow(non_snake_case)]
    underlineThickness: f32
}


#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Glyph {
    unicode: u32,
    advance: f32,
    planeBounds: Option<PlaneBounds>,
    atlasBounds: Option<AtlasBounds>,

}


#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct Kerning {
    unicode1: u32,
    unicode2: u32,
    advance: f32
}


#[derive(Default, Copy, Clone, Debug, Serialize,Deserialize)]
pub struct PlaneBounds {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct AtlasBounds {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

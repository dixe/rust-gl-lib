use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::path::Path;
use std::fs;
use std::error::Error;
use std::str::{FromStr};
use std::fmt;
use itertools::Itertools;
use image::io::Reader as ImageReader;
use image::imageops;
use crate::shader::{BaseShader,Shader};
use crate::gl;

use super::*;
use crate::na;

#[derive(Debug)]
pub struct MsdfFont {
    pub image: image::RgbaImage,
    pub chars: Vec::<PageChar>,
    pub line_height: f32,
}


static FONT_JSON: &str = include_str!("../../../assets/fonts/msdf_arial.json");
static FONT_IMG: &[u8] = include_bytes!("../../../assets/fonts/msdf_arial.png");



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

    pub fn load_font(text: &str, mut image: image::RgbaImage) -> Result<MsdfFont> {

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

            let mut pc = PageChar {
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


                pc.height = h;
                pc.width = w;
                pc.x = atlas.left;
                pc.y = atlas.top;
                pc.x_advance = info.atlas.size * g.advance;
                pc.x_offset = 0.0;
                pc.y_advance = 0.0;
                pc.y_offset = info.atlas.size - h - (h * plane_b.bottom);
            }

            chars.push(pc);
        }


        image = imageops::flip_vertical(&image);
        let font = MsdfFont {
            image,
            chars ,
            line_height
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

    pub fn kerning(&self, left: u32, right: u32) -> f32 {
        0.0
    }

    pub fn create_shader(&self, gl: &gl::Gl) -> BaseShader {

        let vert_source = include_str!("../../../assets/shaders/msdf_text_render.vert");
        let frag_source = include_str!("../../../assets/shaders/msdf_text_render.frag");

        BaseShader::new(gl, vert_source, frag_source).unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct FontInfo {
    atlas: Atlas,
    name: String,
    metrics: Metrics,
    glyphs: Vec::<Glyph>,
    kerning: Vec::<Kerning>
}

#[derive(Debug,Serialize, Deserialize)]
struct Atlas {
    #[serde(alias = "type")]
    type_: String,
    distanceRange: i32,
    size: f32,
    width: u32,
    height: u32,
    yOrigin: String
}


#[derive(Debug, Serialize,Deserialize)]
struct Metrics {
    emSize: u32,
    lineHeight: f32,
    ascender: f32,
    descender: f32,
    underlineY: f32,
    underlineThickness: f32
}


#[derive(Debug,Serialize, Deserialize)]
struct Glyph {
    unicode: u32,
    advance: f32,
    planeBounds: Option<PlaneBounds>,
    atlasBounds: Option<AtlasBounds>,

}


#[derive(Debug, Serialize,Deserialize)]
struct Kerning {
    unicode1: u32,
    unicode2: u32,
    advance: f32
}


#[derive(Default, Copy, Clone, Debug, Serialize,Deserialize)]
struct PlaneBounds {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

#[derive(Debug, Serialize,Deserialize)]
struct AtlasBounds {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

use crate::gl;
use std::collections::HashMap;
use crate::text_rendering::font::{FontType, Font, MsdfFont};
use std::path::Path;
use path_absolutize::*;

/// Cache that will load fonts when needed, might at some point also unload unused fonts
pub struct FontCache {
    pub default: Font,
    pub fonts_path: Option<String>,
    pub msdf_fonts: HashMap::<String, Font>,
    pub softmask_fonts: HashMap::<String, HashMap::<i32, Font>>,
    gl: gl::Gl
}


impl FontCache {

    pub fn new(gl: gl::Gl, default: Font, fonts_path: Option<String>) -> Self {

        Self {
            default,
            fonts_path,
            msdf_fonts: Default::default(),
            softmask_fonts: Default::default(),
            gl
        }
    }

    pub fn default(&mut self, pixel_size: i32) -> &Font {
        let name = self.default.name().to_owned();

        self.get_or_default(pixel_size, &name)
    }

    pub fn get_or_default(&mut self, pixel_size: i32, font_name: &str) -> &Font {

        if pixel_size >= 20  {
            // see if we have msdf, and use that, otherwise use default
            if let Some(font) = self.get_msdf_font(font_name) {
                return font;
            }

            return &self.default;
        }

        // else see if we have the specified font at the specified size.
        if self.has_softmask_font(font_name, pixel_size) { // has to use this check and then unwrap, otherwise borrow checker compains
            return self.get_softmask_font(font_name, pixel_size).unwrap();
        }

        if let Some(font) = self.try_load_softmask_from_path(font_name, pixel_size) {
            self.add_font_with_pixel_size(font_name, font);
            self.get_softmask_font(font_name, pixel_size).unwrap();
        }

        // TODO: Loading creates some challanges with mut on call site, maybe we can figure it out, for now
        // not hot loading of fonts. Might also be the best way to go, since otherwise we will "randomly"
        // start loading fonts and their images

        // TODO: see if we have one with same name, but close in pixel size else return default
        &self.default
    }


    fn has_softmask_font(&self, font_name: &str, pixel_size: i32) -> bool {
        if let Some(map) = self.softmask_fonts.get(font_name) {
            return map.contains_key(&pixel_size);
        }

        false

    }

    fn get_msdf_font(&self, font_name: &str) -> Option<&Font> {
        self.msdf_fonts.get(font_name)
    }


    fn get_softmask_font(&self, font_name: &str, pixel_size: i32) -> Option<&Font> {
        if let Some(map) = self.softmask_fonts.get(font_name) {
            return map.get(&pixel_size);
        }
        None
    }


    fn try_load_softmask_from_path(&self, font_name: &str, pixel_size: i32) -> Option<Font> {
        match &self.fonts_path {
            Some(p) => {
                let json_name = format!("softmask_{font_name}_{pixel_size}.json");
                let png_name = format!("softmask_{font_name}_{pixel_size}.png");
                let json_path = Path::new(p).join(json_name);
                let png_path = Path::new(p).join(png_name);
                if json_path.exists() && png_path.exists() {
                    match MsdfFont::load_from_paths(&json_path, &png_path) {
                        Ok(font) => {
                            return Some(Font::msdf(&self.gl, font));
                        },
                        Err(err) => {
                            println!("Load font from '{:?}' failed {:?}", json_path.absolutize(), err);
                            return None;
                        }
                    }
                }
                return None;

            },
            _ => None
        }
    }

    fn add_font_with_pixel_size(&mut self, font_name: &str, font: Font) {
        if !self.softmask_fonts.contains_key(font_name) {
            self.softmask_fonts.insert(font_name.to_string(), Default::default());
        }

        let size = font.size() as i32;
        let map = self.softmask_fonts.get_mut(font_name).unwrap();

        map.insert(size, font);

    }

    fn add_font(&mut self, font: Font) {
        let font_name = font.name();

        match font.font_type() {
            FontType::Fnt => {
                if !self.softmask_fonts.contains_key(font_name) {
                    self.softmask_fonts.insert(font_name.to_string(), Default::default());
                }

                let size = font.size() as i32;
                let map = self.softmask_fonts.get_mut(font_name).unwrap();

                map.insert(size, font);
            },
            FontType::Msdf => {
                self.msdf_fonts.insert(font_name.to_string(), font);
            }
        }
    }
}

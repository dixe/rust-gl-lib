use std::collections::HashMap;
use crate::text_rendering::font::{FontType, Font};


/// Cache that will load fonts when needed, might at some point also unload unused fonts
pub struct FontCache {
    pub default: Font,
    fonts_path: Option<String>,
    msdf_fonts: HashMap::<String, Font>,
    fnt_fonts: HashMap::<String, HashMap::<i32, Font>>
}


impl FontCache {

    pub fn new(default: Font, fonts_path: Option<String>) -> Self {

        Self {
            default,
            fonts_path,
            msdf_fonts: Default::default(),
            fnt_fonts: Default::default(),
        }
    }

    pub fn default(&self) -> &Font {
        &self.default
    }

    pub fn get_or_default(&self, pixel_size: i32, font_name: &str) -> &Font {
        if pixel_size > 20  {
            // see if we have msdf, and use that, otherwise use default
            if let Some(font) = self.get_msdf_font(font_name) {
                return font;
            }

            return &self.default;
        }

        // else see if we have the specified font at the specified size.
        if let Some(font) = self.get_fnt_font(font_name, pixel_size) {
            return font;
        }

        // TODO: Loading creates some challanges with mut on call site, maybe we can figure it out, for now
        // not hot loading of fonts. Might also be the best way to go, since otherwise we will "randomly"
        // start loading fonts and their images

        // TODO: see if we have one with same name, but close in pixel size else return default
        &self.default
    }


    fn get_msdf_font(&self, font_name: &str) -> Option<&Font> {
        self.msdf_fonts.get(font_name)
    }


    fn get_fnt_font(&self, font_name: &str, pixel_size: i32) -> Option<&Font> {
        if let Some(map) = self.fnt_fonts.get(font_name) {
            return map.get(&pixel_size);
        }
        None
    }


    fn try_load_fnt_from_path(&mut self, font_name: &str, pixel_size: i32) -> Option<Font> {
        None
    }

    pub fn add_font(&mut self, font: Font) {
        let font_name = font.name();
        match font.font_type() {
            FontType::Fnt => {
                if !self.fnt_fonts.contains_key(font_name) {
                    self.fnt_fonts.insert(font_name.to_string(), Default::default());
                }

                let size = font.size() as i32;
                let map = self.fnt_fonts.get_mut(font_name).unwrap();

                map.insert(size, font);
            },
            FontType::Msdf => {
                self.msdf_fonts.insert(font_name.to_string(), font);
            }
        }
    }
}

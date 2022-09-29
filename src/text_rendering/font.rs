//! Signed distance field fonts

use std::path::Path;
use std::fs;
use std::error::Error;
use std::str::{FromStr};
use std::fmt;
use itertools::Itertools;
use image::io::Reader as ImageReader;
use image::imageops;

use crate::na;

#[derive(Debug)]
pub struct Font {
    pub info: FontInfo,
    pub page: Page,
    pub image: image::RgbaImage
}


static FONT_TEXT: &str = include_str!("../../assets/fonts/Arial.fnt");
static FONT_IMG: &[u8; 85355] = include_bytes!("../../assets/fonts/Arial.png");

//static FONT_TEXT: &str = include_str!("E:/repos/rust-sdf-tool/Calibri.fnt");
//static FONT_IMG: &[u8; 87666] = include_bytes!("E:/repos/rust-sdf-tool/Calibri_0.png");


impl Default for Font {
    fn default() -> Self {

        let loaded_img = match image::load_from_memory(FONT_IMG) {
            Ok(img) => img,
            Err(err) => panic!("Load default font, creating image failed with: {}", err)
        };

        let image = loaded_img.into_rgba8();

        match Font::load_font(FONT_TEXT, image) {
            Ok(font) => font,
            Err(err) => panic!("Load default font failed with: {}", err)
        }
    }
}

//TODO: use this to calculate the SDF from TTF https://www.youtube.com/watch?v=LaYPoMPRSlk


impl Font {

    /// Assumes that the png file referred to in the font is located in the same directory as the .fnt file.
    /// Fonts generated from steps here: <https://github.com/libgdx/libgdx/wiki/Distance-field-fonts>]


    pub fn load_font(text: &str, mut image: image::RgbaImage) -> Result<Font, Box<dyn Error>> {

        let mut lines = text.lines();

        let info_lines: Vec::<&str> = lines.take_while_ref(|l| !l.starts_with("page ")).collect();

        let info: FontInfo = info_lines.join(" ").parse()?;

        // The rest is page. Maybe assuming single page is an error;
        let mut page: Page = lines.collect::<Vec<&str>>().join("\n").parse()?;

        image = imageops::flip_vertical(&image);

        // image is flipped so also flip chars
        for c in &mut page.chars {
            c.y = image.height() as f32 - c.y;
        }

        Ok(Font {
            info,
            page,
            image,
        })

    }

    pub fn load_fnt_font(fnt_path: &Path) -> Result<Font, Box<dyn Error>> {




        let text = fs::read_to_string(fnt_path)?;

        let mut lines = text.lines();

        let info_lines: Vec::<&str> = lines.take_while_ref(|l| !l.starts_with("page ")).collect();

        let info: FontInfo = info_lines.join(" ").parse()?;

        // The rest is page. Maybe assuming single page is an error;
        let mut page: Page = lines.collect::<Vec<&str>>().join("\n").parse()?;


        let parent = fnt_path.parent().ok_or(ParseFontError::PathHasNotParent)?;
        let img_path = parent.join(&page.info.file_name);

        let mut image = ImageReader::open(img_path)?.decode()?.into_rgba8();

        return Self::load_font(&text, image);

        /*
        image = imageops::flip_vertical(&image);

        // image is flipped so also flip chars
        for c in &mut page.chars {
            c.y = image.height() as f32 - c.y;
        }

        Ok(Font {
            info,
            page,
            image,
        })
*/
    }

    /// Return the page char if it exists in the font
    pub fn page_char(&self, char_id: u32) -> Option<PageChar> {

        for c in &self.page.chars {

            if c.id == char_id {
                return Some(*c);
            }
        }
        None
    }

    /// Return the kerning between a left and a right char. Defaults to 0.0
    pub fn kerning(&self, left: u32, right: u32) -> f32 {

        for kerning in self.page.kernings.iter() {

            if kerning.first_id == left && kerning.second_id == right {
                return kerning.amount;
            }
        }

        0.0
    }
}


#[derive(Debug, Default)]
pub struct FontInfo {
    pub spacing: na::Vector2::<i32>,
    pub face: String,
    pub size: i32,
    pub stretch_h: i32,
    pub padding: Padding,
    pub aa: i32,
    pub smooth: i32,
    pub line_height: f32,
    pub scale: Scale,
    pub pages: i32,
    pub packed: bool,
    pub base: i32,
}

impl FromStr for FontInfo  {
    type Err = ParseFontError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut input = s.to_string();
        let rest = input.split_off(5);
        let parts = rest.split(" ");

        let mut info: FontInfo = Default::default();

        for part in parts {
            let splitted: Vec::<&str> = part.split("=").collect();
            if splitted.len() != 2 {
                continue;
            }

            match splitted[0] {
                "face" =>  {
                    info.face = splitted[1].to_string();
                },
                "size" => {
                    info.size = splitted[1].parse()?;
                },
                "stretchH" => {
                    info.stretch_h = splitted[1].parse()?;
                },
                "smooth" => {
                    info.smooth = splitted[1].parse()?;
                },
                "aa" => {
                    info.aa = splitted[1].parse()?;
                },
                "padding" => {
                    info.padding = splitted[1].parse()?;
                },

                "spacing" => {
                    let splt: Vec::<&str> = splitted[1].split(",").collect();
                    info.spacing.x = splt[0].parse().unwrap();
                    info.spacing.y = splt[1].parse().unwrap();
                },
                "lineHeight" => {
                    info.line_height = splitted[1].parse()?;
                },

                "scaleW" => {
                    info.scale.w = splitted[1].parse()?;
                },
                "scaleH" => {
                    info.scale.h = splitted[1].parse()?;
                },
                "pages" => {
                    info.pages = splitted[1].parse()?;
                },
                "packed" => {
                    let val : i32 = splitted[1].parse()?;
                    info.packed = val > 0;
                },
                "base" => {
                    info.base = splitted[1].parse()?;
                },
                _ => { }
            }
        }

        Ok(info)
    }
}

#[derive(Debug, Default)]
pub struct Page {
    pub info: PageInfo,
    pub chars: Vec::<PageChar>,
    pub kernings: Vec::<Kerning>
}

impl FromStr for Page  {
    type Err = ParseFontError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut page: Page = Default::default();

        let info_line = lines.next().ok_or(ParseFontError::ParsePageError("No page info line".to_string()))?;

        page.info = info_line.parse()?;

        // skip the line that says chars count=XXX
        let _ = lines.next().ok_or(ParseFontError::ParsePageError("No chars info line".to_string()))?;

        for char_line in lines.take_while_ref(|l| l.starts_with("char")) {
            let page_char = char_line.parse()?;
            page.chars.push(page_char);
        }


        // skip the line that says kernings  count=XXX
        let _ = lines.next().ok_or(ParseFontError::ParsePageError("No kernings info line".to_string()))?;

        for kerning_line in lines.take_while_ref(|l| l.starts_with("kerning")) {
            let page_kerning = kerning_line.parse()?;
            page.kernings.push(page_kerning);
        }

        Ok(page)

    }
}

#[derive(Debug, Default)]
pub struct PageInfo {
    pub id: usize,
    pub file_name: String,
}


impl FromStr for PageInfo  {
    type Err = ParseFontError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let parts = s.split(" ");

        let mut info: PageInfo = Default::default();

        for part in parts {
            let splitted: Vec::<&str> = part.split("=").collect();
            if splitted.len() != 2 {
                continue;
            }

            match splitted[0] {
                "id" =>  {
                    info.id = splitted[1].parse()?;

                },
                "file" => {
                    info.file_name = splitted[1].to_string().replace("\"","");

                },
                a => return Err(ParseFontError::ParsePageError(format!("Parsing page info found unknown '{}'", a)))
            };
        }

        Ok(info)
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
    pub page_id: usize,
    pub channel: i32,
}

impl FromStr for PageChar  {
    type Err = ParseFontError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut pc: PageChar = Default::default();
        for part in s.split(" "){
            let splitted: Vec::<&str> = part.split("=").collect();

            match splitted[0] {
                "id" => {
                    pc.id = splitted[1].parse()?;
                },
                "x" => {
                    pc.x = splitted[1].parse()?;
                },
                "y" => {
                    pc.y = splitted[1].parse()?;
                },
                "width" => {
                    pc.width = splitted[1].parse()?;
                },
                "height" => {
                    pc.height = splitted[1].parse()?;
                },
                "xoffset" => {
                    pc.x_offset = splitted[1].parse()?;
                },
                "yoffset" => {
                    pc.y_offset = splitted[1].parse()?;
                },
                "xadvance" => {
                    pc.x_advance = splitted[1].parse()?;
                },
                "yadvance" => {
                    pc.y_advance = splitted[1].parse()?;
                },
                "page" => {
                    pc.page_id = splitted[1].parse()?;
                },
                "chnl" => {
                    pc.channel = splitted[1].parse()?;
                },
                _ => {}
            }
        }

        Ok(pc)
    }
}

#[derive(Debug, Default)]
pub struct Kerning {
    pub first_id: u32,
    pub second_id: u32,
    pub amount: f32
}

impl FromStr for Kerning  {
    type Err = ParseFontError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut kern: Kerning = Default::default();
        for part in s.split(" "){
            let splitted: Vec::<&str> = part.split("=").collect();

            match splitted[0] {
                "first" => {
                    kern.first_id = splitted[1].parse()?;
                },
                "second" => {
                    kern.second_id = splitted[1].parse()?;
                },
                "amount" => {
                    kern.amount = splitted[1].parse()?;
                },
                _=> {}
            }
        }

        Ok(kern)
    }
}

#[derive(Debug, Default)]
pub struct Scale {
    pub w: f32,
    pub h: f32
}


#[derive(Debug, Default)]
pub struct Padding {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}


impl FromStr for Padding  {
    type Err = ParseFontError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec::<&str> = s.split(",").collect();

        let top = splitted[0].parse()?;
        let bottom = splitted[1].parse()?;
        let left = splitted[2].parse()?;
        let right = splitted[3].parse()?;

        Ok(Self {
            top,
            bottom,
            left,
            right,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn load_arial() {

        let path = Path::new("./assets/fonts/Arial.fnt");

        let font = Font::load_fnt_font(&path).unwrap();

        assert_eq!(font.page.chars.len(), 191);

    }
}



#[derive(Debug)]
pub enum ParseFontError {
    GeneralError,
    IntParseError(std::num::ParseIntError),
    FloatParseError(std::num::ParseFloatError),
    BoolParseError(std::str::ParseBoolError),
    NoPagesError,
    ParsePageError(String),
    PathHasNotParent,
}

impl fmt::Display for ParseFontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}

impl Error for ParseFontError {}

impl From<std::num::ParseIntError> for ParseFontError {

    fn from(other: std::num::ParseIntError) -> Self {
        ParseFontError::IntParseError(other)
    }
}


impl From<std::num::ParseFloatError> for ParseFontError {
    fn from(other: std::num::ParseFloatError) -> Self {
        ParseFontError::FloatParseError(other)
    }
}

impl From<std::str::ParseBoolError> for ParseFontError {

    fn from(other: std::str::ParseBoolError) -> Self {
        ParseFontError::BoolParseError(other)
    }
}

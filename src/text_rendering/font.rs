use std::path::Path;
use std::fs;
use std::error::Error;
use std::str::{FromStr};
use std::fmt;
use itertools::Itertools;
use image::io::Reader as ImageReader;


use crate::na;


#[derive(Debug)]
pub enum ParseFontError {
    GeneralError,
    IntParseError(std::num::ParseIntError),
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

impl From<std::str::ParseBoolError> for ParseFontError {

    fn from(other: std::str::ParseBoolError) -> Self {
        ParseFontError::BoolParseError(other)
    }
}



#[derive(Debug)]
pub struct Font {
    info: FontInfo,
    page: Page,
    image: image::RgbImage
}


impl Font {

    /// Assumes that the png file referred to in the font is located in the same directory as the .fnt file.
    pub fn load_fnt_font(fnt_path: &Path) -> Result<Font, Box<dyn Error>> {

        let text = fs::read_to_string(fnt_path)?;

        let mut lines = text.lines();

        let info_lines: Vec::<&str> = lines.take_while_ref(|l| !l.starts_with("page ")).collect();

        let info: FontInfo = info_lines.join(" ").parse()?;

        // The rest is page. Maybe assuming single page is an error;
        let page: Page = lines.collect::<Vec<&str>>().join("\n").parse()?;


        let parent = fnt_path.parent().ok_or(ParseFontError::PathHasNotParent)?;
        let img_path = parent.join(&page.info.file_name);

        let image = ImageReader::open(img_path)?.decode()?.into_rgb8();

        Ok(Font {
            info,
            page,
            image,
        })

    }
}


#[derive(Debug)]
struct FontInfo {
    spacing: na::Vector2::<i32>,
    face: String,
    size: i32,
    stretch_h: i32,
    padding: Padding,
    aa: i32,
    smooth: i32,
    line_height: i32,
    scale: Scale,
    pages: i32,
    packed: bool,
}

impl FontInfo {

    fn empty() -> Self {
        Self {
            spacing: na::Vector2::new(0,0),
            face: "".to_string(),
            size: 0,
            stretch_h: 0,
            line_height: 0,
            scale: Scale {
                w: 0,
                h: 0
            },
            pages: 0,
            aa: 0,
            smooth: 0,
            padding: Padding::empty(),
            packed: false,
        }
    }
}

impl FromStr for FontInfo  {
    type Err = ParseFontError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut input = s.to_string();
        let rest = input.split_off(5);
        let parts = rest.split(" ");

        let mut info = FontInfo::empty();

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
                _ => { }
            }


        }

        Ok(info)
    }
}

#[derive(Debug, Default)]
struct Page {
    info: PageInfo,
    chars: Vec::<PageChar>,
    kernings: Vec::<Kerning>
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
struct PageInfo {
    id: usize,
    file_name: String,
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

#[derive(Debug, Default)]
struct PageChar {
    id: usize,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,
    x_advance: i32,
    y_advance: i32,
    page_id: usize,
    channel: i32,
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
struct Kerning {
    first_id: usize,
    second_id: usize,
    amount: i32
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

#[derive(Debug)]
struct Scale {
    w: i32,
    h: i32
}


#[derive(Debug)]
struct Padding {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}


impl Padding {
    fn empty() -> Self {
        Self {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }
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

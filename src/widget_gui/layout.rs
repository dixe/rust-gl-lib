use crate::widget_gui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutGeometry {
    pub width: LayoutLength,
    pub height: LayoutLength,
    pub pos: Position

}

impl LayoutGeometry {
    pub fn new(width: LayoutLength, height: LayoutLength) -> Self {
        Self {
            width,
            height,
            pos: Default::default(),
        }

    }

    pub fn from_flex(&self, flex_dir: FlexDir) -> LayoutLength {
        match flex_dir {
            FlexDir::X => self.width,
            FlexDir::Y => self.width
        }
    }
}

#[derive( Debug, Clone, Copy, PartialEq)]
pub enum LayoutLength {
    Fixed(Pixel),
    Fill(u8)
}

impl LayoutLength {

    pub fn pixels(&self, flex_info: &FlexInfo) -> Pixel {
        match self {
            LayoutLength::Fixed(px) => *px,
            LayoutLength::Fill(factor) => flex_info.space_per_flex * Pixel::from(*factor)
        }
    }
}



#[derive(Debug)]
pub struct FlexInfo {
    pub space_per_flex: Pixel,
    pub sum_flex_factor: Pixel,
}

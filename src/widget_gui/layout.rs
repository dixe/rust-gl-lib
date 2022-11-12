use crate::widget_gui::*;

pub enum LayoutResult {
    Size(Size),
    RequestChild(Id, BoxContraint)
}


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub pixel_w: Pixel,
    pub pixel_h: Pixel,
}

impl Size {

    pub fn from_flex(&self, flex_dir: FlexDir) -> Pixel {
        match flex_dir {
            FlexDir::X => self.pixel_w,
            FlexDir::Y => self.pixel_h
        }
    }
}


#[derive(Debug)]
pub struct FlexInfo {
    pub space_per_flex: Pixel,
    pub sum_flex_factor: Pixel,
}



#[derive(Debug, Clone, Copy, Default)]
pub struct Alignment {
    pub x: AlignmentX,
    pub y: AlignmentY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignmentX {
    Left,
    Right,
    Center
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignmentY {
    Top,
    Bottom,
    Center
}

impl Default for AlignmentX {
    fn default() -> Self {
        AlignmentX::Left
    }
}

impl Default for AlignmentY {
    fn default() -> Self {
        AlignmentY::Top
    }
}

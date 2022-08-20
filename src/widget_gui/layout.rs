use crate::widget_gui::*;

pub enum LayoutResult {
    Size(Size),
    RequestChild(Id, BoxContraint)
}



#[derive(Debug)]
pub struct FlexInfo {
    pub space_per_flex: Pixel,
    pub sum_flex_factor: Pixel,
}

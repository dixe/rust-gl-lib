use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SheetArrayAnimation {
    pub frames: Vec::<ArrayFrame>,
    pub meta: Meta
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ArrayFrame {
    pub filename: String,
    pub frame: FrameSprite,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: SourceSize,
    pub duration: f64
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Size {
    pub w: i32,
    pub h: i32
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FrameSprite {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
 }


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SourceSize {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub image: String,
    pub size: Size
}

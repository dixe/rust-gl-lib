use crate::general_animation::Animatable;
use crate::texture::TextureId;
use crate::general_animation::Animation;
use crate::na;


type V2 = na::Vector2::<f32>;

#[derive(Debug)]
pub struct SheetAnimation {
    pub texture_id: TextureId,
    pub animation: Animation<Sprite>,
    pub size: V2,
}


#[derive(Debug, Clone, Copy)]
pub struct Sprite
{
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}


impl Animatable for Sprite {

    // pixel art spritesheet animation don't interpolat between frames
    fn lerp(a: &Self, _b: &Self, _t: f32) -> Self {
        *a
    }

}


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

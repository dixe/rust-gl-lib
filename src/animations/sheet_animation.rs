use crate::texture::{self, TextureId};
use crate::general_animation::{Animation, Animatable, Frame};
use crate::{gl, na};
use crate::imode_gui::drawer2d::{Drawer2D, SheetSubSprite};
use crate::imode_gui::ui::Ui;
use crate::math::numeric::Numeric;
use std::collections::HashMap;
use crate::math::{AsV2, AsV2i};


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


pub struct SheetAnimationPlayer<'a> {
    animations: HashMap::<usize, ActiveAnimation<'a>>,
    next_id: usize,
    clear_buffer: Vec::<usize>
}


impl<'a> SheetAnimationPlayer<'a> {

    pub fn new() -> Self {
        Self {
            animations: Default::default(),
            next_id: 1,
            clear_buffer: vec![]
        }
    }

    pub fn start(&mut self, sheet_anim: &'a SheetAnimation, scale: f32, repeat: bool) -> usize {
        let id = self.next_id;

        self.animations.insert(id,
                               ActiveAnimation {
                                   sheet: sheet_anim,
                                   repeat,
                                   elapsed: 0.0,
                                   sprite: sheet_anim.animation.frame(0),
                                   scale,
                               });

        self.next_id += 1;
        id
    }


    pub fn update(&mut self, dt: f32) {

        self.clear_buffer.clear();

        for (id,anim) in &mut self.animations {
            anim.elapsed += dt;

            if let Some(s) = anim.sheet.animation.at(anim.elapsed) {
                anim.sprite = s;
            } else {
                if !anim.repeat {
                    self.clear_buffer.push(*id);
                }
                else {
                    anim.elapsed = 0.0;
                    anim.sheet.animation.at(anim.elapsed);
                }
            }
        }

        for id in &self.clear_buffer {
            self.animations.remove(id);
        }
    }

    pub fn remove(&mut self, id: usize) {
        self.animations.remove(&id);
    }

    pub fn expired(&self, id: usize) -> bool {
        self.animations.contains_key(&id)
    }

    pub fn draw<T : Numeric + std::fmt::Debug>(&mut self, drawer2D: &mut Drawer2D, pos: na::Vector2::<T>, anim_id: usize) {
        if let Some(anim) = self.animations.get(&anim_id) {


            let sprite = SheetSubSprite {
                sheet_size: anim.sheet.size,
                pixel_l: anim.sprite.x,
                pixel_r: anim.sprite.x + anim.sprite.w,
                pixel_b: anim.sprite.y,
                pixel_t: anim.sprite.y + anim.sprite.h,
            };

            let size = na::Vector2::new(anim.sprite.w, anim.sprite.h).v2() * anim.scale;


            let p = pos.v2i();
            drawer2D.render_sprite_sheet_frame(anim.sheet.texture_id, p.x, p.y, size, &sprite);
        }
    }
}


/// Frame to polygons map. Name to polygon, fx body, attack, ect.
//#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub type SheetCollisionPolygons = HashMap::<usize, HashMap::<String, crate::collision2d::polygon::Polygon>>;


struct ActiveAnimation<'a> {
    sheet: &'a SheetAnimation,
    repeat: bool,
    elapsed: f32,
    scale: f32,
    sprite: Sprite
}


pub fn load_by_name(gl: &gl::Gl, path: &std::path::PathBuf, id: &mut usize) -> SheetAnimation {

    let anim_json = std::fs::read_to_string(path);
    let sheet_anim : SheetArrayAnimation = match anim_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            panic!("Error loading json file \n{:?}", err);
        }
    };

    let size = na::Vector2::new(sheet_anim.meta.size.w as f32, (sheet_anim.meta.size.h /2) as f32);

    let mut base_path = path.clone();

    base_path.pop();
    base_path.push(&sheet_anim.meta.image);

    let img = image::open(&base_path).unwrap().into_rgba8();

    let aspect = img.height() as f32 / img.width() as f32;
    let texture_id = texture::gen_texture_rgba_nearest(gl, &img);

    let mut frames = vec![];


    for frame in &sheet_anim.frames {

        frames.push(Frame::<Sprite> {
            data: Sprite
            {
                x: frame.frame.x,
                y: frame.frame.y,
                w: frame.frame.w,
                h: frame.frame.h,
            },
            frame_seconds: frame.duration as f32 / 1000.0

        });
    }

    let anim = SheetAnimation {
        texture_id,
        size: na::Vector2::new(sheet_anim.meta.size.w as f32, sheet_anim.meta.size.h as f32),
        animation: Animation { frames },
    };

    *id += 1;
    anim
}
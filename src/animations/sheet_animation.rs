use std::path::{Path, PathBuf};
use failure;
use crate::texture::{self, TextureId};
use crate::general_animation::{Animation, Animatable, Frame};
use crate::{gl, na};
use crate::imode_gui::drawer2d::{Drawer2D, SheetSubSprite};
use crate::imode_gui::ui::Ui;
use crate::math::numeric::Numeric;
use std::collections::HashMap;
use crate::math::{AsV2, AsV2i};
use crate::collision2d::polygon::Polygon;



pub type AnimationId = usize;
type V2 = na::Vector2::<f32>;

#[derive(Debug)]
pub struct SheetAnimation {
    pub texture_id: TextureId,
    pub name: String,
    pub animation: Animation<Sprite>,
    pub collision_polygons: SheetCollisionPolygons,
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
    animations: HashMap::<AnimationId, ActiveAnimation<'a>>,
    next_id: AnimationId,
    clear_buffer: Vec::<AnimationId>
}


impl<'a> SheetAnimationPlayer<'a> {

    pub fn new() -> Self {
        Self {
            animations: Default::default(),
            next_id: 1,
            clear_buffer: vec![]
        }
    }

    pub fn get_polygon(&self, anim_id: AnimationId, name: &str) -> Option<(&Polygon, f32)> {
        if let Some(active) = self.animations.get(&anim_id) {
            if let Some(map) = active.sheet.collision_polygons.get(&active.frame) {
                return map.get(name).map(|p| (p, active.scale));
            }
        }

        None
    }

    pub fn start(&mut self, sheet_anim: &'a SheetAnimation, scale: f32, repeat: bool) -> AnimationId {
        let id = self.next_id;

        self.animations.insert(id,
                               ActiveAnimation {
                                   sheet: sheet_anim,
                                   repeat,
                                   frame: 0,
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

            if let Some((s, frame)) = anim.sheet.animation.at(anim.elapsed) {
                anim.sprite = s;
                anim.frame = frame;
            } else {
                if !anim.repeat {
                    self.clear_buffer.push(*id);
                }
                else {
                    anim.elapsed = 0.0;
                }
            }
        }

        for id in &self.clear_buffer {
            self.animations.remove(id);
        }
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }

    pub fn expired(&self, id: AnimationId) -> bool {
        self.animations.contains_key(&id)
    }

    pub fn draw<T : Numeric + std::fmt::Debug>(&mut self, drawer2D: &mut Drawer2D, pos: na::Vector2::<T>, anim_id: AnimationId) {
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


pub fn load_sheet_collision_polygons<P: AsRef<Path> + std::fmt::Debug>(path: &P, name: &str) -> SheetCollisionPolygons {
    let mut p = PathBuf::new();
    p.push(path);
    p.push(format!("{name}_polygons.json"));

    let json = std::fs::read_to_string(&p);
    match json {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(data) => data,
                Err(err) => {
                    println!("Collision polyogn path '{:?}'", &p);
                    println!("Jsonfile in the wrong format. Creating default(empty) frame polygons\n{:?}", err);
                    Default::default()
                }
            }
        },
        Err(err) => {
            println!("Collision polyogn path '{:?}'", &p);
            println!("Error loading json file. Creating default(empty) frame polygons\n{:?}", err);
            Default::default()
        }
    }
}




struct ActiveAnimation<'a> {
    sheet: &'a SheetAnimation,
    repeat: bool,
    frame: usize,
    elapsed: f32,
    scale: f32,
    sprite: Sprite
}


pub fn load_by_name<P: AsRef<Path> + std::fmt::Debug>(gl: &gl::Gl, path: &P, name: &str, id: &mut usize) -> SheetAnimation {

    let mut p = PathBuf::new();
    p.push(path);
    p.push(format!("{name}.json"));

    let anim_json = std::fs::read_to_string(p);

    let sheet_anim : SheetArrayAnimation = match anim_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            panic!("Error loading json file \n{:?}", err);
        }
    };

    let size = na::Vector2::new(sheet_anim.meta.size.w as f32, (sheet_anim.meta.size.h /2) as f32);

    let mut base_path = PathBuf::new();

    base_path.push(path);

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


    let collision_polygons = load_sheet_collision_polygons(&path, name);

    let anim = SheetAnimation {
        texture_id,
        name: name.to_string(),
        collision_polygons,
        size: na::Vector2::new(sheet_anim.meta.size.w as f32, sheet_anim.meta.size.h as f32),
        animation: Animation { frames },
    };

    *id += 1;
    anim
}

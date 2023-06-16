#![allow(non_snake_case)]
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use failure;
use crate::texture::{self, TextureId};
use crate::general_animation::{Animation, Animatable, Frame};
use crate::{gl, na};
use crate::imode_gui::drawer2d::{Drawer2D, SheetSubSprite};

use crate::math::numeric::Numeric;
use std::collections::HashMap;
use crate::math::{AsV2, AsV2i};
use crate::collision2d::polygon::{self, Polygon, ComplexPolygon};
use crate::collision2d::gjk;
use crate::image::PreMulAlpha;


pub type AnimationId = usize;
type V2 = na::Vector2::<f32>;
pub type FrameData<FrameDataT> = HashMap::<usize, FrameDataT>;

#[derive(Debug, Clone)]
pub struct SheetAnimation<FrameDataT> {
    pub texture_id: TextureId,
    pub name: String,
    pub animation: Animation<Sprite>,
    pub collision_polygons: ProcessedSheetCollisionPolygons,
    pub frame_data: FrameData<FrameDataT>,
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
    pub meta: Meta,
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
    pub size: Size,
    pub frameTags: Vec::<FrameTag>,
    pub layers: Vec::<Layer>
}



#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Layer {
    #[serde(default)]
    pub cels: Vec::<Cel>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Cel {
    pub frame: usize,
    pub data: String
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize
}

pub struct SheetAnimationPlayer<'a, FrameDataT> {
    animations: HashMap::<AnimationId, ActiveAnimation<'a, FrameDataT>>,
    next_id: AnimationId,
    clear_buffer: Vec::<AnimationId>
}

pub struct Start<'a, FrameDataT> {
    pub sheet: &'a SheetAnimation<FrameDataT>,
    pub scale: f32,
    pub repeat: bool,
    pub flip_y: bool,
}

impl<'a, FrameDataT> SheetAnimationPlayer<'a, FrameDataT> {

    pub fn new() -> Self {
        Self {
            animations: Default::default(),
            next_id: 1,
            clear_buffer: vec![]
        }
    }


    pub fn get_polygon_map(&self, anim_id: AnimationId) -> Option::<&std::collections::hash_map::HashMap<std::string::String, SheetCollisionPolygon>> {
        if let Some(active) = self.animations.get(&anim_id) {
            return active.sheet.collision_polygons.get(&active.frame);
        }
        None
    }

    pub fn get_framedata(&self, anim_id: AnimationId) -> Option::<&FrameDataT> {
        if let Some(active) = self.animations.get(&anim_id) {
            return active.sheet.frame_data.get(&active.frame);
        }
        None
    }

    pub fn get_polygon(&self, anim_id: AnimationId, name: &str) -> Option<(&SheetCollisionPolygon, f32, bool)> {
        if let Some(active) = self.animations.get(&anim_id) {
            if let Some(map) = active.sheet.collision_polygons.get(&active.frame) {
                return map.get(name).map(|p| (p, active.scale, active.flip_y));
            }
        }

        None
    }


    pub fn frame(&self, anim_id: AnimationId) -> Option<usize> {

        if let Some(active) = self.animations.get(&anim_id) {
            return Some(active.frame);
        }
        None
    }

    pub fn flip_y(&mut self, anim_id: AnimationId, flip_y: bool) {
        if let Some(active) = self.animations.get_mut(&anim_id) {
            active.flip_y = flip_y;
        }
    }

    pub fn start(&mut self, start: Start<'a, FrameDataT>) -> AnimationId {
        let id = self.next_id;

        self.animations.insert(id,
                               ActiveAnimation {
                                   sheet: start.sheet,
                                   repeat: start.repeat,
                                   frame: 0,
                                   elapsed: 0.0,
                                   sprite: start.sheet.animation.frame(0),
                                   scale: start.scale,
                                   flip_y: start.flip_y,
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
        !self.animations.contains_key(&id)
    }

    pub fn draw<T : Numeric + std::fmt::Debug>(&self, drawer_2d: &mut Drawer2D, pos: na::Vector2::<T>, anim_id: AnimationId) {
        if let Some(anim) = self.animations.get(&anim_id) {


            let sprite = SheetSubSprite {
                sheet_size: anim.sheet.size,
                pixel_l: anim.sprite.x,
                pixel_r: anim.sprite.x + anim.sprite.w,
                pixel_b: anim.sprite.y,
                pixel_t: anim.sprite.y + anim.sprite.h,
                flip_y: anim.flip_y
            };

            let size = na::Vector2::new(anim.sprite.w, anim.sprite.h).v2() * anim.scale;


            let p = pos.v2i();
            drawer_2d.render_sprite_sheet_frame(anim.sheet.texture_id, p.x, p.y, size, &sprite);
        }
    }
}


/// Frame to polygons map. Name to polygon, fx body, attack, ect.
//#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub type SheetCollisionPolygons = HashMap::<usize, HashMap::<String, crate::collision2d::polygon::Polygon>>;

pub type ProcessedSheetCollisionPolygons = HashMap::<usize, HashMap::<String, SheetCollisionPolygon>>;


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
                    //println!("Collision polyogn path '{:?}'", &p);
                    //println!("Jsonfile in the wrong format. Creating default(empty) frame polygons\n{:?}", err);
                    Default::default()
                }
            }
        },
        Err(err) => {
            //println!("Collision polyogn path '{:?}'", &p);
            //println!("Error loading json file. Creating default(empty) frame polygons\n{:?}", err);
            Default::default()
        }
    }
}




struct ActiveAnimation<'a, FrameDataT> {
    sheet: &'a SheetAnimation<FrameDataT>,
    repeat: bool,
    frame: usize,
    elapsed: f32,
    scale: f32,
    flip_y: bool,
    sprite: Sprite
}


pub fn load_by_name<P: AsRef<Path> + std::fmt::Debug, FrameDataT>(gl: &gl::Gl, json_path: &P, file_name: &str, id: &mut usize, data_map: fn(&str) -> FrameDataT) -> Result<Vec::<SheetAnimation<FrameDataT>>, failure::Error> {

    let anim_json = std::fs::read_to_string(json_path)?;

    let sheet_anim : SheetArrayAnimation = serde_json::from_str(&anim_json)?;

    let _size = na::Vector2::new(sheet_anim.meta.size.w as f32, (sheet_anim.meta.size.h /2) as f32);

    let mut base_path = PathBuf::new();

    base_path.push(json_path);
    // remove file name
    base_path.pop();

    // add img filename
    base_path.push(&sheet_anim.meta.image);

    let mut img = image::open(&base_path)?.into_rgba8();

    //pre multiply alpha since open gl and shaders assume that;

    img.pre_multiply_alpha();

    let _aspect = img.height() as f32 / img.width() as f32;
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

    let mut user_data = HashMap::<usize, String>::new();

    for layer in &sheet_anim.meta.layers {
        for cel in &layer.cels {
            user_data.insert(cel.frame, cel.data.clone());
        }
    }


    // use frameTags as animationNames
    let mut animations = sheet_anim.meta.frameTags.clone();


    // if no tags, use file name
    if animations.len() == 0 {
        animations.push(FrameTag { name: file_name.to_string(), from: 0, to: sheet_anim.frames.len() - 1 });
    }

    let mut res = vec![];
    base_path.pop();
    for tag in &animations {
        let polygons = load_sheet_collision_polygons(&base_path, &tag.name);

        let mut collision_polygons : ProcessedSheetCollisionPolygons = Default::default();

        // make subdivisions for polygon and create new hashmap
        for (frame, map) in &polygons {
            let mut inner : HashMap::<String, SheetCollisionPolygon> = Default::default();

            for (polygon_name, polygon) in map {

                let mut new_poly = polygon.clone();
                let mut sub_divisions = vec![];

                for sub in polygon::calculate_subdivision(&mut new_poly) {
                    sub_divisions.push(sub.indices);
                }

                inner.insert(polygon_name.clone(), SheetCollisionPolygon {
                    polygon: new_poly,
                    sub_divisions
                });

            }

            collision_polygons.insert(*frame, inner);
        }



        let mut frame_data : FrameData<FrameDataT> = Default::default();

        for frame in tag.from..=tag.to {
            if let Some(data) = user_data.get(&frame) {
                frame_data.insert(frame - tag.from, data_map(&data));
            }
        }

        let anim = SheetAnimation {
            texture_id,
            name: tag.name.clone(),
            collision_polygons,
            size: na::Vector2::new(sheet_anim.meta.size.w as f32, sheet_anim.meta.size.h as f32),
            animation: Animation { frames: frames[tag.from..=tag.to].iter().map(|f| (*f).clone()).collect() },
            frame_data
        };

        *id += 1;
        res.push(anim);
    }

    Ok(res)

}

#[derive(Default, Debug, Clone)]
pub struct SheetCollisionPolygon {
    pub polygon: Polygon,
    sub_divisions: Vec::<Vec::<usize>>,
}


impl SheetCollisionPolygon {


    pub fn collide(&self, transform: &na::Matrix3::<f32>, other: &SheetCollisionPolygon, transform_other: &na::Matrix3::<f32>) -> bool {
        let _res = false;
        for indices_1 in &self.sub_divisions {
            let sub_p_1 = ComplexPolygon {
                polygon: &self.polygon,
                indices: &indices_1,
                transform
            };

            for indices_2 in &other.sub_divisions {
                let sub_p_2 = ComplexPolygon {
                    polygon: &other.polygon,
                    indices: &indices_2,
                    transform: transform_other
                };

                let collision = gjk::gjk_intersection(&sub_p_1, &sub_p_2);
                if collision {
                    return true;
                }
            }
        }

        false
    }


    pub fn collide_draw(&self, drawer2d: &mut Drawer2D, transform: &na::Matrix3::<f32>, other: &SheetCollisionPolygon, transform_other: &na::Matrix3::<f32>) -> bool {
        let _res = false;

        for indices_1 in &self.sub_divisions {
            let sub_p_1 = ComplexPolygon {
                polygon: &self.polygon,
                indices: &indices_1,
                transform
            };

            for indices_2 in &other.sub_divisions {
                let sub_p_2 = ComplexPolygon {
                    polygon: &other.polygon,
                    indices: &indices_2,
                    transform: transform_other
                };


                let collision = gjk::gjk_intersection(&sub_p_1, &sub_p_2);

                if collision {
                    drawer2d.convex_polygon(&sub_p_1);
                    drawer2d.convex_polygon(&sub_p_2);
                    return true;
                }
            }
        }

        false
    }
}




pub type SheetAssets<FrameDataT>= std::collections::HashMap::<String, std::collections::HashMap::<String, SheetAnimation<FrameDataT>>>;


pub fn load_folder<P: AsRef<Path> + std::fmt::Debug, FrameDataT>(gl: &gl::Gl, path: &P, data_map: fn(&str) -> FrameDataT) -> SheetAssets<FrameDataT> {

    let mut id = 0;
    let _dir = match std::fs::read_dir(&path) {
        Ok(d) => d,
        Err(err) => {
            println!("Path was '{:?}'", &path);
            panic!("{}",err);
        }
    };

    let mut res = SheetAssets::default();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
            let file_name = entry.file_name().to_str().unwrap();
            let file_name_no_ending = file_name.split(".").next().unwrap().to_string();
            if file_name.ends_with(".json") {
                let _json_names = match load_by_name(gl, &entry.path(), &file_name_no_ending, &mut id, data_map) {
                    Ok(mut sheet_anims) => {
                        let mut pb = std::path::PathBuf::new();
                        pb.push(entry.path());
                        pb.pop();
                        let dir_name = pb.file_name().unwrap().to_str().unwrap().to_string();
                        if !res.contains_key(&dir_name) {
                            res.insert(dir_name.clone(), Default::default());
                        }

                        let map = res.get_mut(&dir_name).unwrap();
                        while let Some(sheet) = sheet_anims.pop() {
                            map.insert(sheet.name.clone(), sheet);

                        }
                    },
                    Err(_e) => {

                    }
                };
            }
        }

    res
}

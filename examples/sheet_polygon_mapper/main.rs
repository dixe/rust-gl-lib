use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::{*};
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::PolygonOptions;
use gl_lib::collision2d::polygon::{Polygon, Dir, self};
use gl_lib::texture::TextureId;
use gl_lib::typedef::*;

use gl_lib::animations::sheet_animation::{self, SheetCollisionPolygons, Sprite, SheetAnimation, load_folder};
use std::path::{Path, PathBuf};
use gl_lib::math::*;
use std::collections::HashMap;
use itertools::Itertools;

fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let path : String = "examples/pixel_sekiro/assets/".to_string();
    let mut assets = load_folder(&gl, &path, |s| s.to_string());

    let mut state = State::Selecting;

    let mut copy : Option<Polygon> = None;

    let mut new_name = "".to_string();
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui);
        match state {
            State::Selecting => {
                for (folder_name, map) in assets.iter().sorted_by_key(|x| x.0) {
                    ui.label(folder_name);
                    let mut p = PathBuf::new();
                    p.push(&path);
                    p.push(folder_name);
                    for (name, asset) in map.iter().sorted_by_key(|x| x.0) {
                        if ui.button(name) {
                            state = State::Edit(Edit {
                                sheet: create_sheet_edit(&p, name, asset),
                                name: "body".to_owned(),
                                folder: folder_name.to_string(),
                                frame: 0
                            });
                        }
                    }
                    ui.newline();
                }

                ui.newline();
                if ui.button("Reload") {
                    assets = load_folder(&gl, &path, |s| s.to_string());
                }

            },
            State::Edit(ref mut edit) => {

                for i in 0..edit.sheet.frames.len() {
                    if edit.frame == i {
                        ui.body_text(&format!("Edit frame {i}"));
                    }
                    else {
                        if ui.button(&format!("Edit frame {i}")) {
                            edit.frame = i;
                        }
                    }
                }

                ui.newline();

                ui.label("Scale");
                ui.slider(&mut edit.sheet.scale, 1.0, 30.0);

                let ret = ui.button("Back");

                for i in 0..edit.sheet.frames.len() {
                    edit.sheet.frames[i].options.transform.scale = edit.sheet.scale;
                }

                ui.newline();

                if ui.button("Save All Frames") {
                    clean(&mut edit.sheet);
                    right_align(&mut edit.sheet);
                    save_all_frames(&path, &edit);

                }

                if ui.button("Copy") {
                    copy = Some(edit.sheet.frames[edit.frame].polygons.get(&edit.name).unwrap().clone());
                }

                if ui.button("Replace") {

                    if let Some(ref c) = copy {
                        let p = edit.sheet.frames[edit.frame].polygons.get_mut(&edit.name).unwrap();
                        *p = c.clone()
                    }
                }

                ui.newline();

                for name in edit.sheet.frames[edit.frame].polygons.keys() {
                    if name == &edit.name {
                        ui.body_text(&name);
                    } else {
                        if ui.button(name) {
                            edit.name = name.clone();
                        }
                    }
                }

                ui.textbox(&mut new_name);
                if ui.button("Add") {
                    for i in 0..edit.sheet.frames.len() {
                        if !edit.sheet.frames[i].polygons.contains_key(&new_name) {
                            edit.sheet.frames[i].polygons.insert(new_name.clone(), Default::default());

                        }
                    }
                    new_name = "".to_string();
                }

                ui.newline();
                if ui.button("Reset") {
                    let p = edit.sheet.frames[edit.frame].polygons.get_mut(&edit.name).unwrap();
                    *p = Polygon::default();
                }

                if ui.button("Calc sub") {
                    let p = edit.sheet.frames[edit.frame].polygons.get_mut(&edit.name).unwrap();
                    polygon::calculate_subdivision(p);
                    println!("{:?}", p.direction());
                }

                ui.newline();
                ui.body_text("Frame data: ");
                ui.textbox(&mut edit.sheet.frames[edit.frame].user_data);


                img_edit(&mut ui, &mut edit.sheet.frames[edit.frame], &edit.name, edit.sheet.size, edit.sheet.texture_id);

                if ret {
                    state = State::Selecting;
                }
            }
        }

        window.gl_swap_window();

    }
}


fn right_align(sheet: &mut SheetEdit) {
    for i in 0..sheet.frames.len() {
        for p in sheet.frames[i].polygons.values_mut() {
            if p.direction() == Dir::Left {
                p.vertices.reverse();
            }
        }
    }
}

fn clean(sheet: &mut SheetEdit) {

    let mut remove = vec![];
    for i in 0..sheet.frames.len() {
        for (key, val) in &sheet.frames[i].polygons {
            if val.vertices.len() == 0 {
                remove.push((i, key.clone()));
            }
        }
    }

    for (i, key) in &remove {
        sheet.frames[*i].polygons.remove(&*key);
    }
}


fn save_all_frames<P: AsRef<Path>>(p: &P, edit: &Edit) {

    let mut path = PathBuf::new();
    path.push(p);
    path.push(&edit.folder);
    path.push(&format!("{}_polygons.json", &edit.sheet.name));

    println!("{:?}", &path);
    let mut data = SheetCollisionPolygons::default();

    for i in 0..edit.sheet.frames.len() {
        data.insert(i, edit.sheet.frames[i].polygons.clone());
    }

    match serde_json::to_string(&data) {
        Ok(json) => {
            std::fs::write(path, json);
        },
        Err(err) => {
            println!("Fail to save\n{:?}", err);
        }
    }
}


fn create_sheet_edit<P: AsRef<Path> + std::fmt::Debug>(path: &P, name: &str, sheet: &SheetAnimation<String>) -> SheetEdit {

    let mut frames : Vec::<FrameEdit> = vec![];
    let polygons = sheet_animation::load_sheet_collision_polygons(&path, name);

    let mut f = 0;

    for frame in &sheet.animation.frames {
        // get body polygon for current frame, or default empty if not found
        let map = match polygons.get(&f) {
            Some(m) => m.clone(),
            None => Default::default()
        };

        f +=1;
        frames.push(FrameEdit {
            polygons: map.clone(),
            anchor: V2i::new(400, 600),
            options: Default::default(),
            sprite: frame.data,
            user_data: sheet.frame_data.get(&f).map(|s| s.to_string()).unwrap_or_default()
        });

    }

    SheetEdit {
        name: name.to_string(),
        size: sheet.size,
        texture_id: sheet.texture_id,
        scale: 10.0,
        frames
    }
}

enum State {
    Selecting,
    Edit(Edit)
}

struct Edit {
    sheet: SheetEdit,
    name: String,
    folder: String,
    frame: usize
}

struct SheetEdit {
    texture_id: TextureId,
    size: V2,
    name: String,
    scale: f32,
    frames: Vec::<FrameEdit>,
}


struct FrameEdit {
    polygons: HashMap::<String, Polygon>,
    anchor: V2i,
    options: PolygonOptions,
    sprite: Sprite,
    user_data: String
}


fn img_edit(ui: &mut Ui, edit: &mut FrameEdit, poly_name: &str, sheet_size: V2, texture_id: TextureId) {


    let scale = edit.options.transform.scale;

    let base_size = V2i::new(edit.sprite.w, edit.sprite.h).v2();



    let sprite = SheetSubSprite {
        sheet_size: sheet_size,
        pixel_l: edit.sprite.x,
        pixel_r: edit.sprite.x + edit.sprite.w,
        pixel_b: edit.sprite.y,
        pixel_t: edit.sprite.y + edit.sprite.h,
        flip_y: false
    };


    ui.drawer2D.render_sprite_sheet_frame(texture_id, edit.anchor.x, edit.anchor.y, base_size * scale, &sprite);

    ui.drag_point(&mut edit.anchor, 10.0);

    edit.options.transform.translation.x = edit.anchor.x as f32;
    edit.options.transform.translation.y = edit.anchor.y as f32;


    let mut output_offset = edit.anchor;
    output_offset.x -= 40;
    output_offset.y += 30;


    let _sprite = SheetSubSprite {
        sheet_size: sheet_size,
        pixel_l: edit.sprite.x,
        pixel_r: edit.sprite.x + edit.sprite.w,
        pixel_b: edit.sprite.y,
        pixel_t: edit.sprite.y + edit.sprite.h,
        flip_y: false
    };

    // draw "center" of polygon on base image with anchor = output_offset
    //TODO:

    if !edit.polygons.contains_key(poly_name) {
        edit.polygons.insert(poly_name.to_string(), Default::default());
    }

    let polygon = edit.polygons.get_mut(poly_name).unwrap();

    ui.polygon_editor(polygon, &mut edit.options);

    // draw ""Correct" polygon, with translation offset
    let mut transform = edit.options.transform;

    transform.translation = output_offset.to_v2();
    transform.scale = 1.0;
    ui.view_polygon(polygon, &transform);

}

trait ToV2 {
    fn to_v2(&self) -> V2;
}

impl ToV2 for V2i {
    fn to_v2(&self) -> V2 {
        V2::new(self.x as f32, self.y as f32)
    }

}


fn screen_to_img_coords(mut v: V2, anchor: V2i, scale: f32) -> V2{
    v.x -= anchor.x as f32;
    v.y -= anchor.y as f32;

    v *= 1.0/scale;
    v
}

fn handle_inputs(ui: &mut Ui) {

    for e in &ui.frame_events {
        match e {
            _ => {}

        }
    }
}

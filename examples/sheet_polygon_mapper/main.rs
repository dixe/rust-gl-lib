use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::{*};
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::PolygonOptions;
use gl_lib::collision2d::polygon::{Polygon};
use gl_lib::texture::TextureId;
use gl_lib::typedef::*;
use gl_lib_proc::sheet_assets;
use gl_lib::animations::sheet_animation::{SheetCollisionPolygons, Sprite, SheetAnimation};
use std::path::{Path, PathBuf};
use gl_lib::math::*;
use std::collections::HashMap;

// generate assets struct
sheet_assets!{Assets "examples/2d_animation_player/assets/"}



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

    let path =  "examples/2d_animation_player/assets/";
    let assets = Assets::load_all(&gl, path);

    let mut state = State::Selecting;

    let mut copy = None;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui);
        match state {
            State::Selecting => {
                if ui.button("idle") {
                    state = State::Edit(Edit {
                        sheet: create_sheet_edit(path, "idle".to_string(), &assets.idle),
                        frame: 0
                    });
                }

                if ui.button("attack") {
                    state = State::Edit(Edit {
                        sheet: create_sheet_edit(path, "attack".to_string(), &assets.idle),
                        frame: 0
                    });
                }


            },
            State::Edit(ref mut edit) => {

                for i in 0..edit.sheet.frames.len() {
                    if ui.button(&format!("Edit frame {i}")) {
                        edit.frame = i;
                    }
                }

                ui.label("Scale");
                ui.slider(&mut edit.sheet.scale, 1.0, 30.0);


                for i in 0..edit.sheet.frames.len() {
                    edit.sheet.frames[i].options.transform.scale = edit.sheet.scale;
                }

                ui.newline();
                if ui.button("Save") {
                    match serde_json::to_string(&edit.sheet.frames[edit.frame].polygon) {
                        Ok(json) => {

                            save(edit, edit.frame);
                        },
                        Err(err) => {
                            println!("Fail to save\n{:?}", err);
                        }
                    }
                }

                if ui.button("Save All") {
                    save_all("examples/2d_animation_player/assets/", &edit);

                }

                if ui.button("Copy") {
                    copy = Some(edit.sheet.frames[edit.frame].polygon.clone());
                }

                if ui.button("Replace") {

                    if let Some(ref c) = copy {
                        edit.sheet.frames[edit.frame].polygon = c.clone()
                    }
                }

                img_edit(&mut ui, &mut edit.sheet.frames[edit.frame],  edit.sheet.size, edit.sheet.texture_id);

            }
        }

        window.gl_swap_window();

    }
}


fn save_all(path_s: &str, edit: &Edit) {

    let mut path = PathBuf::new();
    path.push(path_s);
    path.push(&format!("{}_polygons.json", &edit.sheet.name));

    let mut data = SheetCollisionPolygons::default();


    for i in 0..edit.sheet.frames.len() {
        let mut frame_data : HashMap::<String, Polygon> = Default::default();

        if edit.sheet.frames[i].polygon.vertices.len() > 0 {
            frame_data.insert("body".to_string(), edit.sheet.frames[i].polygon.clone());
        }

        data.insert(i, frame_data);
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


fn load_all<P: AsRef<Path> + std::fmt::Debug>(path: &P) -> SheetCollisionPolygons {
    let json = std::fs::read_to_string(path);
    match json {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(data) => data,
                Err(err) => {
                    println!("{:?}", &path);
                    println!("Jsonfile in the wrong format. Creating default(empty) frame polygons\n{:?}", err);
                    Default::default()
                }
            }
        },
        Err(err) => {
            println!("{:?}", &path);
            println!("Error loading json file. Creating default(empty) frame polygons\n{:?}", err);
            Default::default()
        }
    }
}

fn save(edit: &Edit, frame: usize) {
    match serde_json::to_string(&edit.sheet.frames[frame].polygon) {
        Ok(json) => {
            std::fs::write(&format!("examples/2d_animation_player/assets/{}_{}.json", &edit.sheet.name, frame), json);
        },
        Err(err) => {
            println!("Fail to save\n{:?}", err);
        }
    }
}


fn create_sheet_edit(path_s: &str, name: String, sheet: &SheetAnimation) -> SheetEdit {

    let mut path = PathBuf::new();
    path.push(path_s);
    path.push(&format!("{}_polygons.json", &name));

    let mut frames : Vec::<FrameEdit> = vec![];

    let polygons = load_all(&path);

    let mut f = 0;
    /*
    for polygon_map in &polygons.frame_polygons {
        let polygon = polygon_map.get("body").unwrap_or_default();

        frames.push(FrameEdit {
            polygon,
            anchor: V2i::new(400, 600),
            options: Default::default(),
            sprite: frame.data
        });
    }*/

    for frame in &sheet.animation.frames {
/*

        let vertices_json = std::fs::read_to_string(&format!("examples/2d_animation_player/assets/{}_{}.json", name, f));
        let polygon = match vertices_json {
            Ok(json) => {
                serde_json::from_str(&json).unwrap()
            },
            Err(err) => {
                println!("Error loading json file, creating default polygon \n{:?}", err);
                Polygon {
                    vertices: vec![]
                }
            }
        };
*/
        // get body polygon for current frame, or default empty if not found
        let map = match polygons.get(&f) {
            Some(m) => m.clone(),
            None => Default::default()
        };

        let polygon = match map.get("body") {
            Some(p) => p.clone(),
            None => Default::default()
        };

        f +=1;
        frames.push(FrameEdit {
            polygon: polygon.clone(),
            anchor: V2i::new(400, 600),
            options: Default::default(),
            sprite: frame.data
        });

    }

    SheetEdit {
        name: name,
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
    polygon: Polygon,
    anchor: V2i,
    options: PolygonOptions,
    sprite: Sprite
}

/*

fn load(ui: &mut Ui) -> FrameEdit {

    // TODO: Load from path, maybe make this changeable
    let img = image::open("examples/top_rpg/assets/Sword.png").unwrap().into_rgba8();

    let texture_id = ui.register_image_nearest(&img);


    let base_size = V2::new(img.width() as f32, img.height() as f32);

    let vertices_json = std::fs::read_to_string("examples/collision_polygon_mapper/Sword.json");


    let polygon = match vertices_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            println!("Error loading json file, creating default polygon \n{:?}", err);
            Polygon {
                vertices: vec![V2::new(32.0, 0.0),
                               V2::new(0.0, 32.0),
                               V2::new(32.0, 32.0),
                ]
            }
        }
    };

    let mut edit = FrameEdit {
        polygon,
        texture_id,
        base_size,
        anchor: na::Vector2::new(200, 100),
        name: "Target".to_string(),
        options: Default::default()
    };

    edit.options.transform.scale = 10.0;

    edit
}

*/
fn img_edit(ui: &mut Ui, edit: &mut FrameEdit, sheet_size: V2, texture_id: TextureId) {


    let scale = edit.options.transform.scale;

    let base_size = V2i::new(edit.sprite.w, edit.sprite.h).v2();



    let sprite = SheetSubSprite {
        sheet_size: sheet_size,
        pixel_l: edit.sprite.x,
        pixel_r: edit.sprite.x + edit.sprite.w,
        pixel_b: edit.sprite.y,
        pixel_t: edit.sprite.y + edit.sprite.h,
    };


    ui.drawer2D.render_sprite_sheet_frame(texture_id, edit.anchor.x, edit.anchor.y, base_size * scale, &sprite);

    ui.drag_point(&mut edit.anchor, 10.0);

    edit.options.transform.translation.x = edit.anchor.x as f32;
    edit.options.transform.translation.y = edit.anchor.y as f32;


    let mut output_offset = edit.anchor;
    output_offset.x -= 40;
    output_offset.y += 30;


    let sprite = SheetSubSprite {
        sheet_size: sheet_size,
        pixel_l: edit.sprite.x,
        pixel_r: edit.sprite.x + edit.sprite.w,
        pixel_b: edit.sprite.y,
        pixel_t: edit.sprite.y + edit.sprite.h,
    };

    // draw "center" of polygon on base image with anchor = output_offset
    //TODO:

    ui.polygon_editor(&mut edit.polygon, &mut edit.options);

    // draw ""Correct" polygon, with translation offset
    let mut transform = edit.options.transform;

    transform.translation = output_offset.to_v2();
    transform.scale = 1.0;
    ui.view_polygon(&mut edit.polygon, &transform);

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

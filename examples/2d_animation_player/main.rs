use gl_lib::{gl, helpers};

use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;


use gl_lib::animations::sheet_animation::{Start, SheetAnimationPlayer, load_folder};
use gl_lib::typedef::*;
use gl_lib::collision2d::polygon::{PolygonTransform};
use gl_lib::math::AsV2;

use itertools::Itertools;


/*
// generate assets struct
sheet_assets!{PlayerAssets "examples/2d_animation_player/assets/player/"}

sheet_assets!{SkeletonAssets "examples/2d_animation_player/assets/player/"}
*/


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let path = "examples/pixel_sekiro/assets/";
    let assets = load_folder(&gl, &path, |s| s.to_string());


    //let path = "examples/2d_animation_player/assets/player/";
    //let assets = PlayerAssets::load_all(&gl, path);

    let mut player = SheetAnimationPlayer::new();

    let mut pos = V2i::new(400, 600);

    let mut pos2 = V2i::new(500, 600);

    let mut playing = true;


    let mut scale = 4.0;
    let mut flip_y = false;

    let mut time_scale = 1.0;
    let _hits = 0;

    let mut cur_anim = assets.get("player").unwrap().get("idle").unwrap();

    let mut anim_id = 0;

    let mut show_col_pol = true;

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        ui.consume_events(&mut event_pump);
        let dt = ui.dt() * time_scale;;

        for (folder_name, map) in assets.iter().sorted_by_key(|x| x.0) {
            ui.label(folder_name);
            for (_name, asset) in map {
                if ui.button(&asset.name) {
                    player.remove(anim_id);
                    cur_anim = &asset;
                    anim_id = player.start(Start {sheet: &asset, scale, repeat: true, flip_y});
                }
            }
            ui.newline();
        }

        ui.newline();
        if ui.button("Reload") {
            //assets = Assets::load_all(&gl, path);
        }

        if ui.button("Play/Pause") {
            playing = !playing;
        }

        if ui.button("Flip y") {
            flip_y = !flip_y;
            player.remove(anim_id);
            anim_id = player.start(Start {sheet: &cur_anim, scale, repeat: true, flip_y});
        }

        ui.label("Show collision polygons");
        ui.checkbox(&mut show_col_pol);

        ui.label("Time scale");
        ui.slider(&mut time_scale, 0.1, 3.1);

        ui.label("scale");
        if ui.slider(&mut scale, 0.1, 6.0) {
            player.remove(anim_id);
            anim_id = player.start(Start {sheet: &cur_anim, scale, repeat: true, flip_y});
        }

        ui.drawer2D.z = 1.0;
        // drag animation to be where we want
        ui.drag_point(&mut pos, 10.0);

        ui.drag_point(&mut pos2, 10.0);
        ui.drawer2D.z = 0.0;

        // update animations

        if playing {
            player.update(dt);
        }


        // draw animation frame at locations
        player.draw(&mut ui.drawer2D, pos, anim_id);

        if show_col_pol {
            if let Some(keys) = player.get_polygon_map(anim_id) {
                for (_name, p) in keys {
                    let mut transform = PolygonTransform::default();
                    transform.scale = scale;
                    transform.translation = pos.v2();
                    transform.flip_y = flip_y;

                    ui.view_polygon(&p.polygon, &transform);

                }
            }
        }

        window.gl_swap_window();
    }
}

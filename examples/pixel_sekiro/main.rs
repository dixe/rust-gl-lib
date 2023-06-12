use gl_lib::{gl, helpers, na};
use gl_lib_proc::sheet_assets;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use gl_lib::general_animation::{Animation, Animatable, Frame};
use gl_lib::animations::sheet_animation::{load_folder, Start, SheetAnimation, Sprite, SheetAnimationPlayer, AnimationId};
use gl_lib::typedef::*;
use gl_lib::collision2d::polygon::{PolygonTransform, ComplexPolygon};
use gl_lib::math::AsV2;

mod inputs;

mod entity;
use entity::*;

mod scene;

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

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let player_assets = PlayerAssets::load_all(&gl, "examples/2d_animation_player/assets/");

    //let skeleton_assets = SkeletonAssets::load_all(&gl, "examples/2d_animation_player/assets/");

    let mut animation_player = SheetAnimationPlayer::new();


    let mut pos2 = V2i::new(500, 600);

    let mut playing = true;

    let scale = 4.0;

    let assets = load_folder(&gl, &"examples/2d_animation_player/assets/");

    let mut scene = scene::new(&player_assets, &mut animation_player, &assets);


    let mut time_scale = 1.0;
    let mut poly_name = "body".to_string();
    let mut hits = 0;

    let mut roll_speed = 150.0;

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        let dt = ui.dt() * time_scale;

        if playing {
            scene.animation_player.update(dt);
            scene.update(&mut ui, dt);
        }

        if ui.button("Play/Pause") {
            playing = !playing;
        }

        ui.label("Time scale");
        ui.slider(&mut time_scale, 0.1, 3.1);
        ui.small_text(&format!("{:?}", hits));

        ui.label("Show collision boxes");
        ui.checkbox(&mut scene.show_col_boxes);


        ui.newline();
        ui.label("Roll speed");
        ui.slider(&mut roll_speed,  0.0, 300.0);
        ui.small_text(&format!("{:?}", roll_speed));

        if ui.button("Add skeleton") {
            scene.add_enemy("skeleton", pos2.v2());
        }

        ui.drawer2D.z = 1.0;
        ui.drag_point(&mut pos2, 10.0);
        ui.drawer2D.z = 0.0;

        // draw animation frame at locations
        scene.draw(&mut ui.drawer2D);

        window.gl_swap_window();
    }
}







/*
struct EntityCollidable<'a> {
    entity_id: usize,
    team_id: usize,
    collision_polygon : ComplexPolygon<'a>,
    transform: na::Matrix3::<f32> // homogeneous 3d matrixt for transforming V2 in 2d
}

/// Find weapon/spell collision with hittable part of target
fn find_collision(attacks: &[EntityCollidable], targets: &[EntityCollidable]) {

    for attack in attacks {
        for target in targets {
            if attack.team_id != target.team_id {
                if collision(attack.collision_polygon, attack.transform,
                             target.collision_polygon, target.transform) {

                    //add collision to output
                }
            }
        }
    }
}

/// Method 1
/// store polygons on animaiton_sheet, each frame has a hashmap<string, polygon>, get_polygon takes animation_id, and a polygon_name,
///returns Option<&Polygon>
fn get_attack_polygon(animation_id: usize, player: &SheetAnimationPlayer) {
    // animaiton_sheet_player know which frame and wich animation
    let attack_polygon = player.get_polygon(animation_id, "attack");

    let body_polygon = player.get_polygon(animation_id, "body");

}
*/
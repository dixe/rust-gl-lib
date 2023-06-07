use gl_lib::{gl, helpers, na};
use gl_lib_proc::sheet_assets;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use gl_lib::general_animation::{Animation, Animatable, Frame};
use gl_lib::animations::sheet_animation::{SheetAnimation, Sprite, SheetAnimationPlayer};
use gl_lib::typedef::*;
use gl_lib::collision2d::polygon::ComplexPolygon;


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

    let assets = Assets::load_all(&gl, "examples/2d_animation_player/assets/");

    let mut player = SheetAnimationPlayer::new();

    let mut anim_id = player.start(&assets.attack, 3.0, true);

    let mut pos = V2i::new(300, 400);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        ui.consume_events(&mut event_pump);
        let dt = ui.dt();


        if ui.button("Idle") {
            player.remove(anim_id);
            anim_id = player.start(&assets.idle, 3.0, true)
        }


        if ui.button("Attack") {
            player.remove(anim_id);
            anim_id = player.start(&assets.attack, 3.0, true)
        }

        ui.drawer2D.z = 1.0;
        // drag animation to be where we want
        ui.drag_point(&mut pos, 10.0);
        ui.drawer2D.z = 0.0;

        // update animations
        player.update(dt);


        // draw animation frame at location
        player.draw(&mut ui.drawer2D, pos, anim_id);

        window.gl_swap_window();
    }
}



struct EntityCollidable<'a> {
    entity_id: usize,
    team_id: usize,
    collision_polygon : ComplexPolygon<'a>,
    transform: na::Matrix3::<f32> // homogeneous 3d matrixt for transforming V2 in 2d
}



/*
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

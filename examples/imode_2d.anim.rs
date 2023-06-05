use gl_lib::{gl, helpers, na};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use gl_lib::general_animation::{Animation, Animatable, Frame};
use gl_lib::animations::sheet_animation::{SheetAnimation, Sprite, SheetAnimationPlayer};



// generate assets struct
sheet_assets!(Assests "examples/2d_animation_player/assets/")


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

    let assets = Assets::load_all();

    let mut player = SheetAnimationPlayer::new(&assets);


    let anim_id = player.start(assets.attack.id);

    let mut pos = V2i::new(300, 400);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        ui.consume_events(&mut event_pump);

        delta_time.update();
        let dt = delta_time.time();

        // drag animation to be where we want
        ui.drag_point(&mut pos, 10.0);

        // update animations
        player.update(dt);


        // draw animation frame at location
        player.draw(pos, id);



        window.gl_swap_window();
    }
}

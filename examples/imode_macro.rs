use gl_lib::{self, gl, helpers, na};
use gl_lib_proc::sheet_assets;
use gl_lib::imode_gui::*;
use gl_lib::imode_gui::drawer2d::*;

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

    let p = "examples/2d_animation_player/assets/";

    let a = Assets::load_all(&mut ui, p);

    println!("{:?}", a);

    Ok(())
}

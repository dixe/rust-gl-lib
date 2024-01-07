use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;

use gl_lib::color::Color;


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    let mut event_pump = sdl.event_pump().unwrap();


     // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut w = 100.0;
    let mut h = 100.0;
    let mut r = 30.0;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.slider(&mut w, 10.0, 1000.0);
        ui.newline();

        ui.slider(&mut h, 10.0, 1000.0);
        ui.newline();

        ui.slider(&mut r, 0.0, 100.0);
        ui.newline();

        ui.label(&format!("{}", r));
        if ui.button("square") {
            w = h;
        }

        ui.newline();
        if ui.button("Reload shaders") {
            ui.drawer2D.reload_all_shaders();
        }

        ui.drawer2D.rounded_rect_color(300, 50, w, h, r, Color::RgbA(60, 60, 60, 255));

        window.gl_swap_window();
    }
}

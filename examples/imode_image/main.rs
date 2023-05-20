use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use deltatime;
use gl_lib::text_rendering::font::{Font, MsdfFont, FntFont};
use gl_lib::shader::BaseShader;
use image::imageops;

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

    let img = image::open("examples/imode_image/overbli1.jpg").unwrap().into_rgba8();;

    let texture_id = ui.register_image(&img);

    let mut size = na::Vector2::<i32>::new(100, 100);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.slider(&mut size.x, 10, 1000);
        size.y = size.x;

        ui.newline();

        ui.image(texture_id, size);

        ui.image(texture_id, size * 2);

        ui.image(texture_id, size * 4);
        ui.image_at(texture_id, size * 3, 0, 600);

        window.gl_swap_window();
    }
}

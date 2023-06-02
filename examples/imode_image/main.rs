use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;





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

    let img = image::open("examples/imode_image/Consolas_0_32.png").unwrap().into_rgba8();;

    let aspect = img.height() as f32 / img.width() as f32;
    let texture_id = ui.register_image(&img);

    let mut size = na::Vector2::<f32>::new(100.0, 100.0 * aspect);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.slider(&mut size.x, 10.0, 1000.0);
        size.y = size.x * aspect;

        ui.newline();

        ui.image(texture_id, size);

        ui.image(texture_id, size * 2.0);

        ui.image(texture_id, size * 4.0);
        ui.image_at(texture_id, size * 3.0, 0, 600);

        window.gl_swap_window();
    }
}

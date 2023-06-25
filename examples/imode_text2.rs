use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::typedef::*;
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
    let _onoff = false;
    let color = Color::Rgb(0,0,0);

     // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }


    let mut show = false;


    let _input = "".to_string();
    loop {

        let _c_vec = color.as_vec4();
        unsafe {
            // gl.ClearColor(c_vec.x, c_vec.y, c_vec.z, c_vec.w);

        }

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.drawer2D.render_text("B", 100, 100, 400);

        ui.checkbox(&mut show);

        if show {
            let font_tex = ui.drawer2D.font_cache.default(20).texture_id();

            ui.drawer2D.render_img(font_tex, 50,50, V2::new(600.0, 400.0));
        }

        window.gl_swap_window();
    }
}

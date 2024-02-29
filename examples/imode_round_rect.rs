use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;

use gl_lib::color::Color;


fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    let mut w = 100.0;
    let mut h = 100.0;
    let mut r = 30.0;
    loop {

        ui.start_frame(&mut sdl_setup.event_pump);


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


        ui.end_frame();
    }
}

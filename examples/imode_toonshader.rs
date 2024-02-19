use gl_lib::{gl, ScreenBox, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::text_rendering::text_renderer::TextAlignment;


use gl_lib::color::Color;


fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    loop {
        ui.start_frame(&mut sdl_setup.event_pump);

        ui.body_text("Text");

        ui.end_frame();
    }
}

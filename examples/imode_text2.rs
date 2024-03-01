use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::typedef::*;
use gl_lib::color::Color;


fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    let _onoff = false;
    let color = Color::Rgb(0,0,0);

    let mut show = false;


    let _input = "".to_string();
    loop {

        ui.start_frame(&mut sdl_setup.event_pump);

        ui.drawer2D.render_text("B", 100, 100, 400);

        ui.checkbox(&mut show);

        if show {
            let font_tex = ui.drawer2D.font_cache.default(20).texture_id();

            ui.drawer2D.render_img(font_tex, 50,50, V2::new(600.0, 400.0));
        }

        ui.end_frame();
    }
}

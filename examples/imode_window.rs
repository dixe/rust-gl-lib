use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;

use gl_lib::color::Color;


fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    let mut onoff = false;
    let mut color = Color::Rgb(0,0,0);

    let mut show = true;
    loop {

        ui.start_frame(&mut sdl_setup.event_pump);

        ui.heading_text("Not in a window");

        ui.newline();

        ui.body_text("BOdy text beloiong to the main windows in the app");

        ui.newline();
        ui.small_text("And some small text that belongs the the base");


        if show {
            let res = ui.window_begin("Window1");
            ui.label("Show text:");
            ui.checkbox(&mut onoff);

            if onoff {
                ui.newline();
                ui.body_text("Some text in the window");
            }

            ui.newline();


            ui.color_picker(&mut color);
            ui.window_end("Window1");

            show = !res.closed;
        }

        if !show {
            show = ui.button("Show window");
        }



        ui.color_picker(&mut color);
        ui.color_picker(&mut color);
        ui.color_picker(&mut color);
        ui.color_picker(&mut color);

        ui.end_frame();
    }
}

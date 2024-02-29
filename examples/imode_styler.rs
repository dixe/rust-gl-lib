use gl_lib::{helpers};

use gl_lib::imode_gui::ui::*;
use gl_lib::math::numeric::Numeric;
use gl_lib::imode_gui::style::Style;


fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    let mut style : Style = Default::default();
    loop {

        ui.start_frame(&mut sdl_setup.event_pump);

        ui.style = style.clone();

        if ui.button("auto_wrap") {
            style.auto_wrap = !style.auto_wrap;
        }

        if ui.button("auto_wrap 2") {
            style.auto_wrap = !style.auto_wrap;
        }

        ui.checkbox(&mut style.auto_wrap);

        ui.checkbox(&mut style.auto_wrap);

        ui.newline();
        style.padding.set(slider_for(&mut ui, &format!("Padding: {:.2?}", style.padding.left) , style.padding.left, 0, 30));


        ui.newline();
        style.spacing.x =  slider_for(&mut ui, &format!("Spacing x: {:.2?}", style.spacing.x), style.spacing.x, 0, 30);

        ui.newline();
        style.spacing.y =  slider_for(&mut ui, &format!("Spacing y: {:.2?}", style.spacing.y), style.spacing.y, 0, 30);

        ui.newline();
        ui.color_picker(&mut style.button.color);

        ui.end_frame() ;
    }
}

fn slider_for<T>(ui: &mut Ui, txt: &str, t: T, min: T, max: T) -> T where T : Numeric + std::ops::SubAssign<i32> + std::ops::AddAssign<i32>{
    let mut val = t;
    ui.label(txt);
    ui.slider(&mut val, min, max);
    if ui.button("+") {
        val += 1;
    }
    if ui.button("-") {
        val -= 1;
    }
    val
}

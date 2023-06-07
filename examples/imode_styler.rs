use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::math::numeric::Numeric;
use gl_lib::imode_gui::style::Style;


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

    let mut style : Style = Default::default();
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.style = style.clone();

        ui.consume_events(&mut event_pump);


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

        window.gl_swap_window();
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

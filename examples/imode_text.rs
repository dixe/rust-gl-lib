use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::numeric::Numeric;
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
    let mut onoff = false;
    let mut color = Color::Rgb(0,0,0);

     // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }


    let mut show = true;

    let mut input = "".to_string();
    loop {

        let c_vec = color.as_vec4();
        unsafe {
            // gl.ClearColor(c_vec.x, c_vec.y, c_vec.z, c_vec.w);

        }

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.heading_text("Not in a window");

        ui.newline();

        ui.textbox(&mut input);
        ui.body_text("BOdy text beloiong to the main windows in the app");

        ui.newline();
        ui.small_text(&("And some small text that belongs the the base input: ".to_owned() + &input));


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

        window.gl_swap_window();
    }
}

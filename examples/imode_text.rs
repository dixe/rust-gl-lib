use gl_lib::{gl, ScreenBox, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::text_rendering::text_renderer::TextAlignment;


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
    let mut color = Color::Rgb(27, 27, 27);


    let mut show = true;

    let mut input = "".to_string();
    let offset = 0.0;
    ui.drawer2D.tr.set_text_color(Color::Rgb(240, 240, 240));
    ui.drawer2D.font_cache.fonts_path = Some("assets\\fonts\\".to_string());

    //ui.drawer2D.setup_instance_buffer();

    let mut x_off = 0.0;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Enable(gl::DEPTH_TEST); // for stuff we need this with instanced rendering of quads, so we still can see text on top

            let cc = color.as_vec4();

            gl.ClearColor(cc.x, cc.y, cc.z, cc.w);

            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.heading_text("Not in a window");

        ui.newline();

        //ui.textbox(&mut input);
        ui.body_text("Body text beloiong to the main windows in the app");

        ui.newline();

        if ui.button("Reload shaders") {
            ui.drawer2D.reload_all_shaders();
        }

        /*
        ui.body_text(&("And some small text that belongs the the base input: ".to_owned() + &input));

        ui.newline();

*/
        //ui.newline();

        //println!("{:?}", x_off);
        ui.drawer2D.tr.render_text(&gl, "Text that will slide", TextAlignment::default(), ScreenBox { x: x_off+ 100.0, y: 100.0, width: 300.0, height: 100.0, screen_w: viewport.w as f32, screen_h: viewport.h as f32}, 16);

        ui.slider(&mut x_off, 0.0, 1.0);


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

        ui.finalize_frame();

        window.gl_swap_window();
    }
}

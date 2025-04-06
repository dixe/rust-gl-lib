use gl_lib::{ScreenBox, helpers};


use gl_lib::text_rendering::text_renderer::TextAlignment;


use gl_lib::color::Color;


fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    let gl = sdl_setup.gl.clone();
    let viewport = sdl_setup.viewport;
    let mut ui = sdl_setup.ui();

    let mut onoff = false;
    let mut color = Color::Rgb(27, 27, 27);

    let mut show = false;

    let mut input = "".to_string();
    let _offset = 0.0;
    ui.drawer2D.tr.set_text_color(Color::Rgb(240, 240, 240));
    ui.drawer2D.font_cache.fonts_path = Some("assets\\fonts\\".to_string());
    let mut text_color = ui.drawer2D.tr.color;
    //ui.drawer2D.setup_instance_buffer();

    let mut x_off = 0.0;
    loop {

        ui.start_frame(&mut sdl_setup.event_pump);

        ui.textbox(&mut input);
        ui.body_text("Body text beloiong to the main windows in the app");

        ui.newline();

        if ui.button("Reload shaders") {
            ui.drawer2D.reload_all_shaders();
        }

        ui.newline();

        ui.body_text(&("And some small text that belongs the the base input: ".to_owned() + &input));

        ui.newline();

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



            if ui.color_picker(&mut text_color) {
                ui.drawer2D.tr.set_text_color(text_color);
                ui.style.text_field.text_color = text_color;
            }

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

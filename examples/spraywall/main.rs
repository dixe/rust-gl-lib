use gl_lib::{shader, helpers};
use gl_lib::typedef::*;
use gl_lib::na;
use gl_lib::image::PreMulAlpha;
use gl_lib::imode_gui::widgets::ZoomData;

fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    let path = "examples/spraywall/spraywall.jpg";

    let mut img = image::open(path).unwrap().into_rgba8();

    img.pre_multiply_alpha();

    let texture_id = ui.register_image(&img);
    let aspect = img.height() as f32 / img.width() as f32;

    let mut img_info = ZoomData {
        size: na::Vector2::<f32>::new(800.0, 800.0),
        zoom : 1.0,
        zoom_point: V2::new(img.width() as f32 / 2.0, img.height() as f32 / 2.0),
        texture_id
    };

    loop {

        ui.start_frame(&mut sdl_setup.event_pump);


        // handle inputs

        for _event in &ui.frame_events {
            //println!("{:?}", event);
        }

        ui.window_begin("Settings");

        ui.label("Img Size:");
        ui.slider(&mut img_info.size.x, 50.0, 1900.0);
        img_info.size.y = img_info.size.x * aspect;

        ui.newline();

        ui.label("Zoom:");
        ui.slider(&mut img_info.zoom, 0.1, 10.0);

        ui.window_end("Settings");

        ui.window_begin("Image");

        ui.image_zoom(&mut img_info);

        ui.window_end("Image");

        if ui.button("Reload shader") {
            let vert = std::fs::read_to_string("assets/shaders/objects/image.vert").unwrap();
            let frag = std::fs::read_to_string("assets/shaders/objects/image.frag").unwrap();

            match shader::BaseShader::new(&ui.drawer2D.gl, &vert, &frag) {
                Ok(s) => {
                    ui.drawer2D.texture_shader.shader = s;
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        ui.end_frame();

    }
}

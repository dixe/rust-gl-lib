use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use deltatime;
use gl_lib::text_rendering::font::{Font, MsdfFont, FntFont};
use gl_lib::shader::BaseShader;
use image::{imageops, RgbaImage};
use sdl2::event;
use gl_lib::texture::TextureId;

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

    let mut state = State::NoImage;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        handle_inputs(&mut ui, &mut state);



        if ui.button("Clear img") {
            state = State::NoImage;
        }

        if ui.button("Reload Shader") {
            let vert = "assets/shaders/objects/image.vert";
            let frag = "assets/shaders/objects/image.frag";

            let shader = BaseShader::new(&ui.drawer2D.gl, &std::fs::read_to_string(vert).unwrap(), &std::fs::read_to_string(frag).unwrap());

            match shader {
                Ok(s) => {
                    ui.drawer2D.texture_shader.shader = s;
                },
                Err(err) => {
                    println!("{:?}", err);
                }
            }


        }

        match &mut state {
            State::NoImage => {
                no_image(&mut ui);
            },
            State::Image(img_info) => {
                image(&mut ui, img_info);
            }
        };

        window.gl_swap_window();
    }
}


fn no_image(ui: &mut Ui) {
    ui.heading_text("Drop an image file to explore RGBA components");
}

fn image(ui: &mut Ui, img_info: &mut ImgInfo) {
    ui.slider(&mut img_info.scale, 0.0, 5.0);
    ui.newline();
    ui.image(img_info.texture_id, img_info.orig_size * img_info.scale);
}

#[derive(Debug, Clone)]
enum State {
    NoImage,
    Image(ImgInfo)
}

#[derive(Debug, Clone)]
struct ImgInfo {
    texture_id: TextureId,
    orig_size: na::Vector2::<f32>,
    scale: f32
}


fn handle_inputs(ui: &mut Ui, state: &mut State) {

    use event::Event::*;

    for e in ui.get_frame_inputs() {
        match e {
            DropFile {filename, .. } => {
                match state {
                    State::NoImage => {
                        let img = image::open(filename).unwrap().into_rgba8();

                        let texture_id = ui.register_image(&img);
                        let size = na::Vector2::new(img.width() as f32, img.height() as f32);

                        *state = State::Image(ImgInfo {
                            texture_id,
                            orig_size: size,
                            scale: 1.0
                        })
                    },
                    ref other => {}
                }
            }
            _ => {}
        }
    }
}

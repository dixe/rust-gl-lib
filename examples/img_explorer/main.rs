use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;


use gl_lib::shader::BaseShader;

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

    let shaders = Shaders {
        r: r_shader(&gl),
        g: g_shader(&gl),
        b: b_shader(&gl),
        a: a_shader(&gl),
        rgba: ui.drawer2D.texture_shader.shader.clone()
    };


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

        if ui.button("R") {
            ui.drawer2D.texture_shader.shader = shaders.r.clone();
        }

        if ui.button("G") {
            ui.drawer2D.texture_shader.shader = shaders.g.clone();
        }

        if ui.button("B") {
            ui.drawer2D.texture_shader.shader = shaders.b.clone();
        }

        if ui.button("A") {
            ui.drawer2D.texture_shader.shader = shaders.a.clone();
        }

        if ui.button("RGBA") {
            ui.drawer2D.texture_shader.shader = shaders.rgba.clone();
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


struct Shaders {
    r: BaseShader,
    g: BaseShader,
    b: BaseShader,
    a: BaseShader,
    rgba: BaseShader,
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
                    ref _other => {}
                }
            }
            _ => {}
        }
    }
}


fn r_shader(gl: &gl::Gl) -> BaseShader {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/image.vert");

    let frag_source = "#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  float col = texture2D(text_map, IN.TexCoords).r;
  FragColor = vec4(col, col, col, 1.0);
}
";

    BaseShader::new(gl, vert_source, frag_source).unwrap()

}


fn g_shader(gl: &gl::Gl) -> BaseShader {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/image.vert");

    let frag_source = "#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  float col = texture2D(text_map, IN.TexCoords).g;
  FragColor = vec4(col, col, col, 1.0);
}
";

    BaseShader::new(gl, vert_source, frag_source).unwrap()

}

fn b_shader(gl: &gl::Gl) -> BaseShader {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/image.vert");

    let frag_source = "#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  float col = texture2D(text_map, IN.TexCoords).b;
  FragColor = vec4(col, col, col, 1.0);
}
";

    BaseShader::new(gl, vert_source, frag_source).unwrap()

}

fn a_shader(gl: &gl::Gl) -> BaseShader {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/image.vert");

    let frag_source = "#version 330 core
out vec4 FragColor;

uniform sampler2D text_map;

in VS_OUTPUT {
  vec2 TexCoords;
} IN;

void main()
{
  float col = texture2D(text_map, IN.TexCoords).a;
  FragColor = vec4(col, col, col, 1.0);
}
";

    BaseShader::new(gl, vert_source, frag_source).unwrap()

}

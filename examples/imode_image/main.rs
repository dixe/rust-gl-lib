use gl_lib::{gl, na, helpers, shader};
use gl_lib::image::*;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;





fn main() -> Result<(), failure::Error> {

    let args: Vec<String> = std::env::args().collect();

    let mut path = "examples/imode_image/Consolas_0_32.png";
    if args.len() > 1 {
        path = &args[1];
    } else {
        println!("Using default img path {path}");
    }
    let mut pre_mul = false;
    for arg in &args {
        if arg == "--premul" {
            pre_mul = true;
        }
    }


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

    let mut img = image::open(path).unwrap().into_rgba8();
    if pre_mul {
        img.pre_multiply_alpha();
    }

    let aspect = img.height() as f32 / img.width() as f32;
    let texture_id = ui.register_image_nearest(&img);

    let mut size = na::Vector2::<f32>::new(100.0, 100.0 * aspect);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        ui.slider(&mut size.x, 10.0, 1000.0);
        size.y = size.x * aspect;

        if ui.button("Reload shader") {
            let vert = std::fs::read_to_string("assets/shaders/objects/image.vert").unwrap();
            let frag = std::fs::read_to_string("assets/shaders/objects/image.frag").unwrap();

            match shader::BaseShader::new(&gl, &vert, &frag) {
                Ok(s) => {
                    ui.drawer2D.texture_shader.shader = s;
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        ui.newline();

        ui.image(texture_id, size);

        ui.image(texture_id, size * 2.0);

        ui.image(texture_id, size * 4.0);
        ui.image_at(texture_id, size * 3.0, 0, 600);

        window.gl_swap_window();
    }
}

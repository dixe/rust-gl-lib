use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::GraphInfo;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};

fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    let mut event_pump = sdl.event_pump().unwrap();

     // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut fun : fn(f32) -> f32 = linear;

    let mut info = GraphInfo {
        w: 200,
        h: 200,
        start: 0.0,
        end: 200.0
    };



    let mut fs : Vec::<(&'static str, fn(f32) -> f32)> = Vec::new();

    add_fn(linear, &mut fs);
    add_fn(zero, &mut fs);

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }



        for (name, f) in &fs {
            if ui.button(name) {
                fun = *f;
            }
        }

        ui.newline();

        ui.graph(&info, fun);



        ui.newline();
        ui.combo_box(&mut info.start, -100.0, 100.0);
        ui.slider(&mut info.start, -100.0, 100.0);
        ui.newline();
        ui.combo_box(&mut info.end, 0.0, 200.0);
        ui.slider(&mut info.end, 0.0, 200.0);



        ui.consume_events(&mut event_pump);
        window.gl_swap_window();
    }
}

fn add_fn<>(f: fn(f32) -> f32, fs: &mut Vec::<(&'static str, fn(f32) -> f32)>)
{
    fs.push((get_function_name(f), f));
}


fn get_function_name<F>(_: F) -> &'static str
where
    F: Fn(f32) -> f32,
{
    std::any::type_name::<F>()
}


fn linear(x: f32) -> f32 {
    x
}



fn zero(x: f32) -> f32 {
    0.0
}

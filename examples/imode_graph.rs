use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::GraphInfo;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};

fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

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

        ui.start_frame(&mut sdl_setup.event_pump);

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


        ui.end_frame();
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

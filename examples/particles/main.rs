use gl_lib::{helpers};
use gl_lib::color::Color;
use gl_lib::particle_system::*;

use gl_lib::imode_gui::ui::*;
use sdl2::event;
use rand::Rng;
use gl_lib::particle_system::particle_circle::ParticleCircle;
use gl_lib::typedef::V3;

fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    let mut emitter = emitter::Emitter::new(10000, emitter::emit_random_cirlce, emitter::update_linear);


    let mut args = Args {
        life: 8.0,
        speed: 70.0,
        spread: 3.0,
        color_from: Color::Rgb(255,10,140),
        color_to: Color::RgbA(130, 150, 30, 180)
    };

    let mut state = State::default();
    loop {
        ui.start_frame(&mut sdl_setup.event_pump);

        handle_inputs(&ui, &mut state);

        // ui
        ui.body_text(&format!("Speed: {:.2}", args.speed));
        ui.slider(&mut args.speed, 1.0, 200.0);

        ui.body_text(&format!("Life: {:.2}", args.life));
        ui.slider(&mut args.life, 0.0, 10.0);

        ui.body_text("Spread");
        ui.slider(&mut args.spread, 0.0, 7.0);

        ui.color_picker(&mut args.color_from);

        ui.color_picker(&mut args.color_to);


        if let Some((x,y)) = state.mouse_pos {
            for _ in 0..3 {
                emitter.emit(x, y);
                // can also just emit from a arbitrary emit func
                //emitter.emit_from_fn(x, y, |p, x, y| emit_linear(p, x, y, args));
            }
        }

        let dt = ui.dt();
        emitter.update(dt);
        emitter.draw_all(|p| particle_circle::render(p, &mut ui.drawer2D));


        ui.end_frame();
    }
}


#[derive(Clone, Copy)]
pub struct Args {
    life: f32,
    speed: f32,
    spread: f32,
    color_from: Color,
    color_to: Color
}


#[derive(Default)]
struct State {
    mouse_pos: Option<(f32, f32)>
}


fn handle_inputs(ui: &Ui, state: &mut State) {

    use event::Event::*;

    for e in ui.get_frame_inputs() {
        match e {
            MouseButtonDown {x, y, ..} => {
                state.mouse_pos = Some((*x as f32, *y as f32));
            },
            MouseButtonUp {  ..} => {
                state.mouse_pos = None;
            },
            MouseMotion { x, y, mousestate, ..} => {
                if mousestate.left() {
                    state.mouse_pos = Some((*x as f32, *y as f32));
                }
            },

            _ => {}
        }
    }
}

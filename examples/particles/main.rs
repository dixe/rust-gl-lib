use gl_lib::{gl, na, helpers};
use gl_lib::color::Color;
use gl_lib::particle_system::*;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use sdl2::event;
use rand::Rng;
use gl_lib::particle_system::particle_circle::ParticleCircle;
use gl_lib::typedef::V3;

fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    let mut emitter = emitter::Emitter::new(10000, emitter::emit_1, emitter::update_1);


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
                emitter.emit_from_fn(x, y, |p, x, y| emit_1(p, x, y, args));
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

pub fn emit_1(p: &mut ParticleCircle, x: f32, y: f32, args: Args) {

    // TODO maybe have in struct
    let mut rng = rand::thread_rng();

    let angle : f32 = rng.gen::<f32>() * args.spread - std::f32::consts::PI / 2.0;

    p.pos.x = x;
    p.pos.y = y;

    let x = angle.cos();
    let y = angle.sin();

    let dir = V3::new(x,y, 0.0).normalize();

    p.vel = dir * args.speed;


    let life = (rng.gen::<f32>() - 0.5) * 3.0 + args.life;
    p.life = life;

    p.color_from = args.color_from;
    p.color_to = args.color_to;

    // max size 10
    p.size = rng.gen::<f32>() * 1.1;
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

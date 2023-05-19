use gl_lib::{gl, na, helpers};
use gl_lib::color::Color;
use gl_lib::particle_system::*;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use deltatime;
use gl_lib::text_rendering::font::{Font, MsdfFont, FntFont};
use gl_lib::shader::BaseShader;
use sdl2::event;
use rand::Rng;
use gl_lib::particle_system::particle::Particle;


fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut emitter = emitter::Emitter::new(10000, emitter::emit_1, emitter::update_1);
    let mut delta_time = deltatime::Deltatime::new();


    let mut args = Args {
        life: 8.0,
        speed: 70.0,
        spread: 3.0,
        color_from: Color::Rgb(255,10,140),
        color_to: Color::RgbA(130, 150, 30, 180)
    };

    let mut state = State::default();
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // consume events and store them and their info
        ui.consume_events(&mut event_pump);
        delta_time.update();

        // ui
        ui.body_text(&format!("Speed: {:.2}", args.speed));
        ui.slider(&mut args.speed, 1.0, 200.0);

        ui.body_text(&format!("Life: {:.2}", args.life));
        ui.slider(&mut args.life, 0.0, 10.0);

        ui.body_text("Spread");
        ui.slider(&mut args.spread, 0.0, 7.0);

        ui.color_picker(&mut args.color_from);

        ui.color_picker(&mut args.color_to);



        // Handle frame input after ui, so we know i the ui used fx a click, and we will ignore it
        handle_inputs(&ui, &mut state);

        if let Some((x,y)) = state.mouse_pos {
            for _ in 0..30 {
                emitter.emit_from_fn(x, y, |p, x, y| emit_1(p, x, y, args));
            }
        }

        let dt = delta_time.time();
        emitter.update(dt);
        emitter.draw_all(&ui.drawer2D);

        window.gl_swap_window();
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


pub fn emit_1(p: &mut Particle, x: f32, y: f32, args: Args) {

    // TODO maybe have in struct
    let mut rng = rand::thread_rng();

    let angle : f32 = rng.gen::<f32>() * args.spread - std::f32::consts::PI / 2.0;

    p.pos.x = x;
    p.pos.y = y;

    let x = angle.cos();
    let y = angle.sin();

    let dir = na::Vector2::new(x,y).normalize();

    p.vel = dir * args.speed;


    let life = (rng.gen::<f32>() - 0.5) * 3.0 + args.life;
    p.life = life;

    p.color_from = args.color_from;
    p.color_to = args.color_to;

    // max size 10
    p.size = rng.gen::<f32>() * 1.1;
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
            MouseButtonUp {x, y, ..} => {
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

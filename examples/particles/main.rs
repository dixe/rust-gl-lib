use gl_lib::{gl, na, helpers};
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

    let mut emitter = emitter::Emitter::new(1000, emitter::emit_1, emitter::update_1);
    let mut delta_time = deltatime::Deltatime::new();
    let mut amount = 1;
    let mut speed = 30.0;
    let mut state = State::default();
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        delta_time.update();

        handle_inputs(&mut ui, &mut event_pump, &mut state);
        ui.slider(&mut speed, 1.0, 200.0);


        if let Some((x,y)) = state.mouse_pos {
            emitter.emit_from_fn(x, y, |p, x, y| emit_1(p, x, y, speed));
        }

        let dt = delta_time.time();
        emitter.update(dt);
        emitter.draw_all(&ui.drawer2D);

        window.gl_swap_window();
    }
}

pub fn emit_1(p: &mut Particle, x: f32, y: f32, speed: f32) {

    // TODO maybe have in struct
    let mut rng = rand::thread_rng();

    let angle : f32 = rng.gen::<f32>() * -std::f32::consts::PI;

    p.pos.x = x;
    p.pos.y = y;

    let x = angle.cos();
    let y = angle.sin();

    let dir = na::Vector2::new(x,y).normalize();

    p.vel = dir * speed;

    p.life = 8.0;
    p.total_life = 8.0;

    // max size 10
    p.size = rng.gen::<f32>() * 1.1;
}



#[derive(Default)]
struct State {
    mouse_pos: Option<(f32, f32)>
}

fn handle_inputs(ui: &mut Ui, event_pump: &mut sdl2::EventPump, state: &mut State) {

    use event::Event::*;

    for e in ui.consume_events(event_pump) {
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

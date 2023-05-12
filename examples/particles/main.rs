use gl_lib::{gl, na, helpers};
use gl_lib::particle_system::*;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use deltatime;

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

    let mut emitter = emitter::Emitter::new(1000, emitter::emit_1, emitter::update_1);
    let mut delta_time = deltatime::Deltatime::new();

    let mut amount = 1;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        delta_time.update();


        ui.consume_events(&mut event_pump);

        if ui.button("Emit") {
            for _ in 0..amount {
                emitter.emit();
            }
        }

        ui.label("Amount");
        ui.slider( &mut amount, 1, 100);

        let dt = delta_time.time();
        emitter.update(dt);
        emitter.draw_all(&ui.drawer2D);



        window.gl_swap_window();
    }
}

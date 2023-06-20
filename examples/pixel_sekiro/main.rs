use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::audio::audio_player;
use gl_lib::animations::sheet_animation::{load_folder, SheetAnimationPlayer};
use gl_lib::typedef::*;
use gl_lib::math::AsV2;
mod inputs;
mod entity;
mod ai;
mod scene;


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let audio_subsystem = sdl.audio().unwrap();
    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();

    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut audio_player = audio_player::AudioPlayer::new(audio_subsystem);
    loop {
        audio_player = load_and_run(&gl, audio_player, &mut ui, &window, &mut event_pump)?;
    }
}


pub fn load_and_run(gl: &gl::Gl,
                    mut audio_player: audio_player::AudioPlayer,
                    ui: &mut Ui,
                    window: &sdl2::video::Window,
                    event_pump: &mut sdl2::EventPump) -> Result<audio_player::AudioPlayer, failure::Error> {

    let mut pos2 = V2i::new(500, 600);
    let mut animation_player = SheetAnimationPlayer::new();
    let assets = load_folder(gl, &"examples/pixel_sekiro/assets/", scene::frame_data_mapper);


    audio_player.clear();
    audio_player.add_sound("deflect", &"examples/pixel_sekiro/assets/audio/deflect_1.wav");

    let mut scene = scene::new(&mut animation_player, &assets, audio_player);

    scene.add_enemy("skeleton", pos2.v2());

    let mut playing = true;
    let mut time_scale = 1.0;

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(event_pump);

        let dt = ui.dt() * time_scale;

        if playing {
            scene.animation_player.update(dt);
            scene.update(ui, dt);
        }

        if ui.button("Play/Pause") {
            playing = !playing;
        }

        ui.label("Time scale");
        ui.slider(&mut time_scale, 0.1, 3.1);
        ui.small_text(&format!("{:?}", scene.hits));

        ui.label("Show collision boxes");
        ui.checkbox(&mut scene.show_col_boxes);



        if ui.button("Add skeleton") {
             scene.add_enemy("skeleton", pos2.v2());
        }

        if ui.button("Reload") {
            return Ok(scene.destroy());
        }

        if let Some(ref mut enemy) = scene.enemy {
            if ui.button("Switch attack enemy") {
                enemy.active_combo = (enemy.active_combo + 1) % 2;
            }
        }


        ui.drawer2D.z = 1.0;
        ui.drag_point(&mut pos2, 10.0);
        if let Some(ref mut enemy) = scene.enemy {
            enemy.pos = pos2.v2();
        }
        ui.drawer2D.z = 0.0;

        // draw animation frame at locations
        scene.draw(&mut ui.drawer2D);

        if scene.animation_player.active_animations() != 2 {
            let _a = 2;
        }

        window.gl_swap_window();
    }
}

use gl_lib::{gl, helpers};
use gl_lib::shader::Shader;
use gl_lib::shader::{BaseShader, reload_object_shader};
use gl_lib::typedef::*;
use gl_lib::scene_3d as scene;

pub struct PostPData {
    time: f32
}

fn post_process_uniform_set(gl: &gl::Gl, shader: &mut BaseShader, data : &PostPData) {
    shader.set_f32(gl, "time", data.time);
}

fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let _audio_subsystem = sdl.audio().unwrap();

    // disable v-sync
    sdl_setup.video_subsystem.gl_set_swap_interval(0);

    let mut scene = scene::Scene::<PostPData>::new(gl.clone(), viewport)?;
    scene.set_skybox("assets/cubemap/skybox/".to_string());
    scene.load_all_meshes("E:/repos/Game-in-rust/blender_models/player.glb", true);


    let look_at = V3::new(5.0, 3.1, 5.0);
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));
    scene.camera.look_at(look_at);
    scene.inputs.speed = 15.0;
    scene.inputs.sens = 0.70;

    let mut event_pump = sdl.event_pump().unwrap();

    let player_id = scene.create_entity("player");
    let player_skel_id = scene.entity(&player_id).unwrap().skeleton_id.unwrap();
    scene.controlled_entity = Some(player_id);
    let player2_id = scene.create_entity("player");


    let world_id = scene.create_entity("world");

    let p1 = scene.entity_mut(&player_id).unwrap();
    p1.pos = V3::new(0.0, 00.0, 1.0);

    let p2 = scene.entity_mut(&player2_id).unwrap();
    p2.pos = V3::new(00.0, 2.0, 1.0);


    let mut playing = true;

    let post_process_data = PostPData {
        time : 0.0
    };

    scene.use_fbos(post_process_data, Some(post_process_uniform_set));
    scene.use_stencil();
    scene.use_shadow_map();

    let mut show_options = false;
    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(&mut event_pump);

        let dt = scene.dt();

        // OWN GAME LOGIC
        if scene.ui.button("Play/Pause") {
            playing = !playing;
        }

        // OWN GAME LOGIC
        if scene.ui.button("Change Camera") {
            scene.change_camera();
        }

        let p1 = scene.entities.get(&player_id).unwrap();


        scene.camera_follow(p1.pos + p1.root_motion);


        if scene.ui.button("Reload") {
            reload_object_shader("mesh_shader", &gl, &mut scene.mesh_shader.shader);
            if let Some(ref mut fbos) = scene.fbos {
                reload_object_shader("postprocess", &gl, &mut fbos.post_process_shader.shader);
            }
            if let Some(ref mut stencil) = scene.stencil_shader {
                reload_object_shader("stencil", &gl, &mut stencil.shader);
            }
            reload_object_shader("cubemap", &gl, &mut scene.cubemap_shader);
        }


        if show_options {

            let ui = &mut scene.ui;

            show_options = !ui.window_begin("Options").closed;

            ui.body_text(&format!("fps: {}", 1.0/dt));
            ui.newline();

            ui.label("Sens");
            ui.slider(&mut scene.inputs.sens, 0.01, 1.0);

            ui.label("Speed");
            ui.slider(&mut scene.inputs.speed, 0.01, 20.0);

            ui.newline();
            let p1 = scene.entities.get(&player_id).unwrap();
            ui.body_text(&format!("pos: {:.2?}", p1.pos));

            ui.newline();
            ui.body_text(&format!("root pos: {:.2?}", p1.root_motion));

            ui.newline();
            ui.body_text(&format!("light_pos {:.2?}", scene.light_pos));

            ui.newline();
            ui.body_text("x:");
            ui.slider(&mut scene.light_pos.x, -30.0, 30.0 );

            ui.newline();
            ui.body_text("y:");
            ui.slider(&mut scene.light_pos.y, -30.0, 30.0 );

            ui.newline();
            ui.body_text("z:");
            ui.slider(&mut scene.light_pos.z, 1.0, 300.0 );
            ui.window_end("Options");
        }

        // change animation of entity
        let player_skel = scene.entity(&player_id).unwrap().skeleton_id.unwrap();
        let animations = scene.animations.get(&player_skel).unwrap();
        for (name, anim) in animations {
            if scene.ui.button(name) {
                scene::play_animation(anim.clone(), false, &player_id, &mut scene.player, &mut scene.entities);
            }
        }

        if let Some(ref mut fbos) = scene.fbos {
            fbos.post_process_data.time += dt;
            if scene.ui.button("Reset") {
                let p1 = scene.entities.get_mut(&player_id).unwrap();
                p1.pos = V3::new(-10.0, 0.0, 1.0);
                fbos.post_process_data.time = 0.0;
            }
        }

        scene.ui.newline();

        if !show_options {
            show_options = scene.ui.button("Options");
        }

        scene.ui.newline();

        if scene.player.expired(&player_id) {

            let idle = scene.animations.get(&player_skel_id).unwrap().get("t_pose").unwrap();

            scene::play_animation(idle.clone(), true, &player_id, &mut scene.player, &mut scene.entities);
        }

        if let Some(e) = scene.entities.get_mut(&player_id) {



            //scene.ui.slider(&mut e.angle, 0.0, std::f32::consts::TAU);

        }

        // update aimaiton player, and bones, and root motion if any
        if playing {
            scene.update_animations();
        }

        scene.render();
        window.gl_swap_window();
    }
}

use gl_lib::{gl, helpers};





use gl_lib::shader::Shader;




use gl_lib::shader::{BaseShader, reload_object_shader};
use gl_lib::typedef::*;


mod scene;

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

    let mut scene = scene::Scene::new((), gl.clone(), viewport)?;
    scene.set_skybox(&"assets/cubemap/skybox/");
    scene.load_all_meshes("E:/repos/Game-in-rust/blender_models/Animation_test.glb");

    let mut event_pump = sdl.event_pump().unwrap();

    let player_id = scene.create_entity("Cube");

    let look_at = V3::new(5.0, 3.1, 5.0);
    scene.free_controller.speed = 5.0;
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));
    scene.camera.look_at(look_at);

    let mut playing = true;

    let ppd = PostPData {
        time : 0.0
    };

    scene.use_fbos(ppd, Some(post_process_uniform_set));

    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(&mut event_pump);

        let dt = scene.dt();

        // OWN GAME LOGIC
        if scene.ui.button("Play/Pause") {
            playing = !playing;
        }

        if scene.ui.button("Reload") {
            reload_object_shader("mesh_shader", &gl, &mut scene.mesh_shader.shader);
            if let Some(ref mut fbos) = scene.fbos {
                reload_object_shader("postprocess", &gl, &mut fbos.post_process_shader.shader);
            }
            reload_object_shader("cubemap", &gl, &mut scene.cubemap_shader);
        }

        // change animation of entity
        let player_skel = scene.get_entity(&player_id).unwrap().skeleton_id.unwrap();
        let animations = scene.animations.get(&player_skel).unwrap();
        for (name, anim) in animations {
            if scene.ui.button(name) {
                scene::play_animation(anim.clone(), true, &player_id, &mut scene.player, &mut scene.animation_ids);
            }
        }

        if let Some(ref mut fbos) = scene.fbos {
            fbos.post_process_data.time += dt;
            if scene.ui.button("Reset") {
                fbos.post_process_data.time = 0.0;
            }
        }

        // update aimaiton player, and bones
        if playing {
            scene.update_animations();
        }

        scene.render();
        window.gl_swap_window();
    }
}

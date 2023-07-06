use gl_lib::{gl, helpers};
use gl_lib::shader::Shader;
use gl_lib::shader::{BaseShader, reload_object_shader};
use gl_lib::typedef::*;
use gl_lib::scene_3d as scene;
use itertools::Itertools;
use std::rc::Rc;
use gl_lib::objects::gltf_mesh::{KeyFrame, Animation};
use gl_lib::imode_gui::Ui;
use std::collections::HashMap;
use gl_lib::animations::gltf_animation::update_skeleton_to_key_frame;




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
    let mut event_pump = sdl.event_pump().unwrap();

    // disable v-sync
    let _ = sdl_setup.video_subsystem.gl_set_swap_interval(0);
    loop {
        let _ = run_scene(gl, &mut event_pump, viewport, &window, sdl.clone())?;
    }
}


fn select(ui: &mut Ui, animation: &mut Rc::<Animation>, animations: &HashMap::<Rc::<str>, Rc<Animation>>, id: usize, current: &mut Rc::<str>) {

    let name = format!("Choose anim - {id}");
    let has_window = ui.window_to_id.contains_key(&name);

    if ui.button(current) || has_window {
        let res = ui.window_begin(&name);

        let mut i = 0;
        for (name, anim) in animations.iter().sorted_by_key(|x| x.0) {

            if ui.button(name) {
                *current = name.clone();
                *animation = anim.clone();
            }
            i += 1;

            if i > 3 {
                ui.newline();
                i = 0;
            }
        }
        ui.window_end(&name);
    }
}


fn run_scene(gl: &gl::Gl, event_pump: &mut sdl2::EventPump,
             viewport: gl::viewport::Viewport,
             window: &sdl2::video::Window,
             sdl: sdl2::Sdl) -> Result<(), failure::Error> {

    let mut scene = scene::Scene::<PostPData>::new(gl.clone(), viewport, sdl)?;

    let mut cont = true;

    while !cont {
        scene.frame_start(event_pump);
        cont = scene.ui.button("PLAY");

        scene.render();
        window.gl_swap_window();
    }

    scene.load_all_meshes("E:/repos/Game-in-rust/blender_models/player.glb", true);

    let look_at = V3::new(5.0, 3.1, 5.0);
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));
    scene.camera.look_at(look_at);
    scene.inputs.speed = 15.0;
    scene.inputs.sens = 0.70;



    let player_id = scene.create_entity("player");
    let player_skel_id = scene.entity(&player_id).unwrap().skeleton_id.unwrap();
    scene.controlled_entity = Some(player_id);

    let world_id = scene.create_entity("world");

    let p1 = scene.entity_mut(&player_id).unwrap();
    p1.pos = V3::new(0.0, 00.0, 1.0);

    let mut playing = true;

    let post_process_data = PostPData {
        time : 0.0
    };

    scene.use_fbos(post_process_data, Some(post_process_uniform_set));
    scene.use_stencil();
    scene.use_shadow_map();

    let mut speed = 1.0;
    let mut show_options = false;

    let mut anim_1 = scene.animations.get(&player_skel_id).unwrap().get("idle").unwrap().clone();
    let mut name_1: Rc::<str>= Rc::from("idle".to_owned());

    let mut anim_2 = scene.animations.get(&player_skel_id).unwrap().get("idle").unwrap().clone();
    let mut name_2: Rc::<str> = Rc::from("idle".to_owned());

    let mut tmp_keyframe = anim_1.frames[0].clone();
    let mut t = 0.0;
    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(event_pump);

        let dt = scene.dt();

        // OWN GAME LOGIC
        if scene.ui.button("Play/Pause") {
            playing = !playing;
        }

        if scene.ui.button("Change Camera") {
            scene.change_camera();
        }


        select(&mut scene.ui, &mut anim_1, &scene.animations.get(&player_skel_id).unwrap(), 1, &mut name_1);

        select(&mut scene.ui, &mut anim_2, &scene.animations.get(&player_skel_id).unwrap(), 2, &mut name_2);


        if scene.ui.button("Reload") {
            reload_object_shader("mesh_shader", &gl, &mut scene.mesh_shader.shader);
            if let Some(ref mut fbos) = scene.fbos {
                reload_object_shader("postprocess", &gl, &mut fbos.post_process_shader.shader);
            }
            if let Some(ref mut stencil) = scene.stencil_shader {
                reload_object_shader("stencil", &gl, &mut stencil.shader);
            }
            //reload_object_shader("cubemap", &gl, &mut scene.cubemap_shader);
        }

        if scene.ui.button("Reset Scene") {
            return Ok(());
        }

        scene.ui.combo_box(&mut t, -0.2, 1.2);
        scene.ui.slider(&mut t, 0.0, 1.0);

        // update aimaiton player, and bones, and root motion if any
        if playing {

            let skeleton = &mut scene.skeletons[player_skel_id];


            let frame_1 = &anim_1.frames[0];
            let frame_2 = &anim_2.frames[0];


            frame_1.interpolate(frame_2, t, &mut tmp_keyframe);


            update_skeleton_to_key_frame(skeleton, &tmp_keyframe);
            let bones = scene.bones.get_mut(&player_id).unwrap();
            skeleton.set_all_bones_from_skeleton(bones);
        }

        scene.render();
        window.gl_swap_window();
    }

}

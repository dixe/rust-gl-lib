use gl_lib::{gl, na, helpers};
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
use gl_lib::animations::skeleton::Skeleton;



pub struct PostPData {
    time: f32
}


fn post_process_uniform_set(gl: &gl::Gl, shader: &mut BaseShader, data : &PostPData) {
    shader.set_f32(gl, "time", data.time);
}


fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;

    loop {
        let _ = run_scene(gl, sdl_setup)?;
    }
}


fn select(ui: &mut Ui, animation: &mut Rc::<Animation>, animations: &HashMap::<Rc::<str>, Rc<Animation>>, id: usize, current: &mut Rc::<str>) {

    let name = format!("Choose anim - {id}");
    let has_window = ui.window_to_id.contains_key(&name);

    if ui.button(current) || has_window {
        let _res = ui.window_begin(&name);

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


fn run_scene(gl: &gl::Gl, sdl_setup: &mut helpers::BasicSetup) -> Result<(), failure::Error> {

    let mut scene = scene::Scene::<PostPData, ()>::new(sdl_setup)?;

    let mut cont = true;

    while !cont {
        scene.frame_start(event_pump);
        cont = scene.ui.button("PLAY");

        scene.render();
        window.gl_swap_window();
    }

    scene.load_all_meshes("examples/assets/blender_models/player.glb", true);

    let look_at = V3::new(5.0, 3.1, 5.0);
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));
    scene.camera.look_at(look_at);
    scene.inputs.speed = 15.0;
    scene.inputs.sens = 0.70;



    let player_id = scene.create_entity("player");
    let player_skel_id = scene.entity(&player_id).unwrap().skeleton_id.unwrap();

    scene.controlled_entity = Some(scene::ControlledEntity {
        id: player_id,
        user_data: (),
        control_fn: scene::base_controller
    });

    let _world_id = scene.create_entity("world");

    let p1 = scene.entity_mut(&player_id).unwrap();
    p1.pos = V3::new(0.0, 00.0, 1.0);

    let mut playing = true;

    let post_process_data = PostPData {
        time : 0.0
    };

    scene.use_fbos(post_process_data, Some(post_process_uniform_set));
    scene.use_stencil();
    scene.use_shadow_map();

    let _speed = 1.0;
    let _show_options = false;

    let anims = scene.animations.get(&player_skel_id).unwrap();
    let mut anim_1 = scene.animations.get(&player_skel_id).unwrap().get("run_0").unwrap().clone();
    let mut name_1: Rc::<str>= Rc::from("run_0".to_owned());

    let mut anim_2 = scene.animations.get(&player_skel_id).unwrap().get("run_1").unwrap().clone();
    let mut name_2: Rc::<str> = Rc::from("run_1".to_owned());

    let mut tmp_keyframe = anim_1.frames[0].clone();
    let mut t = 0.0;

    let _skeleton = &scene.skeletons[player_skel_id];
    let _wheel = Wheel {
        frames: vec![anims.get("run_0").unwrap().frames[0].clone(),
                     anims.get("run_1").unwrap().frames[0].clone(),
                     anims.get("run_2").unwrap().frames[0].clone(),
                     anims.get("run_3").unwrap().frames[0].clone()
        ],
        loop_time: 2.0
    };

    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(event_pump);

        let _dt = scene.dt();

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

struct Wheel {
    frames: Vec::<KeyFrame>,
    loop_time: f32
}


fn inverse(keyframe: &KeyFrame, skeleton: &Skeleton) -> KeyFrame {

    let mut new = keyframe.clone();

    let mut name_to_idx: HashMap<String, usize> = HashMap::default();
    for i in 0..skeleton.joints.len() {
        name_to_idx.insert(skeleton.joints[i].name.clone(), i);
    }

    println!("{:?}", name_to_idx);

    for i in 0..skeleton.joints.len() {
        let mut mirror_name = skeleton.joints[i].name.clone();

        if mirror_name.contains(".R") {

            mirror_name = skeleton.joints[i].name.replace(".R", ".L");
            let new_idx = name_to_idx.get(&mirror_name).unwrap();

            let euler = keyframe.joints[i].rotation.euler_angles();
            new.joints[*new_idx].rotation = na::UnitQuaternion::from_euler_angles(euler.0, euler.1, -euler.2);
        } else if mirror_name.contains(".L") {

            mirror_name = skeleton.joints[i].name.replace(".L", ".R");
            let new_idx = name_to_idx.get(&mirror_name).unwrap();

            let euler = keyframe.joints[i].rotation.euler_angles();
            new.joints[*new_idx].rotation = na::UnitQuaternion::from_euler_angles(-euler.0, euler.1, -euler.2);
        }
    }

    new
}

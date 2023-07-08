use gl_lib::{gl, helpers, movement::Inputs, camera::{follow_camera, Camera}};
use gl_lib::shader::Shader;
use gl_lib::shader::{BaseShader, reload_object_shader};
use gl_lib::typedef::*;
use gl_lib::scene_3d as scene;
use itertools::Itertools;
use gl_lib::na::{self, Rotation2};

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


fn run_scene(gl: &gl::Gl, event_pump: &mut sdl2::EventPump,
             viewport: gl::viewport::Viewport,
             window: &sdl2::video::Window,
             sdl: sdl2::Sdl) -> Result<(), failure::Error> {

    let mut scene = scene::Scene::<PostPData, ControlledData>::new(gl.clone(), viewport, sdl)?;


    let mut cont = true;

    while !cont {
        scene.frame_start(event_pump);
        cont = scene.ui.button("PLAY");

        scene.render();
        window.gl_swap_window();
    }

    scene.set_skybox("assets/cubemap/skybox/".to_string());
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
        user_data: ControlledData::Normal,
        control_fn: controller
    });

    let enemy_id = scene.create_entity("player");


    let world_id = scene.create_entity("world");

    let p1 = scene.entity_mut(&player_id).unwrap();
    p1.pos = V3::new(0.0, 00.0, 1.0);

    let p2 = scene.entity_mut(&enemy_id).unwrap();
    p2.pos = V3::new(00.0, 2.0, 1.0);


    let mut playing = true;

    let post_process_data = PostPData {
        time : 0.0
    };

    scene.use_fbos(post_process_data, Some(post_process_uniform_set));
    scene.use_stencil();
    scene.use_shadow_map();

    let mut speed = 1.0;
    let mut show_options = false;
    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(event_pump);

        let dt = scene.dt();

        // OWN GAME LOGIC
        if scene.ui.button("Play/Pause") {
            playing = !playing;
        }

        // OWN GAME LOGIC
        if scene.ui.button("Change Camera") {
            scene.change_camera();
        }


        update_target(&mut scene, player_id, enemy_id);


        let p1 = scene.entities.get(&player_id).unwrap();



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
        for (name, anim) in animations.iter().sorted_by_key(|x| x.0) {
            if scene.ui.button(name) {
                scene::play_animation(anim.clone(), false, speed, &player_id, &mut scene.player, &mut scene.entities);
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
        if scene.ui.slider(&mut speed, 0.1, 10.0) {
            scene.player.change_speed(&player_id, speed);
        }

        scene.ui.newline();

        if !show_options {
            show_options = scene.ui.button("Options");
        }

        scene.ui.newline();

        if scene.player.expired(&player_id) {
            let idle = scene.animations.get(&player_skel_id).unwrap().get("idle").unwrap();
            scene::play_animation(idle.clone(), true, speed, &player_id, &mut scene.player, &mut scene.entities);
        }

        if scene.ui.button("Target") {
            //scene.controlled_entity.set_user_data(user);
        }

        if scene.ui.button("Reset Scene") {
            return Ok(());
        }

        // update aimaiton player, and bones, and root motion if any
        if playing {
            scene.update_animations();
        }

        scene.render();
        window.gl_swap_window();
    }

}

/// when middle click update lock on/off, when lock on, update the target pos in user data, so we can use it
/// later in controller
fn update_target<T,>(scene: &mut scene::Scene<T, ControlledData>, player_id: scene::EntityId, enemy_id: scene::EntityId) {

    let middle_mouse = scene.inputs.middle_mouse;
    if let Some(controlled) = &mut scene.controlled_entity {
        match controlled.user_data {
            ControlledData::Target((id, pos)) => {
                if middle_mouse {
                    controlled.user_data = ControlledData::Normal;
                } else {
                    if let Some(enemy) = scene.entities.get(&enemy_id) {
                        controlled.user_data = ControlledData::Target((enemy_id, enemy.pos));
                    }
               }
            },
            ControlledData::Normal => {
                if middle_mouse {
                    if let Some(enemy) = scene.entities.get(&enemy_id) {
                        controlled.user_data = ControlledData::Target((enemy_id, enemy.pos));
                    }
                }
            }

        }
    }
}

enum ControlledData {
    Normal,
    Target((scene::EntityId, V3))
}


fn controller(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, user_data: &ControlledData) {
    match user_data {
        ControlledData::Normal => controller_regular(entity, camera, follow_camera, inputs, dt),
        ControlledData::Target((_, t)) => controller_target(entity, camera, follow_camera, inputs, dt, t),
    }
}


fn controller_target(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, target: &V3) {

    // face target
    let mut forward = target - entity.pos;
    forward.z = 0.0;
    forward = forward.normalize();

    let tangent = V3::new(forward.y, -forward.x, 0.0);

    let movement = forward * inputs.movement.x + tangent * inputs.movement.y;

    if movement.magnitude() > 0.0 {
        entity.pos += movement.normalize() * inputs.speed * dt;
    }

    if forward.magnitude() > 0.0 {
        let mut new_angle = forward.y.atan2(forward.x);
        angle_change(new_angle, entity, dt);
    }

    // update camera
    follow_camera.update_dist(inputs.mouse_wheel);
    // set desired distance from player
    let xy = entity.pos - forward * follow_camera.desired_distance;

    camera.pos.x = xy.x;
    camera.pos.y = xy.y;
    camera.pos.z += inputs.mouse_movement.yrel * inputs.sens * dt;

    camera.look_at(*target);

}


fn controller_regular(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32) {

     // update player pos
    let mut forward = entity.pos - camera.pos;
    forward.z = 0.0;
    forward = forward.normalize();
    let tangent = V3::new(forward.y, -forward.x, 0.0);

    let mut m = forward * inputs.movement.x + tangent * inputs.movement.y;

    entity.forward_pitch = Rotation2::new(0.0);
    entity.side_pitch = Rotation2::new(0.0);
    if m.magnitude() > 0.0 {

        m = m.normalize(); // check sekrio what happens when holding right or left
        // ignore z since we assume its a char controller that cannot fly

        let mut new_angle = m.y.atan2(m.x);
        angle_change(new_angle, entity, dt);

        entity.forward_pitch = Rotation2::new(0.2 * inputs.movement.x as f32 );
        entity.side_pitch = Rotation2::new(0.2 * inputs.movement.y as f32 );

        entity.pos += m * inputs.speed * dt;
    }

    //Update camera desired pitch and yaw from mouse
    let base_sens = 3.0;

    follow_camera.update_dist(inputs.mouse_wheel);

    follow_camera.desired_pitch += inputs.mouse_movement.yrel * inputs.sens * inputs.inverse_y *  dt * base_sens;

    follow_camera.yaw_change = inputs.mouse_movement.xrel * inputs.sens * dt * base_sens;

    follow_camera.update_camera_target(entity.pos + entity.root_motion);

    follow_camera.update_camera(camera, dt);

}


fn angle_change(new_angle: f32, entity: &mut scene::SceneEntity, dt: f32) {

    let mut diff = new_angle - entity.z_angle.angle();

    // normalize to range -pi to pi
    if diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }

    if diff > std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }

    let sign = diff.signum();
    let r_speed = 10.0;
    // change with max rotation speed
    let mut change = sign * r_speed * dt;

    // if we max speed would over shot target angle, change just the needed amount
    if change.abs() > diff.abs() {
        change = diff;
    }

    // do the update of rotation
    entity.z_angle *= Rotation2::new(change);
}

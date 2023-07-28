use gl_lib::{gl, helpers, movement::Inputs, camera::{follow_camera, Camera}};
use gl_lib::shader::{self, Shader};
use gl_lib::objects::cube;
use gl_lib::shader::{BaseShader, reload_object_shader};
use gl_lib::typedef::*;
use gl_lib::scene_3d as scene;
use itertools::Itertools;
use gl_lib::na::{self, Rotation2, Translation3};

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
    let gl = &sdl_setup.gl;
    let _audio_subsystem = sdl.audio().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    // disable v-sync
    let _ = sdl_setup.video_subsystem.gl_set_swap_interval(0);
    loop {
        let _ = run_scene(gl, &mut event_pump, &window, sdl.clone())?;
    }
}

enum PlayerState {
    Attack,
    Movable,
}

fn run_scene(gl: &gl::Gl, event_pump: &mut sdl2::EventPump,
             window: &sdl2::video::Window,
             sdl: sdl2::Sdl) -> Result<(), failure::Error> {

    let viewport = gl::viewport::Viewport {
        x: 0,
        y: 0,
        w: window.size().0 as i32,
        h: window.size().1 as i32,
    };
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

    scene.load_sound("attack".into(), &"examples/pixel_sekiro/assets/audio/deflect_1.wav");


    let look_at = V3::new(5.0, 3.1, 5.0);
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));
    scene.camera.look_at(look_at);

    scene.inputs.follow.speed = 15.0;
    scene.inputs.follow.sens = 0.15;

    scene.inputs.free.speed = 12.0;
    scene.inputs.free.sens = 0.65;


    let player_id = scene.create_entity("player");
    let _player_state = PlayerState::Movable;

    let _player_skel_id = scene.entity(&player_id).unwrap().skeleton_id.unwrap();

    scene.controlled_entity = Some(scene::ControlledEntity {
        id: player_id,
        user_data: ControlledData {
            camera: CameraState::Normal,
            player: PlayerState::Movable
        },
        control_fn: camera_controller
    });

    let enemy_id = scene.create_entity("player");

    let _world_id = scene.create_entity("world");

    let p1 = scene.entity_mut(&player_id).unwrap();
    p1.pos = V3::new(0.0, 00.0, 0.0);

    let p2 = scene.entity_mut(&enemy_id).unwrap();
    p2.pos = V3::new(0.0, 2.0, 0.0);


    let mut playing = true;

    let post_process_data = PostPData {
        time : 0.0
    };

    scene.use_fbos(post_process_data, Some(post_process_uniform_set));
    //scene.use_stencil();
    scene.use_shadow_map();

    let mut show_options = false;
    scene.skeleton_hit_boxes.insert(player_id, vec![]);

    // TODO: Tmp data for hitboxes should live in scene so we can easy render hitboxes for debug
    let mut hitbox_shader = shader::hitbox_shader::HitboxShader::new(gl).unwrap();
    let base_cube = cube::Cube::new(gl); // base cube used to display hitboxes

    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(event_pump);

        // move to before controller should take whole scene maybe, since we want to update animaiton played
        // controller is actually a camera controller, so should maybe not even do movement2

        handle_input(&mut scene);

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


        let _p1 = scene.entities.get(&player_id).unwrap();


        for col_box in scene.skeleton_hit_boxes.get(&player_id).unwrap() {

            // update model matrix
            // center is just for rendering, since we just translate, rotate and scale a unit cube in a shader
            // The actual box already has these coords added
            let bone_t = Translation3::from(col_box.center);

            // First we want to scale it to the target size
            // its a unit square, so just multiply with w and h
            let mut scale = na::Matrix4::<f32>::identity();
            scale[0] = col_box.side_len;
            scale[5] = col_box.side_len;
            scale[10] = col_box.length;

            let axis = na::Unit::new_normalize(col_box.dir);
            let up = V3::new(0.0, 0.0, 1.0);
            let r = na::Rotation3::face_towards(&axis, &up);

            // TODO:  also rotate based on char z_angle, and maybe pitch and roll ??
            // also use current bones, but that should be done to collision box, so that
            // we can also use it in collision. So this displaying should be done for now
            // since it is just a rendering of the vertices in col_box
            // dir and center should change when applying general rotation and keyframe bone rotation


            // p_t.to_homogeneous() * p_r.to_homogeneous() *
            let model_mat = bone_t.to_homogeneous() *  r.to_homogeneous() *  scale;

            let uniforms = shader::hitbox_shader::Uniforms {
                projection: scene.camera.projection(),
                view: scene.camera.view(),
                model: model_mat
            };

            hitbox_shader.shader.set_used();
            hitbox_shader.set_uniforms(uniforms);

            base_cube.render(gl);
        }

        /*
        // render skeleton collisionBoxes
        for col_box in &skel_col_boxes {

            let b = col_box.make_transformed(p1.pos, na::UnitQuaternion::<f32>::identity());

            let color = na::Vector3::new(1.0, 1.0, 0.0);

            /*
            TODO: Mybe try this just to see if our boxes are somewhat correct
            // goal is to make the follow the bones
             */

            let cube_model = cube::Cube::from_collision_box(col_box.clone(), color, gl);


            hitbox_shader.shader.set_used();

            let uniforms = shader::hitbox_shader::Uniforms {
                projection: scene.camera.projection(),
                view: scene.camera.view()
            };

            hitbox_shader.set_uniforms(uniforms);

            cube_model.render(gl);
        }*/



        if scene.ui.button("Reload") {
            reload_object_shader("mesh_shader", &gl, &mut scene.mesh_shader.shader);
            if let Some(ref mut fbos) = scene.fbos {
                reload_object_shader("postprocess", &gl, &mut fbos.post_process_shader.shader);
            }
            if let Some(ref mut stencil) = scene.stencil_shader {
                reload_object_shader("stencil", &gl, &mut stencil.shader);
            }

            reload_object_shader("hitbox", &gl, &mut hitbox_shader.shader);
        }


        if show_options {

            let ui = &mut scene.ui;

            show_options = !ui.window_begin("Options").closed;

            ui.body_text(&format!("fps: {:.0?}", 1.0/dt));
            ui.newline();

            ui.label("Sens");
            let inputs = scene.inputs.current_mut();
            ui.combo_box(&mut inputs.sens, 0.01, 1.0);
            ui.slider(&mut inputs.sens, 0.01, 1.0);

            ui.newline();

            ui.label("Speed");
            ui.combo_box(&mut inputs.speed, 0.01, 20.0);
            ui.slider(&mut inputs.speed, 0.01, 20.0);

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
        for (name, _) in animations.iter().sorted_by_key(|x| x.0) {
            if scene.ui.button(name) {
                scene.action_queue.push_back(scene::Action::StartAnimation(player_id, name.clone(), 0.0));
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
        let mut speed = scene.ui.deltatime.time_speed;
        if scene.ui.slider(&mut speed, 0.1, 1.3) {
            scene.ui.deltatime.time_speed = speed;
        }

        scene.ui.newline();

        if !show_options {
            show_options = scene.ui.button("Options");
        }

        scene.ui.newline();

        if scene.ui.button("Target") {
            //scene.controlled_entity.set_user_data(user);
        }

        if scene.ui.button("Reset Scene") {
            return Ok(());
        }

        // update aimaiton player, and bones, and root motion if any
        // and actions queue into the scene
        if playing {
            scene.update_actions();
            scene.update_animations();
        }

        scene.render();
        window.gl_swap_window();
    }

}


fn handle_input<A>(scene: &mut scene::Scene<A, ControlledData>) {
    if !scene.allow_char_inputs() {
        return;
    }

    let dt = scene.dt();
    if let Some(ref mut c_ent) = &mut scene.controlled_entity {
        let player = scene.entities.get_mut(&c_ent.id).unwrap();

        if scene.inputs.current().animation_expired {
            // maybe handle different depending on our state
            // default is to play idle
            c_ent.user_data.player = PlayerState::Movable;
            // this transition time is all good and danty, but a better version is contraints on joints
            // then the time will vary based on the differences in the key positions, so a almost identical position will result
            // in a low transition time
            // This could be calculated here and set as transition time. Just find the largest time for each bone
            // and use that
            scene.action_queue.push_back(scene::Action::StartAnimationLooped(c_ent.id, "idle".into(), 0.3));
        }

        match c_ent.user_data.player {
            PlayerState::Movable => {
                match c_ent.user_data.camera {
                    CameraState::Normal => handle_movement_regular(player, &scene.camera, &scene.inputs.current(), dt),
                    CameraState::Target((_, t)) => handle_movement_target(player, &t, &scene.inputs.current(), dt),
                }

                if scene.inputs.current().left_mouse {
                    // set attack state and start animation
                    c_ent.user_data.player = PlayerState::Attack;
                    scene.action_queue.push_back(scene::Action::StartAnimation(c_ent.id, "attack".into(), 0.0));
                    scene.action_queue.push_back(scene::Action::PlaySound("attack".into()));
                }
            },
            PlayerState::Attack => {
                // cannot attack again, or move
            }
        }
    }
}




fn handle_movement_regular(entity: &mut scene::SceneEntity, camera: &Camera, inputs: &Inputs, dt: f32) {

    // update player pos
    let mut forward = entity.pos - camera.pos;
    forward.z = 0.0;
    forward = forward.normalize();
    let tangent = V3::new(forward.y, -forward.x, 0.0);

    let mut m = forward * inputs.movement.x + tangent * inputs.movement.y;

    entity.forward_pitch = Rotation2::new(-0.2);
    entity.side_pitch = Rotation2::new(0.2);

    entity.velocity = V3::new(0.0, 0.0, 0.0);
    if m.magnitude() > 0.0 {

        m = m.normalize(); // check sekrio what happens when holding right or left
        let new_angle = m.y.atan2(m.x);
        entity.target_z_angle =  Rotation2::new(new_angle);
        entity.velocity = m * inputs.speed;
    }

    // TODO: Could be called a more general place
    update_from_conditions(entity, dt);
}

fn update_from_conditions(entity: &mut scene::SceneEntity, dt: f32) {
    // rotate to target facing dir
    angle_change(entity.target_z_angle.angle(), entity, dt);

    // update pos from vel
    entity.pos += entity.velocity * dt;

    //entity.forward_pitch = Rotation2::new(0.1 * entity.velocity.x );
    //entity.side_pitch = Rotation2::new(0.05 * entity.velocity.y );

}

fn handle_movement_target(entity: &mut scene::SceneEntity, target: &V3, inputs: &Inputs, dt: f32) {

    // face target
    let mut forward = target - entity.pos;
    forward.z = 0.0;
    forward = forward.normalize();

    let tangent = V3::new(forward.y, -forward.x, 0.0);

    let movement = forward * inputs.movement.x + tangent * inputs.movement.y;

    entity.velocity = V3::new(0.0, 0.0, 0.0);
    if movement.magnitude() > 0.0 {
        entity.velocity = movement.normalize() * inputs.speed;
    }


    if forward.magnitude() > 0.0 {
        let new_angle = forward.y.atan2(forward.x);
        entity.target_z_angle =  Rotation2::new(new_angle);
    }
    update_from_conditions(entity, dt);
}

/// when middle click update lock on/off, when lock on, update the target pos in user data, so we can use it
/// later in controller
fn update_target<T,>(scene: &mut scene::Scene<T, ControlledData>, _player_id: scene::EntityId, enemy_id: scene::EntityId) {

    let middle_mouse = scene.inputs.current().middle_mouse;

    if let Some(controlled) = &mut scene.controlled_entity {
        match controlled.user_data.camera {
            CameraState::Target((_id, _pos)) => {
                if middle_mouse {
                    controlled.user_data.camera = CameraState::Normal;
                } else {
                    if let Some(enemy) = scene.entities.get(&enemy_id) {
                        controlled.user_data.camera = CameraState::Target((enemy_id, enemy.pos));
                    }
               }
            },
            CameraState::Normal => {
                if middle_mouse {
                    if let Some(enemy) = scene.entities.get(&enemy_id) {
                        controlled.user_data.camera = CameraState::Target((enemy_id, enemy.pos));
                    }
                }
            }

        }
    }
}


struct ControlledData {
    camera: CameraState,
    player: PlayerState
}

enum CameraState {
    Normal,
    Target((scene::EntityId, V3))
}


fn camera_controller(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, user_data: &ControlledData) {

    match user_data.camera {
        CameraState::Normal => controller_regular(entity, camera, follow_camera, inputs, dt),
        CameraState::Target((_, t)) => controller_target(entity, camera, follow_camera, inputs, dt, &t),
    }
}


fn controller_target(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, target: &V3) {

    // face target
    let mut forward = target - entity.pos;
    forward.z = 0.0;
    forward = forward.normalize();

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

    follow_camera.update_dist(inputs.mouse_wheel);

    follow_camera.desired_pitch += inputs.mouse_movement.yrel * inputs.sens * inputs.inverse_y *  dt;

    follow_camera.yaw_change = inputs.mouse_movement.xrel * inputs.sens * dt ;

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
    let r_speed = 20.0;
    // change with max rotation speed
    let mut change = sign * r_speed * dt;

    // if we max speed would over shot target angle, change just the needed amount
    if change.abs() > diff.abs() {
        change = diff;
    }

    // do the update of rotation
    entity.z_angle *= Rotation2::new(change);
}

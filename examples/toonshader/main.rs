use gl_lib::{gl, helpers};
use gl_lib::scene_3d as scene;
use gl_lib::color::Color;
use gl_lib::typedef::V3;
use gl_lib::shader;
use gl_lib::scene_3d::EntityId;
use gl_lib::camera::{follow_camera, Camera};
use gl_lib::movement::Inputs;
use gl_lib::na::{Rotation2};
use gl_lib::scene_3d::actions;
use sdl2::event::Event;
use crate::systems::{missile, GameData, setup_systems, SystemFn, auto_attack};
use gl_lib::goap;

mod systems;


pub type TeamId = i8;
pub struct PostPData {
    time: f32
}

type Scene = scene::Scene::<PostPData, PlayerData>;

struct UiData {
    show_options: bool,
    wire_mode: bool
}

#[derive(Default)]
pub struct PlayerData {
    attacking: bool
}

pub struct Game {
    data: GameData,
    systems: Vec::<SystemFn>
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Unit {
    id: EntityId,
    // fields below could be Datamap/hashmap/vec to be a more struct of arrays instead of array of struct
    hp: f32,
    dead: bool,
    team: TeamId,
    range: f32,
}



fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    //let mut scene = scene::Scene::<PostPData>::new(sld_setugl.clone(), viewport, sdl_setup.ui(), sdl_setup.sdl)?;
    let mut scene = Scene::new(&mut sdl_setup)?;


    let game_data = GameData::default();

    let mut game = Game {
        data: game_data,
        systems: setup_systems(),
    };

    let mut ui_data = UiData {
        show_options: false,
        wire_mode: false,
    };


    setup(&mut scene, &mut game.data);
    spawn_enemy(&mut scene, &mut game.data);

    loop {

        scene.frame_start(&mut sdl_setup.event_pump);

        // could be system too??
        handle_input(&mut scene, &mut game.data);


        // GAME SYSTEMS
        for s in &game.systems {
            s(&mut game.data, &mut scene);
        }

        scene.render();

        // UI on top
        ui(&mut scene, &mut ui_data);

        scene.frame_end();
    }
}


fn setup(scene: &mut Scene, data: &mut GameData) {

    // SCENE MODELS AND RENDERING SETUP
    scene.load_all_meshes("examples/assets/blender_models/player.glb", true);
    scene.load_sound("attack".into(), &"examples/pixel_sekiro/assets/audio/deflect_1.wav");
    // setup default shader
    shader::reload_object_shader("toon_shader", &scene.gl, &mut scene.render_pipelines.default().mesh_shader.shader);
    scene.render_pipelines.default().use_stencil();


    // DAMAGE RENDERING SETUP
    let new_pipe = scene.render_pipelines.add("damage".into());
    new_pipe.clear_buffer_bits = gl::COLOR_BUFFER_BIT;
    new_pipe.shadow_map = None;
    shader::reload_object_shader("damage_shader", &scene.gl, &mut new_pipe.mesh_shader.shader);




    // PLAYER INITIAL SETUP
    let player_id = scene.create_entity("player");
    data.units.push(Unit { id: player_id, hp: 5.0, dead: false, team: 0, range: 10.0 });

    // start idle animation for player
    scene.action_queue.push_back(actions::Action::StartAnimationLooped(player_id, "t_pose".into(), 0.3));
    scene.controlled_entity = Some(scene::ControlledEntity {
        id: player_id,
        user_data: PlayerData::default(),
        control_fn: controller
    });



    // WORLD LAYOUT
    let _world_id = scene.create_entity("World");
    let rock_id = scene.create_entity("Rock");
    let sphere_id = scene.create_entity("Sphere");
    let sphere_1 = scene.create_entity("Sphere");

    scene::update_pos(scene, rock_id, V3::new(00.0, 5.0, 0.0));
    scene::update_pos(scene, sphere_id, V3::new(-1.0, 3.0, 3.0));
    scene::update_pos(scene, sphere_1, V3::new(1.0, -3.0, 1.0));


    scene.follow_controller.desired_distance = 15.0;


    // LIGHT
    // Light pos, clear color and player as controlled entity
    scene.light_pos = V3::new(-10.0, -5.0, 30.0);
    scene.ui.style.clear_color = Color::Rgb(100, 100, 100);
}


// used when we want to attach process in vs before loading models, so we can debug it.
// otherwise is is done before we can attach, so this just pauses the start of program
// unit the button is pressed
fn pre_load(scene: &mut Scene, sdl_setup: &mut helpers::BasicSetup) {

    loop {
        scene.frame_start(&mut sdl_setup.event_pump);

        if scene.ui.button("start") {
            return;
        }

        scene.frame_end();
    }
}


fn ui(scene: &mut Scene, data : &mut UiData) {
    if scene.ui.button("MeshShader") {
        shader::reload_object_shader("mesh_shader", &scene.gl, &mut scene.render_pipelines.default().mesh_shader.shader)
    }

    if scene.ui.button("ToonShader") {
        shader::reload_object_shader("toon_shader", &scene.gl, &mut scene.render_pipelines.default().mesh_shader.shader)
    }

    if scene.ui.button("use stencil") {

        let default_pipeline = scene.render_pipelines.default();
        println!("stencil use pressed {:?}", default_pipeline.stencil_shader.is_some());
        if default_pipeline.stencil_shader.is_some() {
            default_pipeline.stencil_shader = None;
        } else {
            default_pipeline.use_stencil();
        }
    }

    if scene.render_pipelines.default().stencil_shader.is_some() && scene.ui.button("stencil") {
        shader::reload_object_shader("stencil", &scene.gl, &mut scene.render_pipelines.default().stencil_shader.as_mut().unwrap().shader)
    }

    if data.show_options {
        options(scene, data);
    } else {
        data.show_options = scene.ui.button("Options");
    }

    if scene.ui.button("Reload") {
        scene.load_all_meshes("examples/assets/blender_models/player.glb", true);
    }

}


fn options(scene: &mut Scene, data : &mut UiData) {

    let ui = &mut scene.ui;

    data.show_options = !ui.window_begin("Options").closed;

    ui.label("Sens");
    let inputs = scene.inputs.current_mut();
    ui.combo_box(&mut inputs.sens, 0.01, 1.0);
    ui.slider(&mut inputs.sens, 0.01, 1.0);

    ui.newline();

    ui.label("Speed");
    ui.combo_box(&mut inputs.speed, 0.01, 20.0);
    ui.slider(&mut inputs.speed, 0.01, 20.0);


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
    ui.slider(&mut scene.light_pos.z, 0.0, 100.0 );

    ui.newline();
    ui.color_picker(&mut scene.light_color);

    ui.newline();
    if ui.button("Wire Mode") {
        data.wire_mode = !data.wire_mode;

        unsafe {
            if data.wire_mode {
                scene.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
            else {
                scene.gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }

    let default_pipeline = scene.render_pipelines.default();
    if let Some(sm) = &mut default_pipeline.shadow_map {
        ui.newline();
        ui.body_text(&format!("z_near: {:.2?}, z_far: {:.2?}", sm.z_near, sm.z_far));

        ui.newline();
        ui.body_text("z_near:");
        ui.slider(&mut sm.z_near, 0.0, 10.0);

        ui.newline();
        ui.body_text("z_far:");
        ui.slider(&mut sm.z_far, 0.0, 50.0);

        ui.newline();
        ui.body_text("size:");
        ui.slider(&mut sm.size, 1.0, 50.0);
    }


    ui.window_end("Options");

}



fn controller(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, _user_data: &PlayerData) {

    // update entity.pos
    let m = inputs.movement;
    entity.pos += dt * inputs.speed * V3::new(-m.x, m.y, 0.0);


    // update camera
    let new_angle = m.y.atan2(-m.x);

    if m.magnitude() > 0.0 {
        entity.z_angle =  Rotation2::new(new_angle);
    }

    let scroll_speed = 20.0;
    follow_camera.desired_distance += dt * inputs.mouse_wheel * scroll_speed;


    // limit dist to [5.0; 25.0]
    follow_camera.desired_distance = follow_camera.desired_distance.max(5.0).min(25.0);

    let offset_dir = V3::new(10.0, -3.5, 12.0).normalize();

    camera.pos = entity.pos + offset_dir * follow_camera.desired_distance;
    camera.look_at(entity.pos);
}


// should this be in controlled entity controller_fn? Just requried us to also pass action_queue, for now
// taking a scene here is the most "free" since we can look at enemies, use camera ect.
fn handle_input(scene: &mut Scene, game: &mut GameData) {

    // commands like R to spawn new enemy
    // TODO: Find a way to not clone here
    for e in &scene.ui.frame_events.clone() {
        match e {
            Event::KeyUp{keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
                spawn_enemy(scene, game);
            },
            _ => {},

        }
    }


    if !scene.allow_char_inputs() {
        return;
    }

    if let Some(ref mut c_ent) = &mut scene.controlled_entity {
        let player_id = c_ent.id;
        let player = match scene.entities.get_mut(&c_ent.id) {
            Some(p) => p,
            None => {return;}
        };

        let player_data = &mut c_ent.user_data;

        let player_pos = player.pos;
        // play idle
        if scene.inputs.current().animation_expired {
            player_data.attacking = false;
            scene.action_queue.push_back(actions::Action::StartAnimationLooped(c_ent.id, "idle".into(), 0.3));
        }



        let player_unit = game.units.iter().filter(|u| u.id == player_id).nth(0).unwrap();
        if !player_data.attacking && scene.inputs.current().left_mouse {

            if let Some(enemy) = auto_attack::find_closest_enemy(player_unit, game, &scene.entities) {

                // set attack state and start animation
                player_data.attacking = true;
                scene.action_queue.push_back(actions::Action::StartAnimation(c_ent.id, "attack".into(), 0.0));
                scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));


                // find enemy
                if game.units.len() > 0 {

                    let enemy = &game.units[0];

                    // spawn an arrow that homes to enemy
                    let id = scene.create_entity("arrow");
                    scene::update_pos(scene, id, player_pos + V3::new(0.0, 0.0, 1.0));
                    game.missiles.push(missile::Missile {id, target_id: enemy.id });
                }
            }
        }
    }
}



fn spawn_enemy(scene: &mut Scene, game_data: &mut GameData) {

    let enemy_id = scene.create_entity("enemy");

    scene::update_pos(scene, enemy_id, V3::new(5.0, 5.0, 0.0));

    scene.action_queue.push_back(actions::Action::StartAnimationLooped(enemy_id, "dance".into(), 0.3));

    game_data.units.push(Unit { id: enemy_id, hp: 5.0, dead: false, team: 1, range: 5.0 });
}

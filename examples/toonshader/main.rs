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
use gl_lib::goap;
use std::fs;
use std::rc::Rc;
use crate::systems::{goap_ai, missile, GameData, setup_systems, SystemFn, auto_attack, unit};
use crate::systems::goap_ai::GoapSystem;
use crate::ui::{UiData, ui};

mod systems;
mod ui;

pub struct PostPData {
    time: f32
}

type Scene = scene::Scene::<PostPData, ()>;

pub struct Game {
    data: GameData,
    systems: Vec::<SystemFn>,
    actions: Rc::<goap::Actions>,
    goals: Rc::<goap::Goals>,
    paused: bool

}


fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    //let mut scene = scene::Scene::<PostPData>::new(sld_setugl.clone(), viewport, sdl_setup.ui(), sdl_setup.sdl)?;
    let mut scene = Scene::new(&mut sdl_setup)?;


    let mut game_data = GameData::default();
    game_data.goap_action_to_fn.insert("GoToTarget".into(), goap_ai::action_functions::go_to_target);
    game_data.goap_action_to_fn.insert("AcquireTarget".into(), goap_ai::action_functions::acquire_target);
    game_data.goap_action_to_fn.insert("Attack".into(), goap_ai::action_functions::attack);

    let goals : Rc::<goap::Goals> = load_goals().unwrap().into();
    let actions : Rc::<goap::Actions>= load_actions().unwrap().into();

    let mut game = Game {
        data: game_data,
        systems: setup_systems(),
        goals,
        actions,
        paused: true
    };

    let mut ui_data = ui::UiData {
        show_options: false,
        show_goap: true,
        show_player_data: true,
        wire_mode: false,
    };

    setup(&mut scene, &mut game.data);
    let enemy_id = spawn_enemy(&mut scene, &mut game);

    loop {
        scene.frame_start(&mut sdl_setup.event_pump);

        // could be system too??
        handle_input(&mut scene, &mut game);

        if !game.paused {
            // GAME SYSTEMS
            for s in &game.systems {
                s(&mut game.data, &mut scene);
            }
        }

        scene.render();

        // UI on top
        ui(&mut scene, &mut ui_data, &mut game);

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
    data.units.push(unit::Unit { id: player_id} );
    data.units_data.insert(player_id, unit::UnitData {id: player_id,  hp: 5.0, dead: false, team: 0, range: 10.0, cooldown: 0.0 });

    // start idle animation for player
    scene.action_queue.push_back(actions::Action::StartAnimationLooped(player_id, "t_pose".into(), 0.3));
    scene.controlled_entity = Some(scene::ControlledEntity {
        id: player_id,
        user_data: (),
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




fn controller(entity: &mut scene::SceneEntity, camera: &mut Camera, follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, _user_data: &()) {

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
fn handle_input(scene: &mut Scene, game: &mut Game) {

    // commands like R to spawn new enemy
    // TODO: Find a way to not clone here
    for e in &scene.ui.frame_events.clone() {
        match e {
            Event::KeyUp{keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
                spawn_enemy(scene, game);
            },
            Event::KeyUp{keycode: Some(sdl2::keyboard::Keycode::P), .. } => {
                game.paused = ! game.paused;
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
            scene.action_queue.push_back(actions::Action::StartAnimationLooped(c_ent.id, "idle".into(), 0.3));
        }




        if scene.inputs.current().left_mouse {
            if let Some(enemy) = auto_attack::find_closest_enemy(player_id, &mut game.data, &scene.entities) {
                let player_unit = game.data.units_data.get_mut(&player_id).unwrap();


                if player_unit.cooldown <= 0.0 && enemy.dist < player_unit.range {
                    // set attack state and start animation
                    scene.action_queue.push_back(actions::Action::StartAnimation(c_ent.id, "attack".into(), 0.0));
                    scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));

                    player_unit.cooldown = 1.0;
                    // spawn an arrow that homes to enemy
                    let id = scene.create_entity("arrow");
                    scene::update_pos(scene, id, player_pos + V3::new(0.0, 0.0, 1.0));
                    game.data.missiles.push(missile::Missile {id, target_id: enemy.target });
                }
            }
        }
    }
}


fn spawn_enemy(scene: &mut Scene, game: &mut Game) -> EntityId {

    let enemy_id = scene.create_entity("enemy");

    scene::update_pos(scene, enemy_id, V3::new(10.0, 10.0, 0.0));

    game.data.units.push(unit::Unit { id: enemy_id});
    game.data.units_data.insert(enemy_id, unit::UnitData {id: enemy_id, hp: 5.0, dead: false, team: 1, range: 5.0, cooldown: 0.0 });

    game.data.goap_datas.push(goap_ai::GoapData::new(enemy_id, game.goals.clone(), game.actions.clone()));


    enemy_id
}



fn load_goals() -> Result::<goap::Goals, toml::de::Error> {
    let goal_str = fs::read_to_string("examples/toonshader/goals.toml").unwrap();
    toml::from_str(&goal_str)
}


fn load_actions() -> Result::<goap::Actions, toml::de::Error> {
    let action_str = fs::read_to_string("examples/toonshader/actions.toml").unwrap();
    toml::from_str(&action_str)
}

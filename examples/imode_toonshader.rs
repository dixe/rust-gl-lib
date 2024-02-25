use gl_lib::{gl, helpers};



use gl_lib::scene_3d as scene;
use gl_lib::color::Color;
use gl_lib::typedef::V3;
use gl_lib::shader;
use gl_lib::scene_3d::EntityId;
use gl_lib::camera::{follow_camera, Camera};
use gl_lib::movement::Inputs;
use gl_lib::na::{Rotation2};

pub struct PostPData {
    time: f32
}

type Scene = scene::Scene::<PostPData, PlayerData>;

struct Data {
    light_id: EntityId,
    show_options: bool,
    wire_mode: bool
}


#[derive(Default)]
struct PlayerData {
    attacking: bool
        //state: PlayerState
}

#[derive(Default)]
enum PlayerState {
    Attack,
    #[default]
    Movable,
}


fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    //let mut scene = scene::Scene::<PostPData>::new(sld_setugl.clone(), viewport, sdl_setup.ui(), sdl_setup.sdl)?;
    let mut scene = Scene::new(&mut sdl_setup)?;

    scene.load_all_meshes("examples/assets/blender_models/player.glb", true);
    scene.load_sound("attack".into(), &"examples/pixel_sekiro/assets/audio/deflect_1.wav");

    shader::reload_object_shader("toon_shader", &scene.gl, &mut scene.mesh_shader.shader);

    let player_id = scene.create_entity("player");
    let _world_id = scene.create_entity("World");

    let rock_id = scene.create_entity("Rock");
    let sphere_id = scene.create_entity("Sphere");
    let sphere_1 = scene.create_entity("Sphere");
    let light_id = scene.create_entity("Light");

    let enemy_id = scene.create_entity("enemy");

    scene.light_pos = V3::new(-10.0, -5.0, 30.0);

    scene.ui.style.clear_color = Color::Rgb(100, 100, 100);

    scene.controlled_entity = Some(scene::ControlledEntity {
        id: player_id,
        user_data: PlayerData::default(),
        control_fn: controller
    });

    scene::update_pos(&mut scene, rock_id, V3::new(00.0, 5.0, 0.0));
    scene::update_pos(&mut scene, sphere_id, V3::new(-1.0, 3.0, 3.0));
    scene::update_pos(&mut scene, sphere_1, V3::new(1.0, -3.0, 1.0));

    scene::update_pos(&mut scene, enemy_id, V3::new(5.0, 5.0, 0.0));



    let lp = V3::new(1.0, -4.0, 3.0);
    scene::update_pos(&mut scene, light_id, lp);

    scene.use_stencil();

    let mut data = Data {
        show_options: false,
        wire_mode: false,
        light_id
    };


    // start idle animation for player
    scene.action_queue.push_back(scene::Action::StartAnimationLooped(player_id, "idle".into(), 0.3));

    let mut game = Game::default();
    game.enemies.push(enemy_id);


    loop {

        scene.frame_start(&mut sdl_setup.event_pump);

        handle_input(&mut scene, &mut game);


        // GAME SYSTEMS

        missile_system(&mut game.missiles, &mut scene);



        scene.render();

        // UI on top
        ui(&mut scene, &mut data);

        scene.frame_end();
    }
}


fn ui(scene: &mut Scene, data : &mut Data) {
    if scene.ui.button("MeshShader") {
        shader::reload_object_shader("mesh_shader", &scene.gl, &mut scene.mesh_shader.shader)
    }

    if scene.ui.button("ToonShader") {
        shader::reload_object_shader("toon_shader", &scene.gl, &mut scene.mesh_shader.shader)
    }

    if scene.ui.button("use stencil") {
        if scene.stencil_shader.is_some() {
            scene.stencil_shader = None;
        } else {
            scene.use_stencil();
        }
    }

    if scene.stencil_shader.is_some() && scene.ui.button("stencil") {
        shader::reload_object_shader("stencil", &scene.gl, &mut scene.stencil_shader.as_mut().unwrap().shader)
    }

    if data.show_options {
        options(scene, data);
    } else {
        data.show_options = scene.ui.button("Options");
    }
}


fn options(scene: &mut Scene, data : &mut Data) {

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


    if let Some(sm) = &mut scene.shadow_map {
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

    // update light cube to follow the light
    let _lp = scene.light_pos + V3::new(0.0, 0.0, 2.0);
    //update_pos(scene, data.light_id, lp);

}



fn controller(entity: &mut scene::SceneEntity, camera: &mut Camera, _follow_camera: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, _user_data: &PlayerData) {

    // update entity.pos
    let m = inputs.movement;
    entity.pos += dt * inputs.speed * V3::new(-m.x, m.y, 0.0);


    // update camera
    let new_angle = m.y.atan2(-m.x);

    if m.magnitude() > 0.0 {
        entity.z_angle =  Rotation2::new(new_angle);
    }

    let offset = V3::new(10.0, -3.5, 12.0);
    camera.pos = entity.pos + offset;
    camera.look_at(entity.pos);
}


// should this be in controlled entity controller_fn? Just requried us to also pass action_queue, for now
// taking a scene here is the most "free" since we can look at enemies, use camera ect.
fn handle_input(scene: &mut Scene, game: &mut Game) {
    if !scene.allow_char_inputs() {
        return;
    }

    let _dt = scene.dt();
    if let Some(ref mut c_ent) = &mut scene.controlled_entity {
        let player = scene.entities.get_mut(&c_ent.id).unwrap();
        let player_data = &mut c_ent.user_data;

        let player_pos = player.pos;
        // play idle
        if scene.inputs.current().animation_expired {
            player_data.attacking = false;
            scene.action_queue.push_back(scene::Action::StartAnimationLooped(c_ent.id, "idle".into(), 0.3));
        }


        if !player_data.attacking && scene.inputs.current().left_mouse {
            // set attack state and start animation
            player_data.attacking = true;
            scene.action_queue.push_back(scene::Action::StartAnimation(c_ent.id, "attack".into(), 0.0));
            scene.action_queue.push_back(scene::Action::PlaySound("attack".into()));


            // find enemy
            if game.enemies.len() > 0 {

                let enemy_id = game.enemies[0];

                // spawn an arrow that homes to enemy
                let id = scene.create_entity("arrow");
                scene::update_pos(scene, id, player_pos + V3::new(0.0, 0.0, 1.0));
                game.missiles.push(Missile {id, target_id: enemy_id });

            }
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    enemies: Vec::<EntityId>,
    missiles: Vec::<Missile>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Missile {
    id: EntityId,
    target_id: EntityId,
}


fn missile_system(missiles: &mut Vec::<Missile>, scene: &mut Scene) {
    let speed = 20.0;
    let dt = scene.dt();
    let mut i = 0;
    while i < missiles.len() { // use while loop so we can modify missiles during loop

        let m = missiles[i];

        let missile = scene.entities.get(&m.id).unwrap();
        // TODO: this can fail, if the target is dead and gone
        let target = scene.entities.get(&m.target_id).unwrap();

        let dir = target.pos - missile.pos;

        let new_p = missile.pos + dir.normalize() * speed * dt;

        scene::update_dir(scene, m.id, dir);
        scene::update_pos(scene, m.id, new_p);

        // fake some collision
        if dir.xy().magnitude() < 0.2 {
            // remove from missiles
            missiles.swap_remove(i);
            // remove from scene
            scene.remove_entity(&m.id);
        } else {
            i += 1;
        }
    }
}

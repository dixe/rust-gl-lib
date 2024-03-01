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
use gl_lib::scene_3d::RenderPipeline;
use gl_lib::scene_3d::ParticleScene;


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

fn spawn_enemy(scene: &mut Scene, game_data: &mut GameData) {

    let enemy_id = scene.create_entity("enemy");

    scene::update_pos(scene, enemy_id, V3::new(5.0, 5.0, 0.0));

    scene.action_queue.push_back(actions::Action::StartAnimationLooped(enemy_id, "dance".into(), 0.3));

    game_data.enemies.push(Unit { id: enemy_id, hp: 5.0, dead: false });
}

fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    //let mut scene = scene::Scene::<PostPData>::new(sld_setugl.clone(), viewport, sdl_setup.ui(), sdl_setup.sdl)?;
    let mut scene = Scene::new(&mut sdl_setup)?;

    scene.load_all_meshes("examples/assets/blender_models/player.glb", true);
    scene.load_sound("attack".into(), &"examples/pixel_sekiro/assets/audio/deflect_1.wav");

    shader::reload_object_shader("toon_shader", &scene.gl, &mut scene.render_pipelines.default().mesh_shader.shader);

    let new_pipe = scene.render_pipelines.add("damage".into());
    new_pipe.clear_buffer_bits = gl::COLOR_BUFFER_BIT;
    new_pipe.shadow_map = None;

    shader::reload_object_shader("damage_shader", &scene.gl, &mut new_pipe.mesh_shader.shader);


    let player_id = scene.create_entity("player");
    let _world_id = scene.create_entity("World");

    let rock_id = scene.create_entity("Rock");
    let sphere_id = scene.create_entity("Sphere");
    let sphere_1 = scene.create_entity("Sphere");
    let light_id = scene.create_entity("Light");


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

    let lp = V3::new(1.0, -4.0, 3.0);
    scene::update_pos(&mut scene, light_id, lp);


    scene.follow_controller.desired_distance = 15.0;
    scene.render_pipelines.default().use_stencil();

    let mut data = Data {
        show_options: false,
        wire_mode: false,
        light_id
    };


    // start idle animation for player
    scene.action_queue.push_back(actions::Action::StartAnimationLooped(player_id, "t_pose".into(), 0.3));


    let game_data = GameData::default();


    let mut game = Game {
        data: game_data,
        systems: vec![death_system, missile_system]
    };

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
        ui(&mut scene, &mut data);

        scene.frame_end();
    }
}

fn pre_load(scene: &mut Scene, sdl_setup: &mut helpers::BasicSetup) {

    loop {
        scene.frame_start(&mut sdl_setup.event_pump);

        if scene.ui.button("start") {
            return;
        }


        scene.frame_end();
    }
}


fn ui(scene: &mut Scene, data : &mut Data) {
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

    // update light cube to follow the light
    let _lp = scene.light_pos + V3::new(0.0, 0.0, 2.0);
    //update_pos(scene, data.light_id, lp);

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

    let _dt = scene.dt();
    if let Some(ref mut c_ent) = &mut scene.controlled_entity {
        let player = scene.entities.get_mut(&c_ent.id).unwrap();
        let _player_id = c_ent.id;
        let player_data = &mut c_ent.user_data;

        let player_pos = player.pos;
        // play idle
        if scene.inputs.current().animation_expired {
            player_data.attacking = false;
            scene.action_queue.push_back(actions::Action::StartAnimationLooped(c_ent.id, "idle".into(), 0.3));
        }


        if !player_data.attacking && scene.inputs.current().left_mouse {
            // set attack state and start animation
            player_data.attacking = true;
            scene.action_queue.push_back(actions::Action::StartAnimation(c_ent.id, "attack".into(), 0.0));
            scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));


            // find enemy
            if game.enemies.len() > 0 {

                let enemy = &game.enemies[0];

                // spawn an arrow that homes to enemy
                let id = scene.create_entity("arrow");
                scene::update_pos(scene, id, player_pos + V3::new(0.0, 0.0, 1.0));
                game.missiles.push(Missile {id, target_id: enemy.id });
            }
        }
    }


}

type SystemFn = fn(&mut GameData, &mut Scene);

struct Game {
    data: GameData,
    systems: Vec::<SystemFn>
}

#[derive(Debug, Default)]
struct Unit {
    id: EntityId,
    hp: f32,
    dead: bool
}

#[derive(Debug, Default)]
struct GameData {
    enemies: Vec::<Unit>,
    missiles: Vec::<Missile>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Missile {
    id: EntityId,
    target_id: EntityId,
}


trait MissileSystem {
    /// Return number of missiles used for loop
    fn missiles(&self) -> usize;

    /// Return mut missile for given index in loop
    fn missile(&mut self, idx: usize) -> &mut Missile; // should be some kind of missile trait

    /// Call impl for on hit for given missile idx, return bool indicating whether the missile was remove or not.
    /// Used to continue loop correctly
    fn on_missile_hit(&mut self, idx: usize, scene: &mut Scene) -> bool;
}


impl MissileSystem for GameData {
    fn missiles(&self) -> usize {
        self.missiles.len()
    }

    fn missile(&mut self, idx: usize) -> &mut  Missile {
        self.missiles.get_mut(idx).expect("Death system should not have called with idx outside scope")
    }

    fn on_missile_hit(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let m = self.missiles[idx];

        self.missiles.swap_remove(idx);

        // remove from scene
        scene.remove_entity(&m.id);


        // apply damage, if enemy dies, it will get handled by the death system.
        if let Some(enemy) = self.enemies.iter_mut().find(|e| e.id == m.target_id) {
            enemy.hp -= 1.0;
        }

        // damage particle on enemy on hit
        if let Some(target) = scene.entities.get_mut(&m.target_id) {
            // damage mesh
            scene.emitter.emit_new(ParticleScene {
                life: 0.3,
                total_life: 0.3,
                pos: target.pos,
                mesh_id: *scene.meshes.get("Damage".into()).unwrap(),
                render_pipeline_id: 1
            });
        }

        // maybe a missile bounces to next target,
        // and we decrement bounce life, and then
        // we return false unti bounces or not target reached.
        // For now just true since we remove from missiles
        true
    }
}

fn missile_system(game: &mut impl MissileSystem, scene: &mut Scene) {
    let speed = 20.0;
    let dt = scene.dt();
    let mut i = 0;
    while i < game.missiles() { // use while loop so we can modify during loop

        let m = game.missile(i);

        let missile = scene.entities.get(&m.id).unwrap();
        // TODO: this can fail, if the target is dead and gone
        if let Some(target) = scene.entities.get(&m.target_id) {


            let dir = target.pos - missile.pos;

            let new_p = missile.pos + dir.normalize() * speed * dt;

            scene::update_dir(scene, m.id, dir);
            scene::update_pos(scene, m.id, new_p);

            // fake some collision, maybe have missile system call back to impl for hit
            let mut update = true;
            if dir.xy().magnitude() < 0.2 {
                update = game.on_missile_hit(i, scene);
            }
            if update {
                i += 1;
            }
        } else {

            i += 1;
            // remove missile;
        }
    }
}




trait DeathSystem {
    fn units(&self) -> usize;
    fn unit(&mut self, idx: usize) -> &mut Unit; // should be some kind of Hp trait impl
    fn on_death(&mut self, idx: usize, scene: &mut Scene) -> bool;
    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool;
}


fn death_system(game: &mut impl DeathSystem, scene: &mut Scene) {

    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let unit = game.unit(i);

        let mut update = true;

        // if alive and get lower than 0 hp play dead anim
        if !unit.dead && unit.hp <= 0.0 {
            update = game.on_death(i, scene);
        } else if unit.dead {
            update = game.update_dead(i, scene);
        }

        if update {
            i += 1;
        }
    }
}


impl DeathSystem for GameData {
    fn units(&self) -> usize {
        self.enemies.len()
    }

    fn unit(&mut self, idx: usize) -> &mut Unit {
        self.enemies.get_mut(idx).expect("Death system should not have called with idx outside scope")
    }

    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let unit = self.enemies.get_mut(idx).expect("Death system should not have called with idx outside scope");

        if scene.player.expired(&unit.id) {
            // remove unit
            scene.remove_entity(&unit.id);
            self.enemies.swap_remove(idx);
            return false;
        }

        true
    }


    fn on_death(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let unit = self.enemies.get_mut(idx).expect("Death system should not have called with idx outside scope");

        // set dead
        unit.dead = true;
        // start death anim
        scene.action_queue.push_back(actions::Action::StartAnimation(unit.id, "death".into(), 0.0));
        true
    }
}

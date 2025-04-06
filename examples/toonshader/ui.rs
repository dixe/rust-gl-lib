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
use crate::{Game, Scene};
use gl_lib::imode_gui::Ui;



pub struct UiData {
    pub show_options: bool,
    pub show_goap: bool,
    pub wire_mode: bool,
    pub show_player_data: bool
}


pub fn ui(scene: &mut Scene, data : &mut UiData, game: &mut Game) {

    if scene.ui.button("Play/pause") {
        game.paused = ! game.paused;
    }

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

    if data.show_goap {
        show_goap(scene, data, game);
    } else {
        data.show_goap = scene.ui.button("Show Goap");
    }


    if data.show_player_data {
        show_player(scene, data, game);
    } else {
        data.show_player_data = scene.ui.button("Show Player Data");
    }


    if scene.ui.button("Reload") {
        scene.load_all_meshes("examples/assets/blender_models/player.glb", true);
    }

}



fn show_player(scene: &mut Scene, data : &mut UiData, game: &mut Game) {
    let ui = &mut scene.ui;
    data.show_player_data = !ui.window_begin("Player Unit Data").closed;

    if let Some(ref mut controlled_entity) = &mut scene.controlled_entity {
        let player_id = controlled_entity.id;

        let unit_data = game.data.units_data.get(&player_id).unwrap();

        show_unit_data(ui, &unit_data );

    }

    ui.window_end("Player Unit Data");

}


fn show_unit_data(ui: &mut Ui, unit_data: &unit::UnitData ){
    ui.label("Hp:");
    ui.body_text(&format!("{:?}", unit_data.hp));
    ui.newline();

    ui.label("Cooldown:");
    ui.body_text(&format!("{:?}", unit_data.cooldown));
    ui.newline();

    ui.label("Dead:");
    ui.body_text(&format!("{:?}", unit_data.dead));
    ui.newline();

}


fn show_goap(scene: &mut Scene, data : &mut UiData, game: &mut Game) {
    let ui = &mut scene.ui;
    data.show_goap = !ui.window_begin("Show Goap").closed;

    // find enemy_id


    // could be a loop that just breaks in 1 iter
    if let Some((enemy_id, enemy_unit)) = game.data.units_data.iter().filter(|(id, x)| x.team == 1).nth(0) {
        if let Some(goap) = game.data.goap_data_by_entity_id(*enemy_id) {

            ui.label("Goal:");
            ui.body_text(&format!("{:?}", goap.goal.as_ref().map(|g| g.name.clone())));
            ui.newline();


            ui.label("Plan:");

            let mut plan_str : Vec<Rc::<str>> = goap.plan.iter().map(|a| a.name.clone()).collect();
            plan_str.reverse();
            ui.body_text(&format!("{:?}", plan_str));
            ui.newline();



            // show all true/false states
            for (k, v) in &goap.state {
                ui.label(k);
                ui.body_text(&format!(": {:?}", v));
                ui.newline();
            }


            show_unit_data(ui, enemy_unit);

        }
    }

    ui.window_end("Show Goap");

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

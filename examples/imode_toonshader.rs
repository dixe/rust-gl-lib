use gl_lib::{gl, ScreenBox, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::text_rendering::text_renderer::TextAlignment;
use gl_lib::scene_3d as scene;
use gl_lib::color::Color;
use gl_lib::typedef::V3;
use gl_lib::shader;
use gl_lib::scene_3d::EntityId;


pub struct PostPData {
    time: f32
}

type Scene = scene::Scene::<PostPData>;

fn update_pos(scene: &mut Scene, id: EntityId, pos: V3) {
    if let Some(c) = scene.entities.get_mut(&id) {
        c.pos = pos;
    }
}

struct Data {
    light_id: EntityId,
    show_options: bool,
    wire_mode: bool
}

fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
    //let mut scene = scene::Scene::<PostPData>::new(sld_setugl.clone(), viewport, sdl_setup.ui(), sdl_setup.sdl)?;
    let mut scene = scene::Scene::<PostPData>::new(&mut sdl_setup)?;

    scene.load_all_meshes("examples/assets/blender_models/player.glb", true);


    let _player_id = scene.create_entity("player");
    let _world_id = scene.create_entity("World");

    let rock_id = scene.create_entity("Rock");
    let sphere_id = scene.create_entity("Sphere");
    let sphere_1 = scene.create_entity("Sphere");
    let light_id = scene.create_entity("Light");


    scene.light_pos = V3::new(-1.0, -5.0, 30.0);

    scene.ui.style.clear_color = Color::Rgb(100, 100, 100);

    update_pos(&mut scene, rock_id, V3::new(00.0, 5.0, 0.0));
    update_pos(&mut scene, sphere_id, V3::new(-1.0, 3.0, 3.0));
    update_pos(&mut scene, sphere_1, V3::new(1.0, -3.0, 1.0));
    let lp = V3::new(1.0, -4.0, 3.0);
    update_pos(&mut scene, light_id, lp);

    scene.use_stencil();

    let mut data = Data {
        show_options: false,
        wire_mode: false,
        light_id
    };

    loop {

        scene.frame_start(&mut sdl_setup.event_pump);

        scene.render();

        // UI on top
        ui(&mut scene, &mut data);


        scene.frame_end();
    }
}


fn ui(scene: &mut scene::Scene::<PostPData>, data : &mut Data) {
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


fn options(scene: &mut scene::Scene::<PostPData>, data : &mut Data) {

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
    let lp = scene.light_pos + V3::new(0.0, 0.0, 2.0);;
    //update_pos(scene, data.light_id, lp);

}

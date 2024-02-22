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


    scene.light_pos = V3::new(1.0, 5.0, 100.0);

    scene.ui.style.clear_color = Color::Rgb(100, 100, 100);

    update_pos(&mut scene, rock_id, V3::new(00.0, 5.0, 0.0));
    update_pos(&mut scene, sphere_id, V3::new(-1.0, 3.0, 3.0));
    update_pos(&mut scene, sphere_1, V3::new(1.0, -3.0, 1.0));
    let lp = scene.light_pos + V3::new(0.0, 0.0, 5.0);;
    update_pos(&mut scene, light_id, lp);

    scene.use_stencil();
    loop {

        scene.frame_start(&mut sdl_setup.event_pump);



        scene.render();


        // UI on top
        ui(&mut scene);


        scene.frame_end();
    }
}


fn ui(scene: &mut scene::Scene::<PostPData>) {
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

    if scene.ui.button("stencil") {
        shader::reload_object_shader("stencil", &scene.gl, &mut scene.stencil_shader.as_mut().unwrap().shader)
    }
}

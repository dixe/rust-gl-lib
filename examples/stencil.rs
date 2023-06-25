use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer};
use gl_lib::objects::gltf_mesh::{self, Animation};
use gl_lib::shader::{self, mesh_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cube};
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Translation3};


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let _audio_subsystem = sdl.audio().unwrap();
    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();

    println!("{:?}", viewport);
    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let glp_path = "E:/repos/Game-in-rust/blender_models/Animation_test.glb";
    //let glp_path = "E:/repos/Game-in-rust/blender_models/enemy1.glb";

    let gltf_data = gltf_mesh::meshes_from_gltf(glp_path)?;

    let mut shader = mesh_shader::MeshShader::new(&gl)?;

    let mesh_name = "Cube";
    let mesh = gltf_data.meshes.get_mesh(&gl, &mesh_name).unwrap();

    let skin_id = gltf_data.skins.mesh_to_skin.get(mesh_name).unwrap();

    let mut skeleton: Skeleton = gltf_data.skins.skeletons.get(&skin_id).unwrap().clone();

    let mut bones = skeleton.create_bones();

    let mut camera = camera::Camera::new(viewport.w as f32, viewport.h as f32);

    let mut controller = free_camera::Controller::default();



    let mut stencil_shader = shader.clone();
    let vert = include_str!("../assets/shaders/objects/stencil.vert");
    let frag = include_str!("../assets/shaders/objects/stencil.frag");

    stencil_shader.shader = shader::BaseShader::new(&gl, vert, frag).unwrap();



    let _cube = cube::Cube::new(&gl);
    camera.move_to(V3::new(8.4, 4.3, 5.0));
    let la = V3::new(5.0, 3.1, 5.0);
    camera.look_at(la);

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        gl.Enable(gl::STENCIL_TEST);
        gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
        gl.StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
    }

    let mut player = AnimationPlayer::new();

    let mut anim_id = 0;
    let mut playing = true;

    let _t : f32 = 0.0;

    let mut s = 1.0;
    let _animation : Option<&Animation> = None;
    loop {
        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);

        let dt = ui.dt();

        for event in &ui.frame_events {
            controller.update_events(event);
        }

        controller.update_camera(&mut camera, dt);

        if ui.button("Play/Pause") {
            playing = !playing;
        }

        if ui.button("Reload") {
            reload_stencil_shader(&mut stencil_shader);
            reload_mesh_shader(&mut shader);
        }

        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if ui.button(name) {
                    player.remove(anim_id);
                    anim_id = player.start(Start {anim: anim.clone(), repeat: true});
                }
            }
        }

        ui.slider(&mut s, 0.0, 1.5);
        if ui.button("Reset") {
            s = 1.0;
        }
        // update aimaiton player, and bones
        if playing {
            player.update(dt);
        }

        // update skeleton and bones from animation, if animation done, nothing is updated
        player.update_skeleton(anim_id, &mut skeleton);
        skeleton.set_all_bones_from_skeleton(&mut bones);

        draw(&gl, &camera, &bones, &shader, &stencil_shader, &mesh, s);


        window.gl_swap_window();
    }
}

fn reload_stencil_shader(shader: &mut mesh_shader::MeshShader) {
    let vert_shader_path = std::path::Path::new("E:/repos/rust-gl-lib/assets/shaders/objects/stencil.vert");
    let vert_source = std::fs::read_to_string(vert_shader_path.clone())
        .expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new("E:/repos/rust-gl-lib/assets/shaders/objects/stencil.frag");
    let frag_source = std::fs::read_to_string(frag_shader_path.clone())
        .expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    match shader::BaseShader::new(shader.shader.gl(), &vert_source, &frag_source) {
        Ok(s) => {
            println!("Reloaded");
            shader.shader = s;
        },
        Err(e) => {
            println!("{:?}",e);
        }
    }
}


fn reload_mesh_shader(shader: &mut mesh_shader::MeshShader) {
    let vert_shader_path = std::path::Path::new("E:/repos/rust-gl-lib/assets/shaders/objects/mesh_shader.vert");
    let vert_source = std::fs::read_to_string(vert_shader_path.clone())
        .expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new("E:/repos/rust-gl-lib/assets/shaders/objects/mesh_shader.frag");
    let frag_source = std::fs::read_to_string(frag_shader_path.clone())
        .expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    match shader::BaseShader::new(shader.shader.gl(), &vert_source, &frag_source) {
        Ok(s) => {
            println!("Reloaded");
            shader.shader = s;
        },
        Err(e) => {
            println!("{:?}",e);
        }
    }
}


pub fn draw(gl: &gl::Gl,camera: &Camera, bones: &Bones, shader: &mesh_shader::MeshShader, stencil_shader: &mesh_shader::MeshShader, mesh: &Mesh, _s: f32) {
    // first render as normal, to fill stencil buffer

    // PASS 1

    unsafe {
        gl.StencilFunc(gl::ALWAYS, 1, 0xFF);
        gl.StencilMask(0xFF);
        gl.Enable(gl::DEPTH_TEST);
    }

    // DRAW MESH
    shader.shader.set_used();

    let pos = V3::new(0.0, 0.0, 0.0);
    let trans = Translation3::from(pos);
    let model_mat = trans.to_homogeneous();

    let uniforms = mesh_shader::Uniforms {
        light_pos: V3::new(0.0, 100.0, 100.0),
        projection: camera.projection(),
        model: model_mat,
        view: camera.view(),
        view_pos: camera.pos(),
        bones: &bones
    };

    shader.set_uniforms(uniforms);
    mesh.render(gl);

    // PASS 2
    unsafe {
        gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
        gl.StencilMask(0x00);
        gl.Disable(gl::DEPTH_TEST);
    }


    // DRAW MESH AGAIN WITH NEW SHADER, AND AND SCALED UP A LITTLE NIT

    stencil_shader.shader.set_used();

    stencil_shader.set_uniforms(uniforms);

    mesh.render(gl);


    // RESET GL STATE TO RGULAR
    unsafe {
        gl.StencilFunc(gl::ALWAYS, 0, 0xFF);
        gl.StencilMask(0xFF);
        gl.Enable(gl::DEPTH_TEST);
    }

}

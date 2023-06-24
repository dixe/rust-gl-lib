use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer};
use gl_lib::objects::gltf_mesh::{self, KeyFrame, Animation};
use gl_lib::shader::{self, mesh_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cube};
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Scale3, Translation3};
use gl_lib::{buffer, texture};


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let _audio_subsystem = sdl.audio().unwrap();
    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();


    println!("gl={:?}", std::mem::size_of::<gl::Gl>());
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
    controller.speed = 5.0;


    camera.move_to(V3::new(8.4, 4.3, 5.0));
    let mut la = V3::new(5.0, 3.1, 5.0);
    camera.look_at(la);

    let mut player = AnimationPlayer::new();

    let mut anim_id = 0;
    let mut playing = true;

    let mut t : f32 = 0.0;

    let mut s = 1.0;
    let mut animation : Option<&Animation> = None;

    // frame buffer to render to
    let fbo = buffer::FrameBuffer::new(&gl, &viewport);

    if !fbo.complete() {
        panic!("fbo not complete");
    }

    loop {
        // Basic clear gl stuff and get events to UI


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
            reload_mesh_shader(&mut shader);
        }

        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if ui.button(name) {
                    player.remove(anim_id);
                    anim_id = player.start(Start {anim: &anim, repeat: true});
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

        draw(&gl, &mut ui.drawer2D, &fbo, &camera, &bones, &shader, &mesh, s);


        window.gl_swap_window();
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


pub fn draw(gl: &gl::Gl, drawer2d: &mut Drawer2D, fbo: &buffer::FrameBuffer, camera: &Camera, bones: &Bones, shader: &mesh_shader::MeshShader, mesh: &Mesh, s: f32) {

    fbo.bind();
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
        gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // stencil not used so no need for clearing
        gl.Enable(gl::DEPTH_TEST);
    }

    // DRAW MESH
    shader.shader.set_used();

    let pos = V3::new(-10.0, -10.0, 0.0);
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

    fbo.unbind();

    // pass 2, render fbo color texture
    unsafe {
        gl.Disable(gl::DEPTH_TEST);
        gl.ClearColor(0.5, 0.5, 0.1 , 1.0);
        gl.Clear(gl::COLOR_BUFFER_BIT); // stencil not used so no need for clearing

    }

    drawer2d.render_img(fbo.color_tex, 0, 0, V2::new(1200.0, 800.0));

}

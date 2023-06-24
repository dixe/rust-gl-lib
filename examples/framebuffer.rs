use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer};
use gl_lib::objects::gltf_mesh::{self, KeyFrame, Animation};
use gl_lib::shader::{self, mesh_shader, BaseShader, texture_shader, reload_object_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cube};
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Scale3, Translation3};
use gl_lib::{buffer, texture};
 use gl_lib::shader::Shader;

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
        gl.ClearColor(0.0, 0.0, 0.0, 0.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let glp_path = "E:/repos/Game-in-rust/blender_models/Animation_test.glb";
    //let glp_path = "E:/repos/Game-in-rust/blender_models/enemy1.glb";

    let gltf_data = gltf_mesh::meshes_from_gltf(glp_path)?;

    let mut shader = mesh_shader::MeshShader::new(&gl)?;
    let mut post_process_shader = texture_shader::TextureShader::new(&gl)?;
    reload_object_shader("postprocess", &gl, &mut post_process_shader.shader);

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

    let mut time : f32 = 0.0;

    let mut animation : Option<&Animation> = None;

    // frame buffer to render to
    let mesh_fbo = buffer::FrameBuffer::new(&gl, &viewport);

    let mut ui_fbo = buffer::FrameBuffer::new(&gl, &viewport);

    // all has to be 0, since opengl works with premultiplied alphas, so if a is 0, all others have to be 0
    ui_fbo.r = 0.0;
    ui_fbo.g = 0.0;
    ui_fbo.b = 0.0;
    ui_fbo.a = 0.0;

    loop {

        // Setup to use Ui fbo so all ui is drawn to its own frame buffer
        ui_fbo.bind_and_clear();

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
            reload_object_shader("mesh_shader", &gl, &mut shader.shader);
            reload_object_shader("postprocess", &gl, &mut post_process_shader.shader);
        }

        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if ui.button(name) {
                    player.remove(anim_id);
                    anim_id = player.start(Start {anim: &anim, repeat: true});
                }
            }
        }

        time += dt;
        if ui.button("Reset") {
            time = 0.0;
        }


        // update aimaiton player, and bones
        if playing {
            player.update(dt);
        }

        // update skeleton and bones from animation, if animation done, nothing is updated
        player.update_skeleton(anim_id, &mut skeleton);
        skeleton.set_all_bones_from_skeleton(&mut bones);

        ui_fbo.unbind();


        // draw mesh to its own mesh_fbo
        draw(&gl, &mesh_fbo, &camera, &bones, &shader, &mesh);

        // post process step and render mesh_fbo color texture to screen
        post_process(&mut ui.drawer2D, &mesh_fbo, &post_process_shader, time);


        // render ui on top of frame buffer
        ui.drawer2D.render_img(ui_fbo.color_tex, 0, 0, V2::new(1200.0, 800.0));
        window.gl_swap_window();
    }
}


pub fn post_process(drawer2d: &mut Drawer2D, fbo: &buffer::FrameBuffer, post_p_shader: &texture_shader::TextureShader, time: f32) {

    // pass 2, render fbo color texture
    unsafe {
        drawer2d.gl.Disable(gl::DEPTH_TEST);
        drawer2d.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
        drawer2d.gl.Clear(gl::COLOR_BUFFER_BIT); // stencil not used so no need for clearing
    }


    post_p_shader.shader.set_used();
    post_p_shader.shader.set_f32(&drawer2d.gl, "time", time);
    drawer2d.render_img_custom_shader(fbo.color_tex, 0, 0, V2::new(1200.0, 800.0), post_p_shader);
}

pub fn draw(gl: &gl::Gl, fbo: &buffer::FrameBuffer, camera: &Camera, bones: &Bones, shader: &mesh_shader::MeshShader, mesh: &Mesh) {

    fbo.bind_and_clear();

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


}

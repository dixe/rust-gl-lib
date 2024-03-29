use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer};
use gl_lib::objects::gltf_mesh::{self, Animation};
use gl_lib::shader::{mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cubemap::{self, Cubemap}};
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Translation3};
use gl_lib::{buffer};
use gl_lib::shader::Shader;
use std::{thread, sync::{Arc, Mutex}};


fn main() -> Result<(), failure::Error> {
        let mut sdl_setup = helpers::setup_sdl()?;
    let mut ui = sdl_setup.ui();


    let glp_path = "E:/repos/Game-in-rust/blender_models/Animation_test.glb";
    //let glp_path = "E:/repos/Game-in-rust/blender_models/enemy1.glb";

    let gltf_data = gltf_mesh::meshes_from_gltf(glp_path, false)?;

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
    let la = V3::new(5.0, 3.1, 5.0);
    camera.look_at(la);

    let mut player = AnimationPlayer::new();

    let anim_id = 0;
    let mut playing = true;

    let mut time : f32 = 0.0;

    let _animation : Option<&Animation> = None;

    // frame buffer to render to
    let mesh_fbo = buffer::FrameBuffer::new(&gl, &viewport);

    let mut ui_fbo = buffer::FrameBuffer::new(&gl, &viewport);

    // all has to be 0, since opengl works with premultiplied alphas, so if a is 0, all others have to be 0
    ui_fbo.r = 0.0;
    ui_fbo.g = 0.0;
    ui_fbo.b = 0.0;
    ui_fbo.a = 0.0;


    let mut cubemap_shader = load_object_shader("cubemap", gl).unwrap();
    let mut cubemap : Option::<Cubemap> = None;

    let cubemap_imgs : Arc::<Mutex::<Option<Vec::<image::RgbImage>>>> = Arc::new(Mutex::new(None));

    // start thread to load cubemap
    let cm = cubemap_imgs.clone();
    thread::spawn(move || {

        let imgs = cubemap::load_cubemap_images(&"assets/cubemap/skybox/");
        {
            let mut mutex_cm = cm.lock().unwrap();
            *mutex_cm = Some(imgs);
        }

    });


    let clear_bits = gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT;

    loop {

        // Setup to use Ui fbo so all ui is drawn to its own frame buffer
        ui_fbo.bind_and_clear(clear_bits);

        ui.start_frame(&mut sdl_setup.event_pump);

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
            reload_object_shader("cubemap", &gl, &mut cubemap_shader);
        }

        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if ui.button(name) {
                    player.remove(anim_id);
                    player.start(Start {id: anim_id, speed: 1.0, transition: None, anim: anim.clone(), repeat: true});
                }
            }
        }

        if cubemap.is_none() {
            let mut lock = cubemap_imgs.try_lock();
            if let Ok(ref mut mutex_imgs) = lock {
                if let Some(ref imgs) = **mutex_imgs {
                    cubemap = Some(Cubemap::from_images(gl, &imgs));
                    // TODO:  maybe some cleanup of images and img vec, so we don't keep it in ram
                }
                **mutex_imgs = None

            } else {
                println!("try_lock failed");
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
        player.update_skeleton_and_bones(anim_id, &mut skeleton, &mut bones);

        ui_fbo.unbind();


        // draw mesh to its own mesh_fbo
        draw(&gl, &mesh_fbo, &camera, &bones, &shader, &mesh, &cubemap, &cubemap_shader);

        // post process step and render mesh_fbo color texture to screen
        post_process(&mut ui.drawer2D, &mesh_fbo, &post_process_shader, time);


        // render ui on top of frame buffer
        ui.drawer2D.render_img(ui_fbo.color_tex, 0, 0, V2::new(1200.0, 800.0));

        ui.end_frame();
    }
}


pub fn post_process(drawer2d: &mut Drawer2D, fbo: &buffer::FrameBuffer, post_p_shader: &texture_shader::TextureShader, time: f32) {

    // pass 2, render fbo color texture
    // TODO: maybe add this to unbind?? since unbind is a frame buffer bind or screen buffer.
    unsafe {
        drawer2d.gl.Disable(gl::DEPTH_TEST);
        drawer2d.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
        drawer2d.gl.Clear(gl::COLOR_BUFFER_BIT); // stencil not used so no need for clearing
    }

    post_p_shader.shader.set_used();
    post_p_shader.shader.set_f32(&drawer2d.gl, "time", time);
    drawer2d.render_img_custom_shader(fbo.color_tex, 0, 0, V2::new(1200.0, 800.0), post_p_shader);
}

pub fn draw(gl: &gl::Gl, fbo: &buffer::FrameBuffer, camera: &Camera,
            bones: &Bones, shader: &mesh_shader::MeshShader, mesh: &Mesh,
            cubemap_opt: &Option<Cubemap>, cubemap_shader: &BaseShader) {

    let clear_bits = gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT;
    fbo.bind_and_clear(clear_bits);


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

    if let Some(cubemap) = cubemap_opt {

        // DRAW SKYBOX
        cubemap_shader.set_used();
        // could use nalgebra glm to remove translation part on cpu, and not have gpu multiply ect.
        cubemap_shader.set_mat4(gl, "projection", camera.projection());
        cubemap_shader.set_mat4(gl, "view", camera.view());
        cubemap.render(gl);
    }

    fbo.unbind();


}

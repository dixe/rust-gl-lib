use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Scale3, Translation3};
use gl_lib::{buffer, texture};
use gl_lib::shader::Shader;
use std::{thread, sync::{Arc, Mutex}};
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer};
use gl_lib::objects::gltf_mesh::{self, KeyFrame, Animation};
use gl_lib::shader::{self, mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cubemap::{self, Cubemap}};

mod scene;

pub struct PostPData {
    time: f32
}

fn post_process_uniform_set(gl: &gl::Gl, shader: &mut BaseShader, data : &PostPData) {
    shader.set_f32(gl, "time", data.time);
}

fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let _audio_subsystem = sdl.audio().unwrap();

    let mut scene = scene::Scene::new((), gl.clone(), viewport)?;
    let mut event_pump = sdl.event_pump().unwrap();

    scene.load_all_meshes("E:/repos/Game-in-rust/blender_models/Animation_test.glb");

    // map to mesh, skeleton, bones
    let player_id = scene.create_entity("Cube");

    let mut look_at = V3::new(5.0, 3.1, 5.0);
    scene.free_controller.speed = 5.0;
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));


    scene.camera.look_at(look_at);

    let mut playing = true;

    scene.set_skybox(&"assets/cubemap/skybox/");

    let ppd = PostPData {
        time : 0.0
    };

    scene.use_fbos(ppd, Some(post_process_uniform_set));

    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(&mut event_pump);

        let dt = scene.dt();

        // OWN GAME LOGIC
        if scene.ui.button("Play/Pause") {
            playing = !playing;
        }

        if scene.ui.button("Reload") {
            reload_object_shader("mesh_shader", &gl, &mut scene.mesh_shader.shader);
            if let Some(ref mut fbos) = scene.fbos {
                reload_object_shader("postprocess", &gl, &mut fbos.post_process_shader.shader);
            }
            reload_object_shader("cubemap", &gl, &mut scene.cubemap_shader);
        }



        // change animation of entity
        let player_skel = scene.get_entity(&player_id).unwrap().skeleton_id.unwrap();
        let animations = scene.animations.get(&player_skel).unwrap();
        for (name, anim) in animations {
            if scene.ui.button(name) {
                scene::play_animation(anim.clone(), true, &player_id, &mut scene.player, &mut scene.animation_ids);
            }
        }


        if let Some(ref mut fbos) = scene.fbos {
            fbos.post_process_data.time += dt;
            if scene.ui.button("Reset") {
                fbos.post_process_data.time = 0.0;
            }

        }

        // update aimaiton player, and bones
        if playing {
            scene.update_animations();
        }

        scene.render();

            /*
{
        ui_fbo.unbind();


        // draw mesh to its own mesh_fbo
        draw(&gl, &mesh_fbo, &camera, &bones, &shader, &mesh, &cubemap, &cubemap_shader);

        // post process step and render mesh_fbo color texture to screen
        post_process(&mut ui.drawer2D, &mesh_fbo, &post_process_shader, time);


        // render ui on top of frame buffer
        ui.drawer2D.render_img(ui_fbo.color_tex, 0, 0, V2::new(1200.0, 800.0));

}
*/
        window.gl_swap_window();
    }
}


/*
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
*/

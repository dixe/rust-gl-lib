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


    let mut shader = mesh_shader::MeshShader::new(&gl)?;
    let mut post_process_shader = texture_shader::TextureShader::new(&gl)?;
    reload_object_shader("postprocess", &gl, &mut post_process_shader.shader);
/*
    let mesh_name = "Cube";
    let mesh = gltf_data.meshes.get_mesh(&gl, &mesh_name).unwrap();

    let skin_id = gltf_data.skins.mesh_to_skin.get(mesh_name).unwrap();

    let mut skeleton: Skeleton = gltf_data.skins.skeletons.get(&skin_id).unwrap().clone();

    let mut bones = skeleton.create_bones();

    let mut camera = camera::Camera::new(viewport.w as f32, viewport.h as f32);

    let mut controller = free_camera::Controller::default();
     */

    let mut look_at = V3::new(5.0, 3.1, 5.0);
    scene.free_controller.speed = 5.0;
    scene.camera.move_to(V3::new(8.4, 4.3, 5.0));


    scene.camera.look_at(look_at);


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


    let mut cubemap_shader = load_object_shader("cubemap", gl).unwrap();
    let mut cubemap : Option::<Cubemap> = None;

    //TODO: scene.set_skybox(&"assets/cubemap/skybox/");

    loop {

        // set ui framebuffer, consume sdl events, increment dt ect.
        scene.frame_start(&mut event_pump);

        let dt = scene.dt();

        // OWN GAME LOGIC
        if scene.ui.button("Play/Pause") {
            playing = !playing;
        }

        if scene.ui.button("Reload") {
            reload_object_shader("mesh_shader", &gl, &mut shader.shader);
            reload_object_shader("postprocess", &gl, &mut post_process_shader.shader);
            reload_object_shader("cubemap", &gl, &mut cubemap_shader);
        }


        //let names = scene.animations_for(&player_id).unwrap().collect();
        // change animation of entity
        /*
        for name in scene.animations_for(&player_id).unwrap() {
            if scene.ui.button(name) {
                scene.play_animation(player_id, name, true);
            }
        }
*/

        /*
        // hmm, should call into scene since its the owner/manager of this gltf data
        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if scene.ui.button(name) {

                    scene.change_animation(player_id,

                    player.remove(anim_id);
                    anim_id = player.start(Start {anim: &anim, repeat: true});
                }
            }
        }
         */


        time += dt;
        if scene.ui.button("Reset") {
            time = 0.0;
        }


        // update aimaiton player, and bones
        if playing {
            player.update(dt);
        }

        // update skeleton and bones from animation, if animation done, nothing is updated

        // also scene stuff
        scene.update_animations();
        /*
{

        player.update_skeleton(anim_id, &mut skeleton);
        skeleton.set_all_bones_from_skeleton(&mut bones);

}
         */


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

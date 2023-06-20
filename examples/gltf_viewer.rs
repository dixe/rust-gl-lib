use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer};
use gl_lib::objects::gltf_mesh::{self, KeyFrame, Animation};
use gl_lib::shader::mesh_shader;
use gl_lib::typedef::*;
use gl_lib::objects::mesh::Mesh;
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::Translation3;


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let _audio_subsystem = sdl.audio().unwrap();
    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();

    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let glp_path = "E:/repos/Game-in-rust/blender_models/Animation_test.glb";
    //let glp_path = "E:/repos/Game-in-rust/blender_models/enemy1.glb";

    let gltf_data = gltf_mesh::meshes_from_gltf(glp_path)?;

    let shader = mesh_shader::MeshShader::new(&gl)?;

    let mesh_name = "Cube";
    let mesh = gltf_data.meshes.get_mesh(&gl, &mesh_name).unwrap();

    let skin_id = gltf_data.skins.mesh_to_skin.get(mesh_name).unwrap();

    let mut skeleton: Skeleton = gltf_data.skins.skeletons.get(&skin_id).unwrap().clone();

    let mut bones = skeleton.create_bones();


    let mut camera = camera::Camera::new(viewport.w as f32, viewport.h as f32);

    let mut controller = free_camera::Controller::default();

    camera.move_to(V3::new(8.4, 4.3, 5.0));
    let mut la = V3::new(5.0, 3.1, 5.0);
    camera.look_at(la);

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
    }

    let mut player = AnimationPlayer::new();

    let mut anim_id = 0;
    let mut playing = true;

    let pos = V3::new(0.0, 0.0, 0.0);
    let trans = Translation3::from(pos);
    let model_mat = trans.to_homogeneous() * Mat4::identity();

    let mut t : f32 = 0.0;

    let mut animation : Option<&Animation> = None;
    loop {
        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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

        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if ui.button(name) {
                    player.remove(anim_id);
                    anim_id = player.start(Start {anim: &anim, repeat: true});
                }
            }
        }

        // update aimaiton player, and bones
        if playing {
            player.update(dt);
        }

        // update skeleton and bones from animation, if animation done, nothing is updated
        player.update_skeleton(anim_id, &mut skeleton);
        skeleton.set_all_bones_from_skeleton(&mut bones);

        draw(&gl, &camera, &bones, model_mat, &shader, &mesh);


        window.gl_swap_window();
    }
}


pub fn draw(gl: &gl::Gl,camera: &Camera, bones: &Bones, model_mat: Mat4, shader: &mesh_shader::MeshShader, mesh: &Mesh) {

    shader.shader.set_used();

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
}

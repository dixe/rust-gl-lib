use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::audio::audio_player;
use gl_lib::animations::sheet_animation::{load_folder, SheetAnimationPlayer};
use gl_lib::animations::skeleton::Skeleton;
use gl_lib::objects::gltf_mesh::{self, KeyFrame, Animation};
use gl_lib::shader::mesh_shader;
use gl_lib::typedef::*;
use gl_lib::math::AsV2;
use gl_lib::camera::{self, free_camera};
use gl_lib::na::Translation3;


fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let audio_subsystem = sdl.audio().unwrap();
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


    let mut play = false;
    let mut pos = V3::new(0.0, 0.0, 0.0);
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

        shader.shader.set_used();

        let trans = Translation3::from(pos);
        let model = trans.to_homogeneous() * Mat4::identity();

        if ui.slider(&mut la.x, 0.0, 10.0) {
            camera.look_at(la);
        }
        if ui.slider(&mut la.y, 0.0, 10.0) {
            camera.look_at(la);
        }

        if ui.slider(&mut la.z, 0.0, 10.0) {
            camera.look_at(la);
        }

        // Change t, 0 is start of animation, 1 is last frame
        ui.slider(&mut t, 0.0, 1.0);

        if play {
            t += dt;
            if t > 1.0 {
                t = 0.0;
            }
        }

        if ui.button("Play") {
            play = !play;

        }

        if let Some(Animation { frames, ..}) = animation {
            // update skeleton
            let frame_count = frames.len() as f32;
            let frame_idx = usize::min( frames.len() - 1, (frame_count * t) as usize);
            let frame = &frames[frame_idx];

            update_skeleton_to_key_frame(&mut skeleton, frame);
            skeleton.set_all_bones_from_skeleton(&mut bones);
        }

        for skin_id in gltf_data.animations.keys() {
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                if ui.button(&format!("{skin_id}_ -  {name}")) {
                    animation = Some(anim);
                }
            }
        }

        let uniforms = mesh_shader::Uniforms {
            light_pos: V3::new(0.0, 100.0, 100.0),
            projection: camera.projection(),
            model: model,
            view: camera.view(),
            view_pos: camera.pos(),
            bones: &bones
        };


        shader.set_uniforms(uniforms);
        mesh.render(&gl);

        window.gl_swap_window();
    }
}

pub fn update_skeleton_to_key_frame(skeleton: &mut Skeleton, key_frame: &KeyFrame) {
    // interpolate joints new transformation
    for i in 0..skeleton.joints.len() {
        let rotation = key_frame.joints[i].rotation;

        let translation = key_frame.joints[i].translation;

        skeleton.update_joint_matrices(i, rotation, translation);
    }
}

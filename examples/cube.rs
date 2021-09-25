use gl_lib::{gl, objects::cube, shader, camera};
use failure;

use nalgebra as na;

fn main() -> Result<(), failure::Error> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    //sdl.mouse().show_cursor(false);

    //sdl.mouse().set_relative_mouse_mode(true);

    let controller_subsystem = sdl.game_controller().unwrap();

    controller_subsystem.set_event_state(true);

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);


    let width = 800;
    let height = 600;

    let viewport = gl::viewport::Viewport::for_window(width as i32, height as i32);

    let window = video_subsystem
        .window("Game", width, height)
        .opengl()
        .resizable()
        .build()?;



    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    viewport.set_used(&gl);

    unsafe {
        gl.Enable(gl::DEPTH_TEST);

    }

    let mut camera = camera::Camera::new( width as f32, height as f32);
    let shader = create_shader(&gl);
    let cube = cube::Cube::new(&gl);

    let mut model_mat = na::Matrix4::identity();

    let color = na::Vector3::new(0.4, 0.9, 0.3);

    camera.update_pos(na::Vector3::new(0.0, -5.0, 2.0));


    unsafe {
        gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
    loop {

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Set the model matrix
        shader.set_mat4(&gl, "model", model_mat);

        shader.set_mat4(&gl, "view", camera.view());

        shader.set_mat4(&gl, "projection", camera.projection());


        // set color
        shader.set_vec3(&gl, "color", &color);


        cube.render(&gl);
        window.gl_swap_window();
    }

}


fn create_shader(gl: &gl::Gl) -> shader::Shader {
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position =  projection * view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

    let frag_source = r"#version 330 core
out vec4 FragColor;
uniform vec3 color;
void main()
{
    FragColor = vec4(color, 1.0f);
}";


    shader::Shader::new(gl, vert_source, frag_source).unwrap()
}

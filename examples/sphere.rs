use gl_lib::{gl, objects::sphere, shader::{self, Shader}, camera};
use failure;
use std::time::Instant;

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
    let sphere = sphere::Sphere::new(&gl, 50, 50);

    let model_mat = na::Matrix4::identity();

    let color = na::Vector3::new(1.0, 1.0, 1.0);

    let dist = 3.0;


    unsafe {
        //gl.PolygonMode(gl::FRONT, gl::LINE);

        // background color
        gl.ClearColor(0.85, 0.8, 0.7, 1.0);

        gl.CullFace(gl::FRONT);
    }

    let mut instant = Instant::now();

    let mut angle = 0.0;

    let mut line = false;
    let mut r = 1.0;


    loop {

        let delta = (instant.elapsed().as_millis() as f32) / 1000.0;
        instant = Instant::now();

        angle += delta;
        // update camera pos

        let x = f32::sin(angle);
        let y = f32::cos(angle);
        let z = 0.0;

        camera.update_pos(na::Vector3::new(x * dist, y * dist, z));

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Set the model matrix
        shader.set_mat4(&gl, "model", model_mat);
        shader.set_mat4(&gl, "view", camera.view());
        shader.set_mat4(&gl, "projection", camera.projection());

        r = (angle /2.0).sin();
        shader.set_f32(&gl, "radius", r);


        // set color
        shader.set_vec3(&gl, "color", color);


        sphere.render(&gl);
        window.gl_swap_window();

        if angle > std::f32::consts::PI * 2.0 {
            angle = 0.0;


            unsafe {
                if line {
                    gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

                }
                else {
                    gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
                //line = !line;
            }
        }


    }

}

fn create_shader(gl: &gl::Gl) -> shader::BaseShader {
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

out VS_OUTPUT {
   flat vec3 Color;
} OUT;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float radius;

void main()
{
    OUT.Color = aPos;
    vec3 pos = aPos * radius;
    gl_Position =  projection * view * model * vec4(pos.x, pos.y, pos.z, 1.0);
}";

    let frag_source = r"#version 330 core
out vec4 FragColor;
uniform vec3 color;


in VS_OUTPUT {
    flat vec3 Color;
} IN;

void main()
{
    FragColor = vec4(IN.Color * color, 1.0f);
}";


    shader::BaseShader::new(gl, vert_source, frag_source).unwrap()
}

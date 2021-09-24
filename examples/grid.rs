use sdl2::{Sdl};
use gl_lib::{gl, objects::square, shader};
use std::io;
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



    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    viewport.set_used(&gl);


    let shader = create_shader(&gl);
    let square = square::Square::new(&gl).unwrap();

    loop {
        render_grid(&gl, &shader, &square, 8, 7);
        window.gl_swap_window();
    }

    Ok(())
}


fn create_shader(gl: &gl::Gl) -> shader::Shader {
    let vert_source = r"#version 330 core
                                   layout (location = 0) in vec3 aPos;

                                   uniform mat4 model;
                                   void main()
                                   {
                                       gl_Position = model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
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


fn render_grid(gl: &gl::Gl, shader: &shader::Shader, square: &square::Square, rows: usize, cols: usize) {

    // colors of the grid, black and white
    let colors = [na::Vector3::new(0.0, 0.0, 0.0),na::Vector3::new(1.0, 1.0, 1.0)];

    let mut i = 0;
    shader.set_used();
    for row in 0..rows {
        for col in 0..cols {

            // total witdh of screen space is 2, the square has width and height 1

            let mut scale_mat = na::Matrix4::identity();
            let scale_x = 2.0 / (cols as f32);
            let scale_y = 2.0 / (rows as f32);

            //println!("{:?} {} ",scale_x, scale_y);
            scale_mat[0] = scale_x;
            scale_mat[5] = scale_y;
            scale_mat[10] = 1.0;
            scale_mat[15] = 1.0;


            let r = -(1.0-scale_x/2.0) + ((col as f32) * scale_x);
            let c = -(1.0-scale_y/2.0) + ((row as f32) * scale_y);

            let trans_mat = na::Matrix4::new_translation(&na::Vector3::new(r, c, 0.0));

            //println!("{:?}",r);
            scale_mat.append_translation(&na::Vector3::new(r, 0.0, 0.0));


            // Set the model matrix to scale and translate the square
            shader.set_mat4(gl, "model", trans_mat * scale_mat);

            // set color of this square
            shader.set_vec3(gl, "color", &colors[i]);


            square.render(&gl);

            // update color index i
            i = (i + 1) % 2;
        }
    }
}

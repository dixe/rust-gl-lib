use gl_lib::{gl, shader, text_rendering::font, texture, objects};
use failure;
use std::time::Instant;
use std::path::Path;


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
        gl.ClearColor(0.85, 0.8, 0.7, 1.0);
    }


    let shader = create_shader(&gl);

    let font = font::Font::load_fnt_font(Path::new("./assets/fonts/Arial.fnt")).unwrap();
    let tex_id = texture::gen_texture(&gl, &font.image);


    texture::set_texture(&gl, tex_id);
    let color = na::Vector3::new(0.4, 0.9, 0.3);

    // set color
    shader.set_vec3(&gl, "color", &color);

    let char_quad = objects::char_quad::CharQuad::new(&gl, 65, &font);

    shader.set_used();

    loop {
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        char_quad.render(&gl);

        window.gl_swap_window();
    }

}


fn create_shader(gl: &gl::Gl) -> shader::Shader {
    let vert_source = r"#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tex_coord;

out VS_OUTPUT {
    vec2 TexCoords;
} OUT;


void main()
{
    gl_Position = vec4(pos, 0.0, 1.0);
    OUT.TexCoords = tex_coord;
}";

    let frag_source = r"#version 330 core
out vec4 FragColor;
uniform vec3 color;

uniform sampler2D text_map;

in VS_OUTPUT {
   vec2 TexCoords;
} IN;

void main()
{

    float tex_val = IN.TexCoords.x;
    //texture(text_map, IN.TexCoords).r;

    FragColor = vec4(IN.TexCoords.x, IN.TexCoords.y, tex_val, 1.0);
}";


    shader::Shader::new(gl, vert_source, frag_source).unwrap()
}

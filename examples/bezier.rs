use gl_lib::{gl, objects::bezier, shader, na};
use failure;


fn main() -> Result<(), failure::Error> {
    // Init sdl to use opengl
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);


    // Create a window that opengl can draw to
    let width = 800;
    let height = 600;

    let viewport = gl::viewport::Viewport::for_window(width as i32, height as i32);

    let window = video_subsystem
        .window("Game", width, height)
        .opengl()
        .resizable()
        .build()?;


    // Load gl functions and set to sdl video subsystem
    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    viewport.set_used(&gl);

    // Create a default shader
    let shader = shader::Shader::bezier_shader(&gl)?;

    // and a bezier curve
    let bezier = bezier::Bezier::new(
        &gl,
        bezier::Curve {
            p0: na::Vector2::new(0.0, 0.0),
            p1: na::Vector2::new(2.0, 1.0),
            p2: na::Vector2::new(1.0, 1.0)
        });


    loop {
        shader.set_used();
        bezier.render(&gl);
        window.gl_swap_window();
    }
}

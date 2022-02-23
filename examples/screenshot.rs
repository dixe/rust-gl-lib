use failure;
use gl_lib::{gl, objects::square, shader};
use image::{Rgba, RgbaImage};

fn main() -> Result<(), failure::Error> {
    // Init sdl to use opengl
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

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
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    viewport.set_used(&gl);

    // Create a default shader
    let shader = shader::Shader::default_shader(&gl)?;

    // and a default square
    let square = square::Square::new(&gl);

    shader.set_used();
    square.render(&gl);

    window.gl_swap_window();

    let size = (height * width * 4) as usize;

    let screenshot_buffer: Vec::<u8> = vec![0; size];

    unsafe {
        gl.ReadBuffer(gl::FRONT);
        gl.ReadPixels(
            0,
            0,
            width as i32,
            height as i32,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            screenshot_buffer.as_ptr() as *mut std::os::raw::c_void,
        );
    }

    let mut img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let i : usize = (y * 4 * width + x * 4) as usize;
            img.put_pixel(x, y, Rgba([screenshot_buffer[i], screenshot_buffer[i + 1], screenshot_buffer[i + 2], screenshot_buffer[i + 3]]));
        }
    }

    img.save("screenshot.png").unwrap();
    Ok(())
}

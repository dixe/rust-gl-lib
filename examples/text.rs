use gl_lib::gl;
use gl_lib::text_rendering::text_renderer;
use failure;


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


    let width = 1920;
    let height = 1080;

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


    let text_renderer = text_renderer::TextRenderer::new(&gl);

    unsafe {
        // Either disable Depth test or set depth funct to LEQUAL
        gl.Enable(gl::DEPTH_TEST);
        gl.DepthFunc(gl::LEQUAL);
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }


    loop {
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let x = -0.7;
        let y = 0.0;
        let scale = 1.5;
        text_renderer.render_text(&gl, "Yo Yo ho bjd å øæå please wrap me around", x, y, scale);

        window.gl_swap_window();
    }

}

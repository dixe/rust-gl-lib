use crate::gl;
use sdl2;
use failure::Fail;
use crate::shader::rounded_rect_shader::RoundedRectShader;
use crate::shader::circle_shader::CircleShader;
use crate::objects::square::Square;
use crate::text_rendering::text_renderer::TextRenderer;
use crate::text_rendering::font::*;


#[derive(Debug, Fail)]
pub enum SetupError {
    #[fail(display = "Window build error")]
    WindowBuild(sdl2::video::WindowBuildError),
    #[fail(display = "General error")]
    General(String),
    #[fail(display = "Failure")]
    Failure(failure::Error)
}

pub struct BasicSetup {
    pub width: u32,
    pub height: u32,
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub gl: gl::Gl,
    pub window: sdl2::video::Window,
    pub viewport: gl::viewport::Viewport,
    pub gl_context: sdl2::video::GLContext
}


impl From<failure::Error> for SetupError {
    fn from(other: failure::Error) -> Self {
        SetupError::Failure(other)
    }
}




impl From<sdl2::video::WindowBuildError> for SetupError {

    fn from(other: sdl2::video::WindowBuildError) -> Self {
        SetupError::WindowBuild(other)
    }
}


impl From<String> for SetupError {

    fn from(other: String) -> Self {
        SetupError::General(other)
    }
}


pub fn setup_sdl() -> Result<BasicSetup, SetupError> {

    // Init sdl to use opengl
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);




    // Create a window that opengl can draw to
    let width = 1200;
    let height = 800;

    let viewport = gl::viewport::Viewport::for_window(width as i32, height as i32);

    let window = video_subsystem
        .window("Square", width, height)
        .opengl()
        .resizable()
        .build()?;


    // Load gl functions and set to sdl video subsystem
    let gl_context = window.gl_create_context()?;
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    viewport.set_used(&gl);


    Ok(BasicSetup {
        sdl,
        width,
        height,
        video_subsystem,
        gl,
        window,
        viewport,
        gl_context
    })
}


pub struct WidgetSetup {
    pub gl: gl::Gl,
    pub text_renderer : TextRenderer,
    pub circle_shader : CircleShader,
    pub render_square: Square,
    pub rounded_rect_shader: RoundedRectShader
}


pub fn setup_widgets(gl: &gl::Gl) -> Result<WidgetSetup, SetupError> {

    let inner_font = Default::default();
    let font = Font::Msdf(inner_font);
    let text_renderer = TextRenderer::new(gl, font);
    text_renderer.setup_blend(gl);
    let rrs = RoundedRectShader::new(gl)?;
    let cs = CircleShader::new(gl)?;

    let square = Square::new(gl);

    Ok(WidgetSetup {
        gl: gl.clone(),
        text_renderer,
        rounded_rect_shader: rrs,
        render_square: square,
        circle_shader: cs
    })
}

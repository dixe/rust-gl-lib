use gl_lib::{gl, objects::square, shader};
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::TextRenderer;
use gl_lib::widget_gui::widgets::{RowWidget, TextWidget};

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
        .window("Square", width, height)
        .opengl()
        .resizable()
        .build()?;


    // Load gl functions and set to sdl video subsystem
    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    viewport.set_used(&gl);



    // Setup widget ui


    let mut ui_state = create_ui();

    let font = Default::default();

    let mut text_renderer = TextRenderer::new(&gl, font) ;


    text_renderer.setup_blend(&gl);
    let mut render_ctx = render::RenderContext {
        gl: &gl,
        viewport: &viewport,
        tr: &mut text_renderer
    };


    // Set background color to white
    unsafe {
        gl.ClearColor(1.0, 1.0, 1.0, 1.0);
    }


    let root_box = BoxContraint::new(viewport.w, viewport.h);
    layout_widgets(&root_box, &mut ui_state);

    loop {

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render::render_ui(&ui_state, &mut render_ctx);

        window.gl_swap_window();
    }
}



fn create_ui() -> UiState {

    let mut ui_state = UiState::new();
    let row_widget = RowWidget {};
    let row_id = ui_state.add_widget(Box::new(row_widget), None, None);


    let text_widget_1 = TextWidget { text: "Hello".to_string(), scale: 1.0 };
    let _ = ui_state.add_widget(Box::new(text_widget_1), Some(row_id), None);

    let text_widget_2 = TextWidget { text: "World".to_string(), scale: 1.0 };
    let _ = ui_state.add_widget(Box::new(text_widget_2), Some(row_id), None);

    ui_state

}


fn create_shader(gl: &gl::Gl) -> shader::Shader {
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out VS_OUTPUT {
   flat vec3 Color;
} OUT;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    OUT.Color = aColor;
    gl_Position =  projection * view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
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


    shader::Shader::new(gl, vert_source, frag_source).unwrap()
}

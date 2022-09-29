use gl_lib::{gl, objects::square, shader};
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::{text_renderer::TextRenderer, font::Font};
use gl_lib::widget_gui::widgets::{RowWidget, TextWidget};
use gl_lib::helpers;
use std::path::Path;

fn main() -> Result<(), failure::Error> {


    let mut sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;


    // Set background color to white
    unsafe {
        gl.ClearColor(0.5, 0.5, 0.5, 0.5);
    }


    let mut widget_setup = helpers::setup_widgets(gl)?;


    let mut render_ctx = render::RenderContext {
        gl: gl,
        viewport: &viewport,
        tr: &mut widget_setup.text_renderer,
        rounded_rect_shader: &mut widget_setup.rounded_rect_shader,
        render_square: &widget_setup.render_square,
        circle_shader: &mut widget_setup.circle_shader

    };




    // Setup widget ui
    let mut ui_state = create_ui();

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
    let row_id = ui_state.add_widget(Box::new(row_widget), None);


    let text_widget_1 = TextWidget { text: "W".to_string(), scale: 1.0 };
    let _ = ui_state.add_widget(Box::new(text_widget_1), Some(row_id));

    //let text_widget_2 = TextWidget { text: "World".to_string(), scale: 1.0 };
    //let _ = ui_state.add_widget(Box::new(text_widget_2), Some(row_id));

    ui_state

}


fn create_shader(gl: &gl::Gl) -> shader::BaseShader {
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


    shader::BaseShader::new(gl, vert_source, frag_source).unwrap()
}

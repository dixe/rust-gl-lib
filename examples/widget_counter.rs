use gl_lib::{gl, ScreenBox, sdl2};
use gl_lib::text_rendering::text_renderer::TextAlignment;
use gl_lib::shader;
use gl_lib::helpers;
use failure;
use gl_lib::widget_gui::*;
use gl_lib::widget_gui::layout::*;
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::{dispatch_event, dispatch_widget_inputs};
use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;


    // Set background color to white
    unsafe {
        gl.ClearColor(1.0, 0.0, 0.5, 1.0);
    }

    let mut widget_setup = helpers::setup_widgets(gl)?;


    // Setup widget ui
    let (mut ui_info, mut ui_state) = create_ui(&gl);

    let root_box = BoxContraint::new(viewport.w, viewport.h);
    layout_widgets(&root_box, &mut ui_state);

    let mut event_pump = sdl.event_pump().unwrap();

    loop {

        // dispatch events

        for event in event_pump.poll_iter() {
            dispatch_event(&mut ui_state, &event);

            match event {
                KeyDown{keycode: Some(kc), .. } => {
                    match kc {
                        R => {

                            reload_text_shader(&gl, &mut widget_setup);

                            println!("reload");

                        },
                        _ => {}
                    };
                },
                _ => {}
            };
        }

        dispatch_widget_inputs(&mut ui_state);


        let mut render_ctx = render::RenderContext {
            gl: gl,
            viewport: &viewport,
            tr: &mut widget_setup.text_renderer,
            rounded_rect_shader: &mut widget_setup.rounded_rect_shader,
            render_square: &widget_setup.render_square,
            circle_shader: &mut widget_setup.circle_shader
        };



        // handle events for each widget
        while let Some(event) = ui_state.poll_widget_outputs() {
            handle_widget_outputs(&mut ui_info, event, &mut ui_state.widget_input_queue, &mut render_ctx);
        }

        //write_count(&ui_info);


        layout_widgets(&root_box, &mut ui_state);

        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }






        render::render_ui(&ui_state, &mut render_ctx);

        // Render text that outside ui, is affected by out ui_info.slider_ref, that our slider also controlls
        render_ctx.tr.render_text(render_ctx.gl, "Hello", TextAlignment::default(), ScreenBox::new(00.0, 00.0, 1200.0, 700.0, 1200.0, 700.0), ui_info.scale);

        window.gl_swap_window();
    }
}



fn reload_text_shader(gl: &gl::Gl, widget_setup: &mut helpers::WidgetSetup) {


    let vert_shader_path = std::path::Path::new("E:/repos/rust-gl-lib/assets/shaders/sdf_text_render.vert");
    let vert_source = std::fs::read_to_string(vert_shader_path)
        .expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new("E:/repos/rust-gl-lib/assets/shaders/sdf_text_render.frag");
    let frag_source = std::fs::read_to_string(frag_shader_path)
        .expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    match shader::BaseShader::new(gl, &vert_source, &frag_source) {
        Ok(s) => {
            widget_setup.text_renderer.font_mut().change_shader(s);
        },
        Err(e) => {
            println!("{:?}",e);
        }
    }
}



fn write_count(info: &UiInfo) {
    println!("Count is {:?}", info.count);
}


fn handle_widget_outputs(ui_info: &mut UiInfo, event: WidgetOutput, widget_input_queue: &mut WidgetInputQueue, r_ctx: &mut render::RenderContext) {
    if event.widget_id == ui_info.add_button_id {
        // we don't case about the message, just that add was pressed
        ui_info.count += 1;
        // push into counter event queue a message with the count
        widget_input_queue.push_value(ui_info.counter_id, ui_info.count);

    }

    if event.widget_id == ui_info.sub_button_id {
        // we don't case about the message, just that sub was pressed
        ui_info.count -= 1;
        widget_input_queue.push_value(ui_info.counter_id, ui_info.count);
    }

    if event.widget_id == ui_info.smoothness_slider_id {
        if let Some(slider_v) = event.event.downcast_ref::<f64>() {
            println!("Smoothness: {:?}", *slider_v);
            r_ctx.tr.set_smoothness( *slider_v as f32);
        }
    }

    if event.widget_id == ui_info.scale_slider_id {
        if let Some(slider_v) = event.event.downcast_ref::<f64>() {
            println!("scale: {:?}", *slider_v);
            ui_info.scale = *slider_v as i32
        }
    }
}

struct UiInfo {
    count: i32,
    scale: i32,
    counter_id: Id,
    add_button_id: Id,
    sub_button_id: Id,
    smoothness_slider_id:Id,
    scale_slider_id: Id,
}


fn create_ui(gl: &gl::Gl) -> (UiInfo, UiState) {


    let mut ui_state = UiState::new(gl);
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None);

    let count = 0;

    let counter_widget_1 = CounterWidget{ count };
    let counter_id = ui_state.add_widget(Box::new(counter_widget_1), Some(row_id));

    let add_button_widget = ButtonWidget { text: " + ".to_string(), text_scale: 1.0  };

    let add_button_id = ui_state.add_widget(Box::new(add_button_widget), Some(row_id));


    let sub_button_widget = ButtonWidget { text: " - ".to_string(), text_scale: 1.0  };

    let sub_button_id = ui_state.add_widget(Box::new(sub_button_widget), Some(row_id));


    let slider_widget = SliderWidget::new(None, None, 1.0, 0.0, 1.0);
    let smoothness_slider_id = ui_state.add_widget(Box::new(slider_widget), Some(row_id));
    ui_state.set_alignment_x(smoothness_slider_id, AlignmentX::Center);

    let scale = 2.0;
    let scale_widget = SliderWidget::new(None, None, scale, 0.0, 25.0);
    let scale_slider_id = ui_state.add_widget(Box::new(scale_widget), Some(row_id));

    ui_state.set_alignment_x(scale_slider_id, AlignmentX::Right);

    (UiInfo {scale: scale as i32, counter_id, add_button_id, sub_button_id, count, smoothness_slider_id, scale_slider_id}, ui_state)

}

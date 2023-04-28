use gl_lib::{gl};
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::{TextRenderer};
use gl_lib::widget_gui::widgets::*;

use sdl2::event;
use gl_lib::helpers;
use gl_lib::shader::BaseShader;
use std::fs;
use lipsum::lipsum;




static mut lipsum_text : Option<String> = None;
fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;


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


    let mut ui = Ui::default();

    let mut ui_info = UiInfo::default();

    ui_info.slider_ref = 1.0;

    add_slider_pannel(&mut ui, BoxContraint::new(viewport.w / 2, viewport.h), Position{ x:0, y: 0});

    let _root_box = BoxContraint::new(viewport.w, viewport.h);

    ui.layout_all();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut run = true;
    while run {

        for event in event_pump.poll_iter() {
            match event {
                event::Event::Quit {..} => {
                    run = false;
                },
                event::Event::KeyDown{keycode, ..} => {

                    // Reload text shader on R
                    if keycode == Some(sdl2::keyboard::Keycode::R) {


                        let vert_source = fs::read_to_string("assets/shaders/sdf_text_render.vert").unwrap();
                        let frag_source = fs::read_to_string("assets/shaders/sdf_text_render.frag").unwrap();

                        match BaseShader::new(&render_ctx.gl, &vert_source, &frag_source) {
                            Ok(new_shader) => {
                                println!("Reloaded sdf shader");
                                render_ctx.tr.change_shader(new_shader);
                            },
                            Err(msg) => {
                                println!("Shader load failed\n{:?}", msg);
                            }
                        };

                    } else if keycode == Some(sdl2::keyboard::Keycode::L) {

                        let geom = &ui.get_window("lipsum").unwrap().ui_state.geom[1];
                        let screen_space = render::transform_to_screen_space(geom, &render_ctx.viewport);

                        let font = &render_ctx.tr.font();
                        unsafe {
                            let lipsum_info = TextRenderer::render_box(font, &(lipsum_text.as_ref().unwrap()), screen_space.width, 1.0 );
                            println!("Text info: {:?}", lipsum_info);
                        }
                    } else {
                        if ui.windows.len() < 2 {
                            let window = add_keycode_pannel(BoxContraint::new(viewport.w/3, (viewport.h as f32 * 0.7) as i32),
                                                            Position {x: viewport.w / 3, y: 0});
                            ui.windows.insert(format!("lipsum"), window);
                        }
                    }

                    ui.layout_all();
                },
                _ => {}
            };

            ui.dispatch_event(&event);
        }


        ui.handle_widget_events(&mut ui_info);


        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.render(&mut render_ctx);

        let _slider_scale = ui_info.slider_ref as f32;

        // Render text that outside ui, is affected by out ui_info.slider_ref, that our slider also controlls
        //render_ctx.tr.render_text(render_ctx.gl, &format!("Hello {:.2}", slider_scale) , TextAlignment::default(), ScreenBox::new(0.0, 0.0, 1200.0, 700.0, 1200.0, 700.0), slider_scale);


        window.gl_swap_window();
    }

    Ok(())
}



fn handle_slider_outputs(ui_info: &mut UiInfo, event: WidgetOutput, named_widget_inputs: &mut NamedWidgetInputQueue) {
    if let Some(&v) = event.event.downcast_ref::<f64>() {
        ui_info.slider_ref = v;
        named_widget_inputs.send_value_to("lipsum", "text", v);
    }
}



fn add_keycode_pannel(draw_box: BoxContraint, root_pos: Position) -> UiWindow<UiInfo> {

    let mut ui_state = UiState::new();
    let r = RowWidget {} ;

    let row_id = ui_state.add_widget(Box::new(r), None);
    let text;
    unsafe {
        text = lipsum(400);
        lipsum_text = Some(text.clone());
    }

    let text_widget_1 = TextWidget { text, scale: 1.0 };
    let text_widget_id = ui_state.add_widget(Box::new(text_widget_1), Some(row_id));

    let mut window = UiWindow {
        ui_state,
        draw_box,
        root_pos,
        named_widgets: Default::default(),
        handler_functions: Default::default(),
    };


    window.named_widgets.insert(format!("text"), text_widget_id);

    window
}

#[derive(Default)]
struct UiInfo {
    slider_ref: f64,
    slider_id: Id,
}


fn add_slider_pannel(ui: &mut Ui<UiInfo>, draw_box: BoxContraint, root_pos: Position) {

    let mut ui_state = UiState::new();
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None);

    let slider_widget = SliderWidget::new(None, None, 1.0, 0.1, 9.0);

    let slider_id = ui_state.add_widget(Box::new(slider_widget), Some(row_id));


    let mut window = UiWindow {
        ui_state,
        draw_box,
        root_pos,
        named_widgets: Default::default(),
        handler_functions: Default::default(),
    };

    window.named_widgets.insert(format!("slider"), slider_id);
    window.handler_functions.insert(slider_id, handle_slider_outputs);

    ui.add_window("Slider_window".to_string(), window);
}

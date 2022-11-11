use gl_lib::{gl, ScreenBox};
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::TextAlignment;
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::{dispatch_event};
use sdl2::event;
use gl_lib::helpers;


fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;


    // Set background color to white
    unsafe {
        gl.ClearColor(1.0, 1.0, 1.0, 1.0);
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
    let (mut ui_info, mut ui_state) = create_ui();

    let root_box = BoxContraint::new(viewport.w, viewport.h);
    layout_widgets(&root_box, &mut ui_state);

    let mut event_pump = sdl.event_pump().unwrap();

    let mut run = true;
    while run {


        for event in event_pump.poll_iter() {
            match event {
                event::Event::Quit {..} => {
                    run = false;
                },
                _ => {}
            };

            // dispatch sdl events to widget
            dispatch_event(&mut ui_state, &event);
        }

        // Handle each widget output to update our state accordingly
        while let Some(event) = ui_state.poll_widget_outputs() {
            handle_widget_outputs(&mut ui_info, event);
        }

        // Layout widget. could be skipped since we don't resize any widgets
        layout_widgets(&root_box, &mut ui_state);


        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render::render_ui(&ui_state, &mut render_ctx);

        // Render text that outside ui, is affected by out ui_info.slider_ref, that our slider also controlls
        render_ctx.tr.render_text(render_ctx.gl, "Hello", TextAlignment::default(), ScreenBox::new(00.0, 00.0, 1200.0, 700.0, 1200.0, 700.0), ui_info.slider_ref as f32);


        window.gl_swap_window();
    }

    Ok(())
}


fn handle_widget_outputs(ui_info: &mut UiInfo, event: WidgetOutput) {
    if event.widget_id == ui_info.slider_id {
        if let Some(&v) = event.event.downcast_ref::<f64>() {
            ui_info.slider_ref = v;
        }
    }
}


struct UiInfo {
    slider_ref: f64,
    slider_id: Id,
}


fn create_ui() -> (UiInfo, UiState) {

    let mut ui_state = UiState::new();
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None);

    let slider_ref = 4.0;


    let slider_widget = SliderWidget::new(None, None, 4.0, 0.5, 15.0);

    let slider_id = ui_state.add_widget(Box::new(slider_widget), Some(row_id));


    (UiInfo { slider_id, slider_ref }, ui_state)

}

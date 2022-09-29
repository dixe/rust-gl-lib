use gl_lib::{gl, ScreenBox};
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::{TextRenderer, TextAlignment, TextAlignmentX::*};
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::{dispatch_events, run_listeners};
use gl_lib::shader::rounded_rect_shader::RoundedRectShader;
use gl_lib::shader::circle_shader::CircleShader;
use gl_lib::objects::square::Square;
use sdl2::event;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use gl_lib::helpers;


fn main() -> Result<(), failure::Error> {

    let mut sdl_setup = helpers::setup_sdl()?;
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

        // dispatch events
        for event in event_pump.poll_iter() {

            match event {
                event::Event::Quit {..} => {
                    run = false;
                },
                _ => {}
            };


            dispatch_events(&mut ui_state, &event);
        }

        // handle events for each widget
        while let Some(event) = ui_state.poll_widget_event() {
            handle_widget_event(&mut ui_info, event, &mut ui_state.queues);
        }

        run_listeners(&mut ui_state);

        layout_widgets(&root_box, &mut ui_state);

        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render::render_ui(&ui_state, &mut render_ctx);


        // Render text that outside ui, is affected by out ui_info.slider_ref, that our slider also controlls
        render_ctx.tr.render_text(render_ctx.gl, "Hello", TextAlignment::default(), ScreenBox::new(00.0, 00.0, 1200.0, 700.0, 1200.0, 700.0), *ui_info.slider_ref.borrow() as f32);

        window.gl_swap_window();
    }

    Ok(())
}

fn write_count(info: &UiInfo) {
    //println!("Counter is {:?}", info.counter_ref);
}

fn handle_widget_event(ui_info: &mut UiInfo, event: DispatcherEvent, widget_queues: &mut [EventQueue]) {
    if event.target_id == ui_info.slider_id {
        //println!("slider_event {:?}",event)
    }
}


struct UiInfo {
    slider_ref: Rc<RefCell::<f64>>,
    slider_id: Id,
}


fn create_ui() -> (UiInfo, UiState) {

    let mut ui_state = UiState::new();
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None);

    let slider_ref = Rc::new(RefCell::new(4.0));


    let slider_widget = SliderWidget::new(None, None, Rc::clone(&slider_ref), 0.5, 15.0);

    let slider_id = ui_state.add_widget(Box::new(slider_widget), Some(row_id));


    (UiInfo { slider_id, slider_ref }, ui_state)

}

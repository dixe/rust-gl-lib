use gl_lib::gl;
use gl_lib::helpers;
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::TextRenderer;
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::dispatch_events;
use gl_lib::shader::rounded_rect_shader::RoundedRectShader;
use gl_lib::objects::square::Square;
use sdl2::event;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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


    let mut ui_state = create_ui();


    let root_box = BoxContraint::new(viewport.w, viewport.h);
    layout_widgets(&root_box, &mut ui_state);

    let mut event_pump = sdl.event_pump().unwrap();

    loop {
        layout_widgets(&root_box, &mut ui_state);
        // dispatch events

        for event in event_pump.poll_iter() {
            dispatch_events(&mut ui_state, &event);
        }



        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render::render_ui(&ui_state, &mut render_ctx);

        window.gl_swap_window();
    }
}



fn create_ui() -> UiState {

    let mut ui_state = UiState::new();
    let counter_widget_1 = CounterWidget { count: Rc::new(RefCell::new(0)) };
    let counter_id = ui_state.add_widget(Box::new(counter_widget_1), None);

    // Add dispatcher for counter
    //ui_state.set_widget_dispatcher(counter_id, Box::new(counter_dispatcher));

    // Add listener for counter
    //ui_state.set_widget_listener(counter_id, Box::new(counter_listener));

    ui_state

}

/*

*/
/*
fn counter_listener(event: Box::<dyn Any>, ctx: &mut ListenerCtx) {

    let widget = &mut ctx.widgets[ctx.id];

    widget.handle_event(event);
}
*/

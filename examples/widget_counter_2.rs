use gl_lib::gl;
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::TextRenderer;
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::{handle_events, run_listeners};
use sdl2::event;
use std::any::Any;

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

    let mut event_pump = sdl.event_pump().unwrap();

    loop {
        layout_widgets(&root_box, &mut ui_state);
        // handle events

        for event in event_pump.poll_iter() {
            handle_events(&mut ui_state, &event);
        }

        while let Some(event) = ui_state.poll_widget_event() {
            handle_widget_event(2, 1, event, &mut ui_state.queues);
        }



        run_listeners(&mut ui_state);
        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render::render_ui(&ui_state, &mut render_ctx);

        window.gl_swap_window();
    }
}


fn handle_widget_event(add_button_id: Id, counter_id: Id, event: HandlerEvent, widget_queues: &mut [EventQueue]) {

    match event.target_id {
        add_button_id => {
            println!("add to counter {:?}", counter_id);
            widget_queues[counter_id].push_back(Box::new(1));
        }

    };

}


fn create_ui() -> UiState {


    let mut ui_state = UiState::new();
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None, None);

    let counter_widget_1 = CounterWidget { count: 0 };
    let counter_id = ui_state.add_widget(Box::new(counter_widget_1), Some(row_id), None);

    // Add listener for counter
    ui_state.set_widget_listener(counter_id, Box::new(counter_listener));


    let button_widget = ButtonWidget::<Id> { text: " + ".to_string(), text_scale: 1.0, state: counter_id  };

    let button_id = ui_state.add_widget(Box::new(button_widget), Some(row_id), None);

    // Add handler for button
    ui_state.set_widget_handler(button_id, Box::new(button_handler));

    // Add listener for button
    ui_state.set_widget_listener(button_id, Box::new(button_listener));


    ui_state

}



fn counter_listener(event: Box::<dyn Any>, ctx: &mut ListenerCtx) {

    let widget = &mut ctx.widgets[ctx.id];

    widget.handle_event(event);
}




fn button_handler(event: &event::Event, self_id: Id, queue: &mut HandlerQueue) {
    use event::Event::*;
    match event {
        MouseButtonUp { mouse_btn, ..} => {
            println!("{:?}", event);
            // TODO: only on left click
            queue.push_back(HandlerEvent { target_id: self_id, event: Box::new(1)});
        },
        _ => {}
    };


}


fn button_listener(event: Box::<dyn Any>, ctx: &mut ListenerCtx) {

    println!("{:?}", event);
    let widget = &mut ctx.widgets[ctx.id];

    widget.handle_event(event);
}

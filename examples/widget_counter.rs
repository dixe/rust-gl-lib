use gl_lib::{gl, objects::square, shader};
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::TextRenderer;
use gl_lib::widget_gui::widgets::CounterWidget;
use gl_lib::widget_gui::event_handling::handle_events;
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

        // handle events

        for event in event_pump.poll_iter() {
            handle_events(&mut ui_state, &event);
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
    let counter_widget_1 = CounterWidget { count: 0 };
    let counter_id = ui_state.add_widget(Box::new(counter_widget_1), None, None);

    // Add handler for counter
    ui_state.set_widget_handler(counter_id, Box::new(counter_handler));

    // Add listener for counter
    ui_state.set_widget_listener(counter_id, Box::new(counter_listener));

    ui_state

}


fn counter_handler(event: &event::Event, queue: &mut EventQueue) {
    use event::Event::*;
    match event {
        TextInput { text, ..} => {
            match text.as_str() {
                " " => {
                    queue.push_back(Box::new("stre".to_string()));
                    queue.push_back(Box::new(42i32));
                },
                _ => {}
            }
        }
        _ => {}
    };

}


fn counter_listener(event: &mut dyn Any, ctx: &mut ListenerCtx) {

    let mut widget = &mut ctx.widgets[ctx.id];

    widget.handle_event(event);

    println!("{:?}", event);


}

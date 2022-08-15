use gl_lib::gl;
use failure;
use gl_lib::widget_gui::*;
use gl_lib::text_rendering::text_renderer::TextRenderer;
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::{dispatch_events, run_listeners};
use gl_lib::shader::rounded_rect_shader::RoundedRectShader;
use gl_lib::objects::square::Square;
use sdl2::event;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;



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


    let (mut ui_info, mut ui_state) = create_ui();

    let font = Default::default();
    let mut text_renderer = TextRenderer::new(&gl, font) ;
    text_renderer.setup_blend(&gl);
    let mut rrs = RoundedRectShader::new(&gl).unwrap();

    let square = Square::new(&gl);

    let mut render_ctx = render::RenderContext {
        gl: &gl,
        viewport: &viewport,
        tr: &mut text_renderer,
        rounded_rect_shader: &mut rrs,
        render_square: &square
    };


    // Set background color to white
    unsafe {
        gl.ClearColor(1.0, 1.0, 1.0, 1.0);
    }


    let root_box = BoxContraint::new(viewport.w, viewport.h);
    layout_widgets(&root_box, &mut ui_state);

    let mut event_pump = sdl.event_pump().unwrap();

    loop {

        // dispatch events

        for event in event_pump.poll_iter() {
            dispatch_events(&mut ui_state, &event);
        }

        // handle events for each widget
        while let Some(event) = ui_state.poll_widget_event() {
            handle_widget_event(&mut ui_info, event, &mut ui_state.queues);
        }

        run_listeners(&mut ui_state);

        //write_count(&ui_info);


        layout_widgets(&root_box, &mut ui_state);

        // rendering
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render::render_ui(&ui_state, &mut render_ctx);

        window.gl_swap_window();
    }
}




fn write_count(info: &UiInfo) {
    println!("Counter is {:?}", info.counter_ref);
}


fn handle_widget_event(ui_info: &mut UiInfo, event: DispatcherEvent, widget_queues: &mut [EventQueue]) {
    if event.target_id == ui_info.add_button_id {
        widget_queues[ui_info.counter_id].push_back(Box::new(1));;
    }

    if event.target_id == ui_info.sub_button_id {

    }

}

struct UiInfo {
    counter_ref: Rc<RefCell::<i32>>,
    counter_id: Id,
    add_button_id: Id,
    sub_button_id: Id,
}


fn create_ui() -> (UiInfo, UiState) {


    let mut ui_state = UiState::new();
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None, None);

    let counter_ref = Rc::new(RefCell::new(0));

    let counter_widget_1 = CounterWidget { count: Rc::clone(&counter_ref) };
    let counter_id = ui_state.add_widget(Box::new(counter_widget_1), Some(row_id), None);

    // Add listener for counter
    ui_state.set_widget_listener(counter_id, Box::new(counter_listener));


    let add_button_widget = ButtonWidget { text: " + ".to_string(), text_scale: 1.0  };

    let add_button_id = ui_state.add_widget(Box::new(add_button_widget), Some(row_id), None);

    // Add dispatcher for add button
    ui_state.set_widget_dispatcher(add_button_id, Box::new(button_dispatcher));


    let sub_button_widget = ButtonWidget { text: " - ".to_string(), text_scale: 1.0  };

    let sub_button_id = ui_state.add_widget(Box::new(sub_button_widget), Some(row_id), None);

    // Add dispatcher for sub button
    ui_state.set_widget_dispatcher(sub_button_id, Box::new(button_dispatcher));


    (UiInfo {counter_id, add_button_id, sub_button_id, counter_ref }, ui_state)

}



fn counter_listener(event: Box::<dyn Any>, ctx: &mut ListenerCtx) {

    let widget = &mut ctx.widgets[ctx.id];

    widget.handle_event(event);
}




fn button_dispatcher(event: &event::Event, self_id: Id, queue: &mut DispatcherQueue) {
    use event::Event::*;
    match event {
        MouseButtonUp { mouse_btn, ..} => {
            // TODO: only on left click
            queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
        },
        _ => {}
    };
}

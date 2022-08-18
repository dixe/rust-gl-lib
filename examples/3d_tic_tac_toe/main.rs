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
        .window("3D tic tac toe", width, height)
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


    let root_box = BoxContraint::new(viewport.w/2, viewport.h/2);

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



//        panic!("STOP");
        window.gl_swap_window();
    }
}





fn handle_widget_event(ui_info: &mut UiInfo, event: DispatcherEvent, widget_queues: &mut [EventQueue]) {
    println!("event: {:?}", event);

}

struct UiInfo {
    grid_info: GridInfo
}


fn create_ui() -> (UiInfo, UiState) {


    let mut ui_state = UiState::new();

    let grid_info = create_2d_grid(&mut ui_state);

    (UiInfo {grid_info }, ui_state)

}

#[derive(Debug)]
struct GridInfo {
    ids: [Id; 4*4]
}


fn create_2d_grid(ui_state: &mut UiState) -> GridInfo {

    let col = ColumnWidget {};

    let mut attribs = LayoutAttributes::default();
    attribs = attribs.flex_width(1)
        .flex_height(1);

    let col_id = ui_state.add_widget(Box::new(col), None);
    ui_state.set_widget_attributes(col_id, attribs.clone());
    let mut grid_info = GridInfo { ids: [0;4*4] };
    let mut i = 0;
    for _ in 0..4 {
        let row = RowWidget {};
        let row_id = ui_state.add_widget(Box::new(row), Some(col_id));
        ui_state.set_widget_attributes(row_id, attribs.clone());

        for _ in 0..4 {
            let button_widget = ButtonWidget { text: format!("{}", i).to_string(), text_scale: 1.0  };

            let button_id = ui_state.add_widget(Box::new(button_widget), Some(row_id));


            ui_state.set_widget_attributes(button_id, attribs.clone());

            grid_info.ids[i] = button_id;
            i += 1;
        }
    }

    grid_info

}


// Can this be on the button is self? Basically buttons always just react to mouse up event. Or make it just default
fn button_dispatcher(event: &event::Event, self_id: Id, queue: &mut DispatcherQueue) {
    use event::Event::*;
    match event {
        MouseButtonUp {..} => {
            // TODO: only on left click
            queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
        },
        _ => {}
    };
}

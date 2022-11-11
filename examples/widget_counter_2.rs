use gl_lib::gl;
use gl_lib::helpers;
use failure;
use gl_lib::widget_gui::*;
use gl_lib::widget_gui::widgets::*;
use gl_lib::widget_gui::event_handling::{dispatch_event, dispatch_widget_inputs};



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

    loop {

        // dispatch events

        for event in event_pump.poll_iter() {
            dispatch_event(&mut ui_state, &event);
        }

        dispatch_widget_inputs(&mut ui_state);

        // handle events for each widget
        while let Some(event) = ui_state.poll_widget_outputs() {
            handle_widget_outputs(&mut ui_info, event, &mut ui_state.widget_input_queue);
        }

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
    println!("Count is {:?}", info.count);
}


fn handle_widget_outputs(ui_info: &mut UiInfo, event: WidgetOutput, widget_input_queue: &mut WidgetInputQueue) {
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
}

struct UiInfo {
    count: i32,
    counter_id: Id,
    add_button_id: Id,
    sub_button_id: Id,
}


fn create_ui() -> (UiInfo, UiState) {


    let mut ui_state = UiState::new();
    let row = RowWidget {};

    let row_id = ui_state.add_widget(Box::new(row), None);

    let count = 0;

    let counter_widget_1 = CounterWidget{ count };
    let counter_id = ui_state.add_widget(Box::new(counter_widget_1), Some(row_id));

    let add_button_widget = ButtonWidget { text: " + ".to_string(), text_scale: 1.0  };

    let add_button_id = ui_state.add_widget(Box::new(add_button_widget), Some(row_id));


    let sub_button_widget = ButtonWidget { text: " - ".to_string(), text_scale: 1.0  };

    let sub_button_id = ui_state.add_widget(Box::new(sub_button_widget), Some(row_id));


    (UiInfo {counter_id, add_button_id, sub_button_id, count }, ui_state)

}

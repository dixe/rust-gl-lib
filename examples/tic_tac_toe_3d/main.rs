use failure;
use gl_lib::widget_gui::*;

use gl_lib::widget_gui::widgets::*;
use gl_lib::{gl};
use gl_lib::widget_gui::event_handling::{dispatch_event};

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


    let _render_ctx = render::RenderContext {
        gl: gl,
        viewport: &viewport,
        tr: &mut widget_setup.text_renderer,
        rounded_rect_shader: &mut widget_setup.rounded_rect_shader,
        render_square: &widget_setup.render_square,
        circle_shader: &mut widget_setup.circle_shader
    };

    // Setup widget ui


    let (mut ui_info, mut ui_state) = create_ui(&gl);

    let mut render_ctx = render::RenderContext {
        gl: gl,
        viewport: &viewport,
        tr: &mut widget_setup.text_renderer,
        rounded_rect_shader: &mut widget_setup.rounded_rect_shader,
        render_square: &widget_setup.render_square,
        circle_shader: &mut widget_setup.circle_shader
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
            dispatch_event(&mut ui_state, &event);
        }

        // handle events for each widget
        while let Some(event) = ui_state.poll_widget_outputs() {
            handle_widget_outputs(&mut ui_info, event);
        }

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





fn handle_widget_outputs(_ui_info: &mut UiInfo, event: WidgetOutput) {
    println!("event: {:?}", event);

}

struct UiInfo {
    grid_info: GridInfo
}


fn create_ui(gl: &gl::Gl) -> (UiInfo, UiState) {


    let mut ui_state = UiState::new(gl);

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

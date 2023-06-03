use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::{*};
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::{PolygonOptions};
use gl_lib::collision2d::polygon::{Polygon};
use deltatime;



type V2 = na::Vector2::<f32>;

enum State {
    View,
    Edit(usize)
}

fn main() -> Result<(), failure::Error> {

    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    ui.drawer2D.font_cache.fonts_path = Some("assets/fonts/".to_string());

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut polygons = vec![
        Polygon {
            vertices: vec![V2::new(32.0, 64.0),
                           V2::new(300.0, 32.0),
                           V2::new(32.0, 32.0),
            ],
        },
        Polygon {
            vertices: vec![V2::new(32.0, 70.0),
                           V2::new(200.0, 32.0),
                           V2::new(64.0, 32.0),
            ]
        }
    ];


    let mut t = 0.5;

    let mut delta_time = deltatime::Deltatime::new();
    let mut play = false;

    let mut animation_seconds = 1.0;

    let mut options : Vec::<PolygonOptions> = vec![Default::default(), Default::default()];

    options[0].transform.scale = 1.0;
    options[1].transform.scale = 1.0;

    let mut state = State::View;

    // create windows, and set their initial position
    ui.window_begin("t_start");
    ui.window_end("t_start");
    ui.window_begin("t_end");
    ui.window_end("t_end");

    let w_start = ui.windows.get_mut(&1).unwrap();
    w_start.base_container_context.anchor_pos.x = 650;
    w_start.base_container_context.anchor_pos.y = 50;

    let w_end = ui.windows.get_mut(&2).unwrap();
    w_end.base_container_context.anchor_pos.x = 900;
    w_end.base_container_context.anchor_pos.y = 50;

    let mut tmp_polygon = Polygon { vertices: vec![]};

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        delta_time.update();
        let dt = delta_time.time();

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui);


        // UI
        match state {
            State::View => {
                // draw polygons
                ui.view_polygon(&mut polygons[0], &mut options[0].transform);
                ui.view_polygon(&mut polygons[1], &mut options[1].transform);

                if ui.button(&format!("Edit 0 ({})", polygons[0].vertices.len())) {
                    state = State::Edit(0)
                }
                if ui.button(&format!("Edit 1 ({})", polygons[1].vertices.len())) {
                    state = State::Edit(1)
                }

                // ui for t, and star and end transforms
                ui.label("t");
                ui.slider(&mut t, 0.0, 1.0);

                ui.slider(&mut animation_seconds, 0.1, 3.0);
                let txt = if play { "Pause"} else { "Play"};
                if ui.button(txt) {
                    play = !play;
                }

                if play {
                    t += dt * 1.0 / animation_seconds;
                    if t > 1.0 {
                        t = 0.0;
                    }
                }

                ui.window_begin("t_start");
                center_transform_ui(&mut ui, &mut polygons[0]);
                ui.window_end("t_start");


                ui.window_begin("t_end");
                center_transform_ui(&mut ui, &mut polygons[1]);
                ui.window_end("t_end");

                if let Some(transform) = Polygon::interpolate(&mut tmp_polygon, &polygons[0], &options[0].transform, &polygons[1], &options[1].transform, t) {
                    // draw tmp_polygon
                    ui.view_polygon(&tmp_polygon, &transform);
                }
            },
            State::Edit(i) => {
                ui.polygon_editor(&mut polygons[i], &mut options[i]);
                if ui.button("View") {
                    state = State::View;
                }
            }
        }

        window.gl_swap_window();
    }
}


fn center_transform_ui(ui: &mut Ui, p: &mut Polygon) {

    let mut c = p.center();

    ui.label("center");
    ui.newline();

    let mut x = c.x;
    let mut y = c.y;
    if ui.slider2d(&mut x, &mut y, 0.0, 1100.0, 0.0, 700.0) {
        c.x = x;
        c.y = y;
        p.set_center(c);
    }

}

fn handle_inputs(ui: &mut Ui) {

    for _e in &ui.frame_events {

    }
}

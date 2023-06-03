use gl_lib::{gl, na, helpers, color::Color};
use gl_lib::imode_gui::drawer2d::{*};
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::widgets::PolygonTransform;
use sdl2::event;
use std::collections::HashSet;
use gl_lib::collision2d::gjk;
use gl_lib::collision2d::polygon::{self, Polygon, ComplexPolygon};


type V2 = na::Vector2::<f32>;
type V3 = na::Vector3::<f32>;

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

    let mut polygon = Polygon {
        vertices: vec![V2::new(32.0, 0.0),
                       V2::new(0.0, 32.0),
                       V2::new(32.0, 32.0),
        ]
    };

    let mut t_start = PolygonTransform::default();
    t_start.scale = 1.0;

    let mut t_end = PolygonTransform::default();
    t_end.scale = 2.0;
    let mut t = 0.5;

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui);

        // ui for t, and star and end transforms
        ui.label("t");
        ui.slider(&mut t, 0.0, 1.0);

        ui.window_begin("t_start");
        transform_ui(&mut ui, &mut t_start);
        ui.window_end("t_start");


        ui.window_begin("t_end");
        transform_ui(&mut ui, &mut t_end);
        ui.window_end("t_end");

        // draw polygons

        ui.view_polygon(&polygon, &t_start);
        ui.view_polygon(&polygon, &t_end);

        interpolated(&mut ui, &polygon, &t_start, &t_end, t);


        window.gl_swap_window();
    }
}


fn interpolated(ui: &mut Ui, polygon: &Polygon, t_start: &PolygonTransform, t_end: &PolygonTransform, t: f32) {

    let transform = PolygonTransform {
        translation: lerp_v2(t_start.translation, t_end.translation, t),
        rotation: lerp(t_start.rotation, t_end.rotation, t),
        scale: lerp(t_start.scale, t_end.scale, t),
    };


    ui.view_polygon(&polygon, &transform);

}

fn lerp_v2(a: V2, b: V2, t: f32) -> V2 {
    (1.0-t) * a + b*t
}



fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0-t) * a + b*t
}

fn transform_ui(ui: &mut Ui, transform: &mut PolygonTransform) {


    ui.label("translation");
    ui.newline();
    let mut x = transform.translation.x;
    let mut y = transform.translation.y;
    ui.slider2d(&mut x, &mut y, 0.0, 1100.0, 0.0, 700.0);

    transform.translation.x = x;
    transform.translation.y = y;

    ui.label("trans.x");
    ui.slider(&mut transform.translation.x, 0.0, 1100.0);

    ui.newline();
    ui.label("trans.y");
    ui.slider(&mut transform.translation.y, 0.0, 700.0);

    ui.newline();

    ui.label("scale");
    ui.slider(&mut transform.scale, 0.50, 10.0);

}

fn handle_inputs(ui: &mut Ui) {

    use event::Event::*;
    use sdl2::keyboard::Keycode;

    for e in &ui.frame_events {

    }
}

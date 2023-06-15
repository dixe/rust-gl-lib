use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::{*};
use gl_lib::imode_gui::ui::*;



use gl_lib::collision2d::polygon::{Polygon, PolygonTransform};
use core::ops::Mul;


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

    let polygon = Polygon {
        vertices: vec![V2::new(32.0, 0.0),
                       V2::new(0.0, 32.0),
                       V2::new(32.0, 32.0),
        ]
    };

    let mut t_start = PolygonTransform::default();
    t_start.scale = 1.0;
    t_start.translation.x = 10.0;
    t_start.translation.y = 200.0;

    let mut t_end = PolygonTransform::default();
    t_end.scale = 2.0;
    t_end.translation.x = 300.0;
    t_end.translation.y = 300.0;
    let mut t = 0.5;

    let mut play = false;

    let mut animation_seconds = 1.0;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        let dt = ui.dt();

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui);

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
        translation: lerp(t_start.translation, t_end.translation, t),
        rotation: lerp(t_start.rotation, t_end.rotation, t),
        scale: lerp(t_start.scale, t_end.scale, t),
        flip_y: false
    };


    ui.view_polygon(&polygon, &transform);

}



fn lerp<T: std::ops::Mul<f32, Output = T> + std::ops::Add<Output = T>>(a: T, b: T, t: f32) -> T  where f32: Mul<T, Output = T> {// where T: core::ops::Mul<T{
    let t1 = ease_out_back(t);

    let x :T = (1.0 - t1) * a;
    let y :T = b * t1;

    x + y
}

fn ease_out_circ(x: f32) -> f32 {
    return f32::sqrt(1.0 - f32::powi(x - 1.0, 2));
}

fn ease_out_bounce(x: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if x < 1.0 / d1 {
        return n1 * x * x;
    } else if x < 2.0 / d1 {
        return n1 * (x - 1.5 / d1) * x + 0.75;
    } else if x < 2.5 / d1 {
        return n1 * (x - 2.25 / d1) * x + 0.9375;
    } else {
        return n1 * (x - 2.625 / d1) * x + 0.984375;
    }
}

fn ease_out_back(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;

    return 1.0 + c3 * f32::powi(x - 1.0, 3) + c1 * f32::powi(x - 1.0, 2);
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

    
    

    for _e in &ui.frame_events {

    }
}

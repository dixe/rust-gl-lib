use gl_lib::{gl, helpers, na};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use std::collections::VecDeque;

fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut curve = Curve {
        samples: 50,
        points: vec! [
            na::Vector2::<f64>::new(50.0, 400.0),
            na::Vector2::<f64>::new(100.0, 300.0),
            na::Vector2::<f64>::new(200.0, 200.0),
            na::Vector2::<f64>::new(300.0, 300.0),
            na::Vector2::<f64>::new(400.0, 350.0),
        ],
        draw_helpers: false,
        draw_all: false
    };


    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        ui.consume_events(&mut event_pump);

        ui.slider(&mut curve.samples, 2, 50);

        ui.label("Draw helper lines");
        ui.checkbox(&mut curve.draw_helpers);
        ui.label("Draw all lines");
        ui.checkbox(&mut curve.draw_all);

        draw_curve(&mut curve, &mut ui);

        window.gl_swap_window();
    }
}

fn draw_curve(curve: &mut Curve, ui: &mut Ui) {

    for p in &mut curve.points {
        let mut pos = Pos {x: p.x as i32, y: p.y as i32 };
        ui.drag_point(&mut pos);
        p.x = pos.x as f64;
        p.y = pos.y as f64;
    }

    if curve.points.len() < 2 {
        return;
    }

    let mut prev = curve.points[0];
    let draw_i = curve.samples / 2;
    for i in 0..curve.samples {

        let t = (1 + i) as f64 / curve.samples as f64;

        let draw = curve.draw_all || (curve.draw_helpers && i == curve.samples/ 2);

        let new_p = calc_point_recursive(t, &curve.points, ui, draw);

        ui.drawer2D.line(prev.x as i32, prev.y as i32, new_p.x as i32, new_p.y as i32, 3);

        prev = new_p;
    }
}

fn calc_point_recursive(t: f64, points: &Vec::<na::Vector2::<f64>>, ui: &mut Ui, draw: bool) -> na::Vector2::<f64> {

    let mut ps = vec![];

    // setup
    for i in 0..points.len() {
        ps.push(points[i]);
    }

    let mut iter_len = ps.len();
    let mut start = 0;

    let mut run = true;

    while iter_len > 0 {
        for i in start..(start + iter_len - 1) {
            let p0 = ps[i];
            let p1 = ps[i + 1];
            let new_p = lerp(t, p0, p1);

            ps.push(new_p);
        }
        start += iter_len;
        iter_len -= 1;
    }

    if draw {
        for i in 0..(ps.len() - 1) {
            let p0 = ps[i];
            let p1 = ps[i + 1];
            ui.drawer2D.line(p0.x as i32, p0.y as i32, p1.x as i32, p1.y as i32, 2);
        }
    }

    ps[ps.len() - 1]
}

fn lerp(t: f64, p0: na::Vector2::<f64>, p1: na::Vector2::<f64>) -> na::Vector2::<f64> {
    (1.0 - t) * p0 + t * p1
}


struct Curve {
    samples: usize,
    points: Vec::<na::Vector2::<f64>>,
    draw_helpers: bool,
    draw_all: bool
}

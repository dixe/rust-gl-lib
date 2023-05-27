use gl_lib::{gl, na, helpers, color::Color};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use sdl2::event;
use std::collections::HashSet;

type V2 = na::Vector2::<f32>;
type V3 = na::Vector3::<f32>;

mod polygon;
use polygon::*;

mod line_segment_intersection;
use line_segment_intersection as lsi;

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


    let mut state = State {
        polygon: Polygon {
/*
            vertices: vec![V2::new(440.0, 217.0),
                           V2::new(647.0, 527.0),
                           V2::new(332.0, 563.0),
                           V2::new(520.0, 382.0),

            ],
*/

            vertices: vec![V2::new(600.0, 230.0),
                           V2::new(750.0, 380.0),
                           V2::new(580.0, 380.0),
                           V2::new(460.0, 322.0),
                           V2::new(589.0, 666.0),
                           V2::new(340.0, 666.0),
                           V2::new(484.0, 416.0),
                           V2::new(363.0, 281.0),
                           V2::new(428.0, 454.0),
                           V2::new(289.0, 567.0),
                           V2::new(285.0, 179.0),

            ],
        },
        sub_divisions: vec![],
        selected: Default::default(),
        mode: Mode::NewPoint,
        show_idx: false,
        show_pos: false
    };

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui, &mut state);

        ui.small_text("Tab to toggle vertices info");
        ui.newline();
        ui.small_text("s to calc triangulation");
        ui.newline();
        ui.small_text("ctrl-z to undo");
        ui.newline();
        ui.small_text("c to clear vertices");
        ui.newline();
        ui.small_text("a select all, esc deslects");


        render_poly(&mut ui, &mut state.polygon, &state.selected, state.show_pos, state.show_idx);

        render_sub_poly(&mut ui, &state.polygon, &state.sub_divisions);

        render_mode(&mut ui, &state.mode);

        render_selected(&mut ui, &mut state);

        window.gl_swap_window();
    }
}


fn render_selected(ui: &mut Ui, state: &mut State) {
    if state.selected.len() == 0 {
        return;
    }

    let mut avg = na::Vector2::<f32>::new(0.0, 0.0);

    for idx in &state.selected {
        avg.x += state.polygon.vertices[*idx].x;
        avg.y += state.polygon.vertices[*idx].y;
    }

    avg /= state.selected.len() as f32;

    let mut drag = na::Vector2::<i32>::new(avg.x as i32, avg.y as i32);

    ui.drag_point(&mut drag, 15.0);

    drag.x = drag.x - avg.x as i32;
    drag.y = drag.y - avg.y as i32;

    for idx in &state.selected {
        state.polygon.vertices[*idx].x += drag.x as f32;
        state.polygon.vertices[*idx].y += drag.y as f32;
    }

}

fn render_intersect(ui: &mut Ui, polygon: &Polygon) {
    // go over all side by side pairs and compare with every other side by side pair

    if polygon.vertices.len() <= 3 {
        return;
    }

    for p in polygon.intersections() {
        ui.drawer2D.circle(p.x, p.y, 7.0, Color::Rgb(250, 5, 5));
    }

}


fn render_mode(ui: &mut Ui, mode: &Mode) {

    match mode {
        Mode::Select(p1, p2) => {
            let tl = na::Vector2::new(f32::min(p1.x, p2.x), f32::max(p1.y, p2.y));
            let br = na::Vector2::new(f32::max(p1.x, p2.x), f32::min(p1.y, p2.y));

            ui.drawer2D.rounded_rect(tl.x, tl.y, br.x - tl.x, br.y - tl.y);
        },
        _ => {}
    }
}



fn render_sub_poly(ui: &mut Ui, polygon: &Polygon, sub_divisions: &Vec::<Vec::<usize>>) {


    for sub in sub_divisions {
        let len = sub.len();
        for idx in 0..len {
            let i1 = sub[idx];
            let i2 = sub[(idx + 1) % len];

            let p1 = polygon.vertices[i1];
            let p2 = polygon.vertices[i2];

            ui.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 7.0);


        }
    }
}

fn render_poly(ui: &mut Ui, poly: &mut Polygon, selected: &HashSet::<usize>, show_pos: bool, show_idx: bool) {

    let len = poly.vertices.len();

    for i in 0..len {
        let p1 = &mut poly.vertices[i];

        let mut r = 8.0;

        if selected.contains(&i) {
            r = 10.0;
        }


        let mut drag = na::Vector2::<i32>::new(p1.x as i32, p1.y as i32);
        ui.drag_point(&mut drag, r);
        p1.x = drag.x as f32;
        p1.y = drag.y as f32;

        if show_pos {
            ui.drawer2D.render_text(&format!("({:?})", p1), p1.x as i32, p1.y as i32 + 20, 14);
        }

        if show_idx {
            ui.drawer2D.render_text(&format!("{i}"), p1.x as i32, p1.y as i32, 14);
        }

        if i < len - 1 {
            let p1 = poly.vertices[i];
            let p2 = poly.vertices[i + 1];
            ui.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
        }

    }

    if len > 2 {
        let p1 = poly.vertices[len - 1];
        let p2 = poly.vertices[0];
        ui.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
    }
/*
    for sub_p in &poly.sub_polygons {
        for idx in &sub_p.indices {
            ui.drawer2D.rounded_rect(poly.vertices[*idx].x, poly.vertices[*idx].y, 10, 10);
        }
    };
*/
    render_intersect(ui, poly);
}

struct State {
    polygon: Polygon,
    sub_divisions: Vec::<Vec::<usize>>,
    selected: HashSet::<usize>,
    mode: Mode,
    show_idx: bool,
    show_pos: bool
}


pub enum Mode {
    NewPoint,
    Select(V2, V2),
}

fn handle_inputs(ui: &mut Ui, state: &mut State) {

    use event::Event::*;

    use sdl2::keyboard::Keycode;

    for e in ui.get_frame_inputs() {
        match e {
            MouseButtonDown {x, y, ..} => {
                let xf = *x as f32;
                let yf = *y as f32;
                state.mode = Mode::Select(V2::new(xf, yf), V2::new(xf, yf));
            },

            MouseButtonUp {x, y, ..} => {
                let xf = *x as f32;
                let yf = *y as f32;

                let new = V2::new(xf, yf);
                match state.mode {
                    Mode::NewPoint => {
                        state.selected.clear();
                        state.polygon.vertices.push(new);
                    },
                    Mode::Select(start, _) => {
                        state.selected.clear();
                        let diff = new - start;

                        if diff.magnitude() < 3.0 {
                            state.polygon.vertices.push(new);
                            state.mode = Mode::NewPoint;
                        } else {
                            let tl = na::Vector2::new(f32::min(new.x, start.x), f32::max(new.y, start.y));
                            let br = na::Vector2::new(f32::max(new.x, start.x), f32::min(new.y, start.y));

                            for i in 0..state.polygon.vertices.len() {
                                let p = &state.polygon.vertices[i];
                                if p.x >= tl.x && p.x <= br.x && p.y <= tl.y && p.y >= br.y {
                                    state.selected.insert(i);
                                }
                            }
                            state.mode = Mode::NewPoint;
                        }
                    },
                }
            },
            MouseMotion { x, y, mousestate, ..} => {
                let xf = *x as f32;
                let yf = *y as f32;

                let new = V2::new(xf, yf);
                match state.mode {
                    Mode::Select(start, _) => {
                        state.mode = Mode::Select(start, new);
                    },
                    _ => {}
                }
            },

            KeyDown { keycode: Some(Keycode::Escape), ..} => {
                state.selected.clear();
            },

            KeyDown { keycode: Some(Keycode::S), ..} => {
                println!("\n\n");
                let sub_divisions = calculate_subdivision(&mut state.polygon);
                state.sub_divisions.clear();
                for sub in sub_divisions {
                    state.sub_divisions.push(sub.indices);
                }

            },

            KeyDown { keycode: Some(Keycode::Tab), ..} => {
                state.show_idx = !state.show_idx
            },

            KeyDown { keycode: Some(Keycode::Backquote), ..} => {
                state.show_pos = !state.show_pos
            },

            KeyDown { keycode: Some(Keycode::C), ..} => {
                state.sub_divisions.clear();
                state.selected.clear();
                state.polygon = Polygon {
                    vertices: vec![],
                };
            },

            KeyDown { keycode: Some(Keycode::A), ..} => {
                for i in 0..state.polygon.vertices.len() {
                    state.selected.insert(i);
                }

            },
            KeyDown { keycode: Some(Keycode::T), ..} => {
                if !test1() {
                    println!("Test 1 failed");
                }
            },

            KeyDown { keycode: Some(Keycode::Z), keymod, ..} => {
                use sdl2::keyboard::Mod;
                if keymod.intersects(Mod::LCTRLMOD) {

                    state.sub_divisions.clear();
                    state.polygon.vertices.pop();
                    let len = state.polygon.vertices.len();

                    state.selected.remove(&len);
                }
            },

            _ => {}

        }
    }
}

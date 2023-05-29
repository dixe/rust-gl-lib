use gl_lib::{gl, na, helpers, color::Color};
use gl_lib::imode_gui::drawer2d::{self, *};
use gl_lib::imode_gui::ui::*;
use sdl2::event;
use std::collections::HashSet;

type V2 = na::Vector2::<f32>;
type V3 = na::Vector3::<f32>;

mod polygon;
use polygon::*;

mod line_segment_intersection;
use line_segment_intersection as lsi;

mod options;
mod gjk;


fn new_poly() -> Poly {

    let mut poly : Poly = Default::default();

    poly.polygon.vertices = vec![V2::new(580.0, 217.0),
                         V2::new(647.0, 527.0),
                         V2::new(332.0, 563.0),
                         V2::new(340.0, 230.0),
    ];

    poly.select_all();
    poly
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

    let mut options = options::Options::default();

    options.selected_v_color = Color::Rgb(56, 121, 24);
    let mut state = State {
        polygons: vec![ new_poly() ],
        polygon_mode: PolygonMode::Object(Some(0)),
        mode: Mode::NewPoint,
        options,
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
        ui.newline();


        options::options_ui(&mut ui, &mut state.options);

        let mut draggable = None;
        let mut selected = None;

        match state.polygon_mode {
            PolygonMode::Object(idx) => {

                selected = idx;
                ui.small_text("Object mode");

                if let Some(id) = idx {
                    if ui.button("to edit mode") {
                        state.polygon_mode = PolygonMode::Edit(id);
                    }
                }
                for i in 0..state.polygons.len() {

                    ui.newline();
                    if Some(i) == idx {
                        ui.body_text(&format!("{i}"));

                    } else {
                        if ui.button(&format!("{i}")) {
                            state.polygon_mode = PolygonMode::Object(Some(i));
                        }
                    }
                }

                ui.newline();
                if ui.button("Add new") {
                    state.polygons.push(new_poly());
                }

                if state.options.check_collision {
                    check_collision(&mut ui.drawer2D, &mut state.polygons);
                }


                // render polygons
                let mut i = 0;
                for poly in &mut state.polygons {
                    render_poly(&mut ui, poly, &state.options, Some(i) == draggable, Some(i) == selected);
                    if render_selected(&mut ui, poly) {
                        state.polygon_mode = PolygonMode::Object(Some(i));
                    }

                    render_sub_poly(&mut ui, poly);
                    i += 1;
                }
            },

            PolygonMode::Edit(idx) => {
                draggable = Some(idx);
                ui.small_text("Edit mode");
                if ui.button("to obj mode") {
                    state.polygon_mode = PolygonMode::Object(Some(idx));
                    for poly in &mut state.polygons {
                        poly.select_all();
                    }
                }

                // render polygon to edit, hide the others
                render_poly(&mut ui, &mut state.polygons[idx], &state.options, true, false);
                render_selected(&mut ui, &mut state.polygons[idx]);

            }
        }
        render_mode(&mut ui, &state.mode);
        window.gl_swap_window();
    }
}


fn render_selected(ui: &mut Ui, poly: &mut Poly) -> bool {
    if poly.selected.len() == 0 {
        return false
    }

    let mut avg = na::Vector2::<f32>::new(0.0, 0.0);

    for idx in &poly.selected {
        avg.x += poly.polygon.vertices[*idx].x;
        avg.y += poly.polygon.vertices[*idx].y;
    }

    avg /= poly.selected.len() as f32;

    let mut drag = na::Vector2::<i32>::new(avg.x as i32, avg.y as i32);

    let res = ui.drag_point(&mut drag, 15.0);

    drag.x = drag.x - avg.x as i32;
    drag.y = drag.y - avg.y as i32;

    for idx in &poly.selected {
        poly.polygon.vertices[*idx].x += drag.x as f32;
        poly.polygon.vertices[*idx].y += drag.y as f32;
    }

    res
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



fn render_sub_poly(ui: &mut Ui, poly: &mut Poly) {

    let polygon = &poly.polygon;

    for sub in &poly.sub_divisions {
        let len = sub.len();
        for idx in 0..len {
            let i1 = sub[idx];
            let i2 = sub[(idx + 1) % len];

            let p1 = polygon.vertices[i1];
            let p2 = polygon.vertices[i2];

            ui.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 2.0);


        }
    }
}

fn render_poly(ui: &mut Ui, poly: &mut Poly, options: &options::Options, draggable: bool, selected: bool) {

    let polygon = &mut poly.polygon;
    let len = polygon.vertices.len();

    for i in 0..len {
        let p1 = &mut polygon.vertices[i];

        let mut r = 8.0;

        if poly.selected.contains(&i) {
            r = 10.0;
        }

        if selected {
            r = 10.0;
        }


        let mut drag = na::Vector2::<i32>::new(p1.x as i32, p1.y as i32);
        if draggable {
            ui.drag_point(&mut drag, r);
        } else {
            let mut color = options.v_color;
            if selected {
                color = options.selected_v_color;
            }

            ui.drawer2D.circle(drag.x, drag.y, r, color);
        }

        p1.x = drag.x as f32;
        p1.y = drag.y as f32;

        if options.show_pos {
            ui.drawer2D.render_text(&format!("({:?})", p1), p1.x as i32, p1.y as i32 + 20, 14);
        }

        if options.show_idx {
            ui.drawer2D.render_text(&format!("{i}"), p1.x as i32, p1.y as i32, 20);
        }

        if i < len - 1 {
            let p1 = polygon.vertices[i];
            let p2 = polygon.vertices[i + 1];
            ui.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
        }

    }

    if len > 2 {
        let p1 = polygon.vertices[len - 1];
        let p2 = polygon.vertices[0];
        ui.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
    }

    render_intersect(ui, &poly.polygon);
}

struct State {
    polygons: Vec::<Poly>,
    polygon_mode: PolygonMode,
    mode: Mode,
    options: options::Options
}

#[derive(Default)]
struct Poly {
    polygon: Polygon,
    sub_divisions: Vec::<Vec::<usize>>,
    selected: HashSet::<usize>,
}
impl Poly {

    fn select_all(&mut self) {
        for i in 0..self.polygon.vertices.len() {
            self.selected.insert(i);
        }
    }
}

enum PolygonMode {
    Object(Option<usize>),
    Edit(usize),
}

pub enum Mode {
    NewPoint,
    Select(V2, V2),
}

fn handle_inputs(ui: &mut Ui, state: &mut State) {

    use event::Event::*;

    use sdl2::keyboard::Keycode;

    for e in &ui.frame_events {

        match state.polygon_mode {

            PolygonMode::Object(_) => {
                handle_object_mode(&e, &mut state.mode, &mut state.options);
            },
            PolygonMode::Edit(idx) => {
                handle_edit_mode(&e, state.polygons.get_mut(idx).unwrap(), &mut state.mode);
            }
        }

        match e {
            MouseButtonDown {x, y, ..} => {
                let xf = *x as f32;
                let yf = *y as f32;
                state.mode = Mode::Select(V2::new(xf, yf), V2::new(xf, yf));
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

            KeyDown { keycode: Some(Keycode::S), ..} => {
                calc_and_set_subdivision(&mut state.polygons);
            },

            KeyDown { keycode: Some(Keycode::E), ..} => {
                match state.polygon_mode {
                    PolygonMode::Object(id) => {
                        if let Some(idx) = id {
                            if idx < state.polygons.len() {
                                state.polygon_mode = PolygonMode::Edit(idx);
                            }
                        }
                    },
                    _ => {}
                }
            },

            KeyDown { keycode: Some(Keycode::Tab), ..} => {
                state.options.show_idx = !state.options.show_idx
            },

            KeyDown { keycode: Some(Keycode::Backquote), ..} => {
                state.options.show_pos = !state.options.show_pos
            },

            KeyDown { keycode: Some(Keycode::T), ..} => {
                if !test1() {
                    println!("Test 1 failed");
                }
            },
            _ => {}

        }
    }
}


fn calc_and_set_subdivision(polygons: &mut Vec::<Poly>) {
    for poly in polygons {
        let sub_divisions = calculate_subdivision(&mut poly.polygon);
        poly.sub_divisions.clear();
        for sub in sub_divisions {
            poly.sub_divisions.push(sub.indices);
        }
    }
}





fn handle_edit_mode(event: &event::Event, poly: &mut Poly, mode: &mut Mode) {

    use event::Event::*;
    use sdl2::keyboard::Keycode;

    match event {
        MouseButtonUp {x, y, ..} => {
            let xf = *x as f32;
            let yf = *y as f32;

            let new = V2::new(xf, yf);
            match mode {
                Mode::NewPoint => {
                    poly.selected.clear();
                    poly.polygon.vertices.push(new);
                },
                Mode::Select(start, _) => {
                    poly.selected.clear();
                    let diff = new - *start;

                    if diff.magnitude() < 3.0 {
                        poly.polygon.vertices.push(new);
                        *mode = Mode::NewPoint;
                    } else {
                        let tl = na::Vector2::new(f32::min(new.x, start.x), f32::max(new.y, start.y));
                        let br = na::Vector2::new(f32::max(new.x, start.x), f32::min(new.y, start.y));

                        for i in 0..poly.polygon.vertices.len() {
                            let p = &poly.polygon.vertices[i];
                            if p.x >= tl.x && p.x <= br.x && p.y <= tl.y && p.y >= br.y {
                                poly.selected.insert(i);
                            }
                        }
                        *mode = Mode::NewPoint;
                    }
                },
            }
        },

        KeyDown { keycode: Some(Keycode::Escape), ..} => {
            poly.selected.clear();
        },

        KeyDown { keycode: Some(Keycode::A), ..} => {

            for i in 0..poly.polygon.vertices.len() {
                poly.selected.insert(i);
            }
        },


        KeyDown { keycode: Some(Keycode::Z), keymod, ..} => {
            use sdl2::keyboard::Mod;
            if keymod.intersects(Mod::LCTRLMOD) {

                poly.sub_divisions.clear();
                poly.polygon.vertices.pop();
                let len = poly.polygon.vertices.len();

                poly.selected.remove(&len);
            }
        },

        _ => {}

    }
}



// maybe have
//fn handle_object_mode(event: &event::Event, selected_obj: Option<usize>, state: &mut state)
fn handle_object_mode( event: &event::Event, mode: &mut Mode, options: &mut options::Options) {

    use event::Event::*;
    use sdl2::keyboard::Keycode;

    match event {
        MouseButtonUp {x, y, ..} => {
            *mode = Mode::NewPoint;
        },
        KeyDown { keycode: Some(Keycode::C), ..} => {
            options.check_collision = !options.check_collision;
        },
        _ => {}
    }
}

fn check_collision(drawer2D: &mut Drawer2D, polygons: &mut Vec::<Poly>) {
    calc_and_set_subdivision(polygons);

    for i in 0..(polygons.len() - 1) {
        for j in (i+1)..polygons.len() {
            poly_collision(drawer2D, &polygons[i], &polygons[j]);
        }
    }
}


// does not return early
fn poly_collision(drawer2D: &mut Drawer2D, p1: &Poly, p2: &Poly) -> bool {

    let mut res = false;
    for indices_1 in &p1.sub_divisions {
        let sub_p_1 = gjk::ComplexPolygon {
            polygon: &p1.polygon,
            indices: &indices_1
        };

        for indices_2 in &p2.sub_divisions {
            let sub_p_2 = gjk::ComplexPolygon {
                polygon: &p2.polygon,
                indices: &indices_2
            };

            let collision = gjk::gjk_intersection(&sub_p_1, &sub_p_2);
            if collision {
                res = true;
                drawer2D.convex_polygon(&sub_p_1);
                drawer2D.convex_polygon(&sub_p_2);

            }
        }
    }
    res
}

impl<'a> drawer2d::ConvexPolygon for gjk::ComplexPolygon<'a> {
    fn set_vertices(&self, buffer: &mut Vec::<f32>, viewport_height: f32) {
        for &i in self.indices {
            let v = self.polygon.vertices[i];
            buffer.push(v.x);
            buffer.push(viewport_height - v.y);
            buffer.push(0.0);
        }
    }
}

use gl_lib::{gl, na, helpers, color::Color};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use sdl2::event;
use std::collections::HashSet;

type V2 = na::Vector2::<f32>;
type V3 = na::Vector3::<f32>;

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
            vertices: vec![],
            sub_polygons: vec![]
        },
        selected: Default::default(),
        mode: Mode::NewPoint,
        show_help: false
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


        render_poly(&mut ui, &mut state.polygon, &state.selected, state.show_help);

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


fn render_poly(ui: &mut Ui, poly: &mut Polygon, selected: &HashSet::<usize>, show_help: bool) {

    let len = poly.vertices.len();

    for i in 0..len {
        let p1 = &mut poly.vertices[i];

        let mut r = 8.0;

        if selected.contains(&i) {
            r = 10.0;
        }

        if show_help {
            //ui.drawer2D.render_text(&format!("({:?}) - {i}", p1), p1.x as i32, p1.y as i32, 14);
            ui.drawer2D.render_text(&format!("{i}"), p1.x as i32, p1.y as i32, 20);
        }

        let mut drag = na::Vector2::<i32>::new(p1.x as i32, p1.y as i32);
        ui.drag_point(&mut drag, r);
        p1.x = drag.x as f32;
        p1.y = drag.y as f32;

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

    for sub_p in &poly.sub_polygons {
        for idx in &sub_p.indices {
            ui.drawer2D.rounded_rect(poly.vertices[*idx].x, poly.vertices[*idx].y, 10, 10);
        }
    };

    render_intersect(ui, poly);
}

struct State {
    polygon: Polygon,
    selected: HashSet::<usize>,
    mode: Mode,
    show_help: bool
}


pub enum Mode {
    NewPoint,
    Select(V2, V2),
}

pub struct Polygon {
    vertices: Vec::<V2>,
    sub_polygons: Vec::<SubPolygon>
}


pub struct SubPolygon {
    indices: Vec::<usize>
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
                calulate_subdivision(&mut state.polygon);
            },

            KeyDown { keycode: Some(Keycode::Tab), ..} => {
                state.show_help = !state.show_help
            },

            KeyDown { keycode: Some(Keycode::C), ..} => {
                state.selected.clear();
                 state.polygon = Polygon {
                     vertices: vec![],
                     sub_polygons: vec![]
                 };
            },

            KeyDown { keycode: Some(Keycode::A), ..} => {
                for i in 0..state.polygon.vertices.len() {
                    state.selected.insert(i);
                }

            },

            KeyDown { keycode: Some(Keycode::Z), keymod, ..} => {
                use sdl2::keyboard::Mod;
                if keymod.intersects(Mod::LCTRLMOD) {

                    state.polygon.vertices.pop();
                    let len = state.polygon.vertices.len();

                    state.selected.remove(&len);
                }
            },

            _ => {}

        }
    }
}




fn calulate_subdivision(polygon: &mut Polygon) {
    polygon.sub_polygons.clear();

    // first make it ofirst make
    polygon.sub_polygons.push(SubPolygon {
        indices: vec![]
    });


    let dir = direction(&polygon);

    match dir {
        Dir::Left => {
            polygon.vertices.reverse();
        },
        _ => {
        }
    }

    if let Some(wide_idx) = first_wide(polygon) {
        // search from left to right for the first polygon that does not produce any intersections
        // and does not make a line outside the polygon

        // loop over posible connections

        println!("Found: {:?}", find_valid_connection(polygon, wide_idx));
    }
}

fn find_valid_connection(polygon: &Polygon, idx: usize) -> usize {
    let cur_p = polygon.vertices[idx];

    let len = polygon.vertices.len();

    let mut res = 0;
    for conn_idx in 0..len {
        let mut valid = true;

        if conn_idx == (len + idx - 1) % len || conn_idx == idx || conn_idx == (idx + 1 % len) {
            // skip0 neighbours and same index
            continue;
        }
        let connection_point = polygon.vertices[conn_idx];

        // now check all lines in polygon

        'inner: for i in 0..len {
            let idx1 = i;
            let idx2 = (i + 1) % len;
            if idx1 == idx || idx1 == conn_idx
                || idx2 == idx || idx2 == conn_idx {
                    continue;
                }
            let c = polygon.vertices[idx1];
            let d = polygon.vertices[idx2];

            if let Some(_) = lsi::line_segment_intersect(cur_p, connection_point, c,d) {
                // line conn
                valid = false;
                break 'inner;
            }
        }


        if valid {
            let inside = is_inside(idx, conn_idx, polygon);
            res = conn_idx;
            println!("Found {:?}", (idx, conn_idx, inside));
        }
    }

    res
}

fn is_inside(idx1: usize, idx2: usize, polygon: &Polygon) -> bool {

    // just check that the new edge is between the two edges.
    // find incoming edge as normalization
    // new edge is higher angle than basline,
    // new edge is lower angle than outgoing edge

    let len = polygon.vertices.len();
    let from = polygon.vertices[(len + idx1 - 1) % len];
    let p = polygon.vertices[idx1];
    let to = polygon.vertices[(idx1 + 1) % len];

    let mut from_dir = from - p;
    from_dir.y *= -1.0; // sdl inverse coords, neeed to be fixed when using atan2
    let mut to_dir = to - p;
    to_dir.y *= -1.0;

    // between -pi and pi has to be in 0 to tau
    let v1 = from_dir.y.atan2(from_dir.x);
    let mut v2 = to_dir.y.atan2(to_dir.x);
    if v2 < 0.0 {
        v2 += std::f32::consts::TAU;
    }
    v2 -= v1;

    let mut new_dir = polygon.vertices[idx2] - polygon.vertices[idx1];
    new_dir.y *= -1.0;

    let mut new_v = (new_dir.y).atan2(new_dir.x);
    if new_v < 0.0 {
        new_v += std::f32::consts::TAU;
    }

    new_v -= v1;



    return new_v < v2 && new_v > 0.0;
}


fn first_wide(polygon: &Polygon) -> Option<usize> {
    let len = polygon.vertices.len();

    for i in 0..polygon.vertices.len() {
        let before = polygon.vertices[(len + i - 1) % len];
        let pi = polygon.vertices[i];
        let after = polygon.vertices[(i + 1) % len];

        if is_wide_angle(vec3(before), vec3(pi), vec3(after)) {
            return Some(i);
        }
    }

    None
}

fn wide_indices(polygon: &Polygon) -> Vec::<usize> {
    let mut res = vec![];

    let len = polygon.vertices.len();

    for i in 0..polygon.vertices.len() {
        let before = polygon.vertices[(len + i - 1) % len];
        let pi = polygon.vertices[i];
        let after = polygon.vertices[(i + 1) % len];

        if is_wide_angle(vec3(before), vec3(pi), vec3(after)) {
            res.push(i);
        }
    }

    res

}

fn calulate_subdivision_2(polygon: &mut Polygon) {

    let len = polygon.vertices.len();
    let mut min_dot = 1.0;
    let mut min_idx = 0;
    println!("\n\n\n");
    for i in 0..polygon.vertices.len() {
        let before = polygon.vertices[(len + i - 1) % len];
        let pi = polygon.vertices[i];
        let after = polygon.vertices[(i + 1) % len];



        // Find v1 that is how much to rotation dir1 to align with x axis

        let mut dir1 = (before - pi).normalize();
        // invert the y, sdl has 0 at top and max at bottom, atan2 expect the reverse for y
        dir1.y *= -1.0;

        let v1 = dir1.y.atan2(dir1.x);

        let mut dir2 = (after - pi).normalize();
        // invert the y, sdl has 0 at top and max at bottom, atan2 expect the reverse for y
        dir2.y *= -1.0;

        let mut v2 = dir2.y.atan2(dir2.x);
        if v2 < 0.0 {
            v2 += std::f32::consts::TAU;
        }

        // v1 is negative angle, that is how much to rotate to hav that align with x axis, add (subtract) from v2
        let mut v = v2 -v1;
        if v > std::f32::consts::TAU {
            v -= std::f32::consts::TAU;
        }
        println!("{:?}", v);
        let d = dir1.dot(&dir2);
        if d <= 0.0 {
            //polygon.sub_polygons[0].indices.push(i);
        }

        if is_wide_angle(vec3(before), vec3(pi), vec3(after)) {
            polygon.sub_polygons[0].indices.push(i);
        }

    }

}



fn direction(polygon: &Polygon) -> Dir {

    let mut num_wide = 0;

    // assume right, and if not return left

    for i in 1..polygon.vertices.len() {
        let v1_i = (i + 1) % polygon.vertices.len();
        let v2_i = (i + 2) % polygon.vertices.len();

        let v0 = vec3(polygon.vertices[i]);
        let v1 = vec3(polygon.vertices[v1_i]);
        let v2 = vec3(polygon.vertices[v2_i]);

        if is_wide_angle(v0, v1, v2) {
            num_wide += 1;
        }
    }


    if num_wide > (polygon.vertices.len()  / 2 ) {
        return Dir::Left;
    }

    return Dir::Right;
}

fn vec3(v: V2) -> V3{
    V3::new(v.x, v.y, 0.0)
}

// The triangles are always right handed. So when the cross product is below 0 between the two edges the angle is > 180 deg
fn is_wide_angle(v0: na::Vector3::<f32>, v1: na::Vector3::<f32>, v2: na::Vector3::<f32>) -> bool {
    (v1 - v0).cross(&(v2-v1)).z < 0.0
}


enum Dir {
    Left,
    Right
}





fn tmp() {

    /*
    polygon_shader.shader.set_used();


    let transform = na::Matrix4::<f32>::identity();

    polygon_shader.shader.set_mat4(gl, "transform", transform);

    polygon.render(gl);
     */
}

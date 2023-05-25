use gl_lib::{gl, na, helpers, objects};
use gl_lib::shader::Shader;
use gl_lib::color::Color;
use gl_lib::particle_system::*;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use deltatime;
use gl_lib::text_rendering::font::{Font, MsdfFont, FntFont};
use gl_lib::shader::BaseShader;
use sdl2::event;
use rand::Rng;
use gl_lib::particle_system::particle::Particle;
use std::collections::HashSet;

type V2 = na::Vector2::<i32>;

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


    let mut vertices: Vec<f32> = vec![
        // positions
        0.5, -0.5, 0.0,
        0.5,  0.5, 0.0,
        -0.5,  0.5, 0.0,
        -0.5, -0.5, 0.0,
    ];

    let mut indices: Vec<u32> = vec![
        0,1,3,
        1,2,3];

    let polygon = objects::polygon::Polygon::new(gl, &indices, &vertices, None);
    let polygon_shader = gl_lib::shader::PosColorShader::new(gl).unwrap();

    let mut state = State {
        polygons: vec![
            Polygon {
                vertices: vec![]
            }
        ],
        selected: Default::default(),
        mode: Mode::NewPoint,

    };

    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui, &mut state);

        render_poly(&mut ui, &mut state.polygons[0], &state.selected);

        render_mode(&mut ui, &state.mode);


        window.gl_swap_window();
    }
}

fn render_mode(ui: &mut Ui, mode: &Mode) {

    match mode {
        Mode::Select(p1, p2) => {
            let tl = na::Vector2::new(i32::min(p1.x, p2.x), i32::max(p1.y, p2.y));
            let br = na::Vector2::new(i32::max(p1.x, p2.x), i32::min(p1.y, p2.y));

            ui.drawer2D.rounded_rect(tl.x, tl.y, br.x - tl.x, br.y - tl.y);
        },
        _ => {}
    }
}


fn render_poly(ui: &mut Ui, poly: &mut Polygon, selected: &HashSet::<usize>) {

    let len = poly.vertices.len();

    for i in 0..len {
        let p1 = &mut poly.vertices[i];

        let mut r = 5.0;

        if selected.contains(&i) {
            r = 10.0;
        }
        ui.drag_point(p1, r);

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

}

struct State {
    polygons: Vec::<Polygon>,
    selected: HashSet::<usize>,
    mode: Mode
}


enum Mode {
    NewPoint,
    Select(V2, V2),
    Selection
}



struct Polygon {
    vertices: Vec::<V2>,
}



fn handle_inputs(ui: &mut Ui, state: &mut State) {

    use event::Event::*;

    for e in ui.get_frame_inputs() {
        match e {
            MouseButtonDown {x, y, ..} => {
                state.mode = Mode::Select(V2::new(*x, *y), V2::new(*x, *y));
            },

            MouseButtonUp {x, y, ..} => {
                let new = V2::new(*x, *y);
                match state.mode {
                    Mode::NewPoint => {
                        state.polygons[0].vertices.push(new);
                    },
                    Mode::Select(start, _) => {
                        let diff = new - start;

                        if diff.x.abs() < 2 && diff.y.abs() < 2 {
                            state.polygons[0].vertices.push(new);
                            state.mode = Mode::NewPoint;
                        } else {
                            let tl = na::Vector2::new(i32::min(new.x, start.x), i32::max(new.y, start.y));
                            let br = na::Vector2::new(i32::max(new.x, start.x), i32::min(new.y, start.y));

                            for i in 0..state.polygons[0].vertices.len() {
                                let p = &state.polygons[0].vertices[i];
                                if p.x >= tl.x && p.x <= br.x && p.y <= tl.x && p.y >= br.y {
                                    state.selected.insert(i);
                                }
                            }

                            if state.selected.len() > 0 {
                                state.mode = Mode::Selection;
                            } else {
                                state.mode = Mode::NewPoint;
                            }
                        }
                    },
                    Mode::Selection => {
                        state.selected.clear();

                        state.mode = Mode::NewPoint;

                    }
                }
            },
            MouseMotion { x, y, mousestate, ..} => {
                let new = V2::new(*x, *y);
                match state.mode {
                    Mode::Select(start, _) => {
                        state.mode = Mode::Select(start, new);
                    },
                    _ => {}
                }
            },
            _ => {}

        }
    }
}





fn tmp() {

    /*
    polygon_shader.shader.set_used();


    let transform = na::Matrix4::<f32>::identity();

    polygon_shader.shader.set_mat4(gl, "transform", transform);

    polygon.render(gl);
     */
}

use failure;
use gl_lib::color::Color;
use gl_lib::na::vector;
use gl_lib::sdl_gui as gls;
use gl_lib::shader::{ColorShader, TransformationShader};
use gl_lib::{gl, na};

mod triangulate;
use triangulate::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Triangulate,
    Step,
    Clear,
    Render,
}

fn main() -> Result<(), failure::Error> {
    let width = 1000;
    let height = 800;

    let mut window = gls::window::SdlGlWindow::new("Polygon triangulation", width, height).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let gl = &window.gl().clone();

    let mut model = Model {
        gl: gl.clone(),
        poly: vec![],
        triangulation: None,
        filled_poly: None,
    };

    let point_square = gl_lib::objects::square::Square::new(gl);
    let mut shader = gl_lib::shader::PosShader::new(gl).unwrap();

    let mut polygon_shader = gl_lib::shader::PosColorShader::new(gl).unwrap();

    let line_drawer = LineDrawer {
        gl,
        shader: &shader,
        width: width as f32,
        height: height as f32,
        square: &point_square,
    };

    while !window.should_quit() {
        shader.shader.set_used();

        shader.set_color(Color::Rgb(52, 235, 225));
        for p in &model.poly {
            let transform = transform_2d(0.05, *p, width as f32, height as f32);
            shader.shader.set_mat4(gl, "transform", transform);

            point_square.render(gl);
        }

        if model.poly.len() > 1 {
            shader.set_color(Color::Rgb(0, 0, 0));
            let mut cur = 0;
            let mut next = 1;

            for i in 0..model.poly.len() {
                let next = (i + 1) % model.poly.len();

                let p0 = model.poly[i];
                let p1 = model.poly[next];

                line_drawer.draw_line(p0, p1);
            }
        }

        if let Some(ref triang) = model.triangulation {
            for tri in &triang.triangles {
                let p0 = triang.polygon[tri.p0];
                let p1 = triang.polygon[tri.p1];
                let p2 = triang.polygon[tri.p2];

                line_drawer.draw_line(p0, p1);
                line_drawer.draw_line(p1, p2);
                line_drawer.draw_line(p2, p0);
            }
        }

        if let Some(ref filled) = model.filled_poly {
            let trans = na::Matrix4::identity();

            polygon_shader.set_transform(trans);

            filled.render(gl);
        }

        window.update(&mut model);
    }

    Ok(())
}

struct LineDrawer<'a, TShader>
where
    TShader: gl_lib::shader::TransformationShader,
{
    gl: &'a gl::Gl,
    width: f32,
    height: f32,
    shader: &'a TShader,
    square: &'a gl_lib::objects::square::Square,
}

impl<'a, TShader> LineDrawer<'a, TShader>
where
    TShader: gl_lib::shader::TransformationShader,
{
    fn draw_line(&self, from: Point, to: Point) {
        let line_transform = gl_lib::objects::square::Square::line_2d_transform(
            from,
            to,
            self.width,
            self.height,
            0.01,
        );

        self.shader.set_transform(line_transform);
        self.square.render(&self.gl);
    }
}

fn transform_2d(scale: f32, location: Point, window_w: f32, window_h: f32) -> na::Matrix4<f32> {
    let s = na::geometry::Scale3::new(scale, scale, scale);

    let x = (location.x / window_w) * 2.0 - 1.0;
    let y = (location.y / window_h) * 2.0 - 1.0;

    let t = na::geometry::Translation3::new(x, y, 0.0);

    t.to_homogeneous() * s.to_homogeneous()
}

struct Model {
    poly: Polygon,
    gl: gl::Gl,
    triangulation: Option<Triangulation>,
    filled_poly: Option<gl_lib::objects::polygon::Polygon>,
}

impl gls::Ui<Message> for Model {
    fn handle_message(&mut self, msg: &Message, _: &gls::window::WindowComponentAccess) {
        let width = 1000.0;
        let height = 800.0;
        match msg {
            Message::Triangulate => self.triangulation = Some(triangulate_ear_clipping(&self.poly)),
            Message::Clear => {
                self.triangulation = None;
                self.filled_poly = None;
                self.poly = vec![];
            }
            Message::Render => {
                if let Some(ref triang) = self.triangulation {
                    self.filled_poly = Some(create_filled_poly_indiv_triangles(&self.gl, triang, width, height));
                }
            }
            _ => {}
        }
    }

    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;

        let mut row = Row::new()
            .spacing(5)
            .add(Button::new("Triangulate", Some(Message::Triangulate)))
            .add(Button::new("Clear", Some(Message::Clear)))
            .add(Button::new("Render", Some(Message::Render)));

        row.into()
    }

    fn handle_sdl_event(&mut self, event: sdl2::event::Event) {
        use sdl2::event::Event;
        match event {
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.poly.push(vector!(x as f32, 800.0 - y as f32));
                    println!("{:?}", (x, y));
                }
            }
            _ => {}
        }
    }
}

fn create_filled_poly_packed(gl: &gl::Gl, triang: &Triangulation, width: f32, height: f32) -> gl_lib::objects::polygon::Polygon {
    let mut indices = vec![];
    let mut vertices = vec![];
    let mut colors = vec![];

    let all_colors = [
        Color::Rgb(255, 0, 0),
        Color::Rgb(0, 255, 0),
        Color::Rgb(0, 0, 255),
    ];

    let mut i = 0;
    for point in &triang.polygon {
        vertices.push(point.x / width * 2.0 - 1.0);
        vertices.push(point.y / height * 2.0 - 1.0);
        vertices.push(0.0);

        colors.push(all_colors[i % all_colors.len()]);
        i += 1;
    }

    for tri in &triang.triangles {
        indices.push(tri.p0 as u32);
        indices.push(tri.p1 as u32);
        indices.push(tri.p2 as u32);
    }

    println!("{:?}", colors);

    gl_lib::objects::polygon::Polygon::new(
        gl,
        &indices,
        &vertices,
        Some(&colors),
    )
}


fn create_filled_poly_indiv_triangles(gl: &gl::Gl, triang: &Triangulation, width: f32, height: f32) -> gl_lib::objects::polygon::Polygon {
    let mut indices = vec![];
    let mut vertices = vec![];
    let mut colors = vec![];

    let all_colors = [
        Color::Rgb(255, 0, 0),
        Color::Rgb(0, 255, 0),
        Color::Rgb(0, 0, 255),
    ];


    let mut idx = 0;
    let mut col_idx = 0;
    for tri in &triang.triangles {

        let col = all_colors[col_idx % all_colors.len()];

        col_idx += 1;
        indices.push(idx);
        indices.push(idx + 1);
        indices.push(idx + 2);
        idx += 3;

        let v0 = triang.polygon[tri.p0];
        vertices.push(v0.x / width * 2.0 - 1.0);
        vertices.push(v0.y / height * 2.0 - 1.0);
        vertices.push(0.0);
        colors.push(col);


        let v1 = triang.polygon[tri.p1];
        vertices.push(v1.x / width * 2.0 - 1.0);
        vertices.push(v1.y / height * 2.0 - 1.0);
        vertices.push(0.0);
        colors.push(col);


        let v2 = triang.polygon[tri.p2];
        vertices.push(v2.x / width * 2.0 - 1.0);
        vertices.push(v2.y / height * 2.0 - 1.0);
        vertices.push(0.0);
        colors.push(col);

    }

    println!("{:?}", colors);


    gl_lib::objects::polygon::Polygon::new(
        gl,
        &indices,
        &vertices,
        Some(&colors),
    )
}

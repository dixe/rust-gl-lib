use failure;
use gl_lib::color::Color;
use gl_lib::na::vector;
use gl_lib::sdl_gui as gls;
use gl_lib::shader::{ColorShader, TransformationShader};
use gl_lib::{gl, na};
use ttf_parser;
use rand::Rng;

mod triangulate;
use triangulate::*;

mod ttf;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Triangulate,
    Clear,
    Fill,
    Switch,
    RandomChar,
}

fn main() -> Result<(), failure::Error> {
    let width = 1000.0;
    let height = 800.0;

    let data = include_bytes!("../../assets/fonts/calibri.ttf");

    let face = ttf_parser::Face::from_slice(data.as_slice(), 0).unwrap();

    let polygons = ttf::char_to_poly('8', &face, width, height, 1);

    let mut window =
        gls::window::SdlGlWindow::new("Polygon triangulation", width as u32, height as u32)
            .unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let gl = &window.gl().clone();

    let mut model = Model {
        gl: gl.clone(),
        mode: Mode::Glyph(GlyphModel {
            polys: polygons,
            triangulations: vec![],
            filled_polys: vec![],
        }),
        triangulation: None,
        face
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

    let render_info = RenderInfo {
        gl,
        shader: &shader,
        point_square: &point_square,
        polygon_shader: &polygon_shader,
        line_drawer: &line_drawer,
        height,
        width,
    };

    while !window.should_quit() {
        model.mode.render(&render_info);

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

struct Model<'a> {
    mode: Mode,
    gl: gl::Gl,
    triangulation: Option<Triangulation>,
    face: ttf_parser::Face<'a>,
}

enum Mode {
    Draw(DrawModel),
    Glyph(GlyphModel),
}

struct DrawModel {
    poly: Polygon,
    triangulation: Option<Triangulation>,
    filled_poly: Option<gl_lib::objects::polygon::Polygon>,
}

struct GlyphModel {
    polys: Vec<Polygon>,
    triangulations: Vec<Triangulation>,
    filled_polys: Vec<gl_lib::objects::polygon::Polygon>,
}

impl Mode {
    fn clear(&mut self) {
        match self {
            Mode::Draw(ref mut d) => {
                d.poly.clear();
                d.triangulation = None;
                d.filled_poly = None;
            }
            Mode::Glyph(ref mut dm) => {
                dm.polys.clear();
                dm.triangulations.clear();
                dm.filled_polys.clear();
            }
        }
    }

    fn render(&self, ri: &RenderInfo) {
        match self {
            Mode::Draw(ref d) => {
                render_draw(d, ri);
            }
            Mode::Glyph(ref dm) => {
                render_glyph(dm, ri);
            }
        }
    }

    fn fill(&mut self, gl: &gl::Gl, width: f32, height: f32) {
        match self {
            Mode::Draw(ref mut draw) => {
                if let Some(ref triang) = draw.triangulation {
                    draw.filled_poly = Some(create_filled_poly_packed(
                        gl,
                        triang,
                        width,
                        height,
                        &[Color::Rgb(255, 0, 0)],
                    ));
                }
            }
            Mode::Glyph(ref mut model) => {
                model.filled_polys.clear();
                for (i, triang) in model.triangulations.iter().enumerate() {
                    let color = if triang.dir == Direction::Right {
                        Color::Rgb(0, 0, 0)
                    } else {
                        Color::Rgb(255, 255, 255)
                    };
                    model.filled_polys.push(create_filled_poly_packed(
                        gl,
                        triang,
                        width,
                        height,
                        &[color],
                    ));
                }
            }
        }
    }
}

struct RenderInfo<'a> {
    gl: &'a gl::Gl,
    shader: &'a gl_lib::shader::PosShader,
    point_square: &'a gl_lib::objects::square::Square,
    polygon_shader: &'a gl_lib::shader::PosColorShader,
    line_drawer: &'a LineDrawer<'a, gl_lib::shader::PosShader>,
    height: f32,
    width: f32,
}

fn render_glyph(model: &GlyphModel, render_info: &RenderInfo) {
    render_info.shader.shader.set_used();

    for poly in &model.polys {
        render_poly(poly, render_info);
    }

    for triang in &model.triangulations {
        render_triangulation(triang, render_info);
    }

    for filled in &model.filled_polys {
        render_filled(filled, render_info);
    }
}

fn render_poly(poly: &Polygon, render_info: &RenderInfo) {
    let gl = render_info.gl;
    render_info.shader.shader.set_used();

    // points color
    render_info.shader.set_color(Color::Rgb(52, 235, 225));
    for point in poly {
        let transform = transform_2d(0.05, *point, render_info.width, render_info.height);
        render_info
            .shader
            .shader
            .set_mat4(gl, "transform", transform);

        render_info.point_square.render(gl);
    }

    if poly.len() > 1 {
        render_info.shader.set_color(Color::Rgb(0, 0, 0));
        let mut cur = 0;
        let mut next = 1;

        for i in 0..poly.len() {
            let next = (i + 1) % poly.len();

            let p0 = poly[i];
            let p1 = poly[next];

            render_info.line_drawer.draw_line(p0, p1);
        }
    }
}

fn render_filled(filled: &gl_lib::objects::polygon::Polygon, render_info: &RenderInfo) {
    let trans = na::Matrix4::identity();

    render_info.polygon_shader.set_transform(trans);

    filled.render(&render_info.gl);
}

fn render_triangulation(triang: &Triangulation, render_info: &RenderInfo) {
    for tri in &triang.triangles {
        let p0 = triang.polygon[tri.p0];
        let p1 = triang.polygon[tri.p1];
        let p2 = triang.polygon[tri.p2];

        render_info.line_drawer.draw_line(p0, p1);
        render_info.line_drawer.draw_line(p1, p2);
        render_info.line_drawer.draw_line(p2, p0);
    }
}

fn render_draw(model: &DrawModel, render_info: &RenderInfo) {
    let gl = render_info.gl;

    render_info.shader.set_color(Color::Rgb(52, 235, 225));

    render_poly(&model.poly, render_info);

    if let Some(ref triang) = model.triangulation {
        render_triangulation(triang, render_info);
    }

    if let Some(ref filled) = model.filled_poly {
        render_filled(filled, render_info);
    }
}

impl gls::Ui<Message> for Model<'_> {
    fn handle_message(&mut self, msg: &Message, _: &gls::window::WindowComponentAccess) {
        let width = 1000.0;
        let height = 800.0;
        match msg {
            Message::Triangulate => match self.mode {
                Mode::Draw(ref mut draw) => {
                    draw.triangulation = Some(triangulate_ear_clipping(&draw.poly));
                }
                Mode::Glyph(ref mut model) => {
                    model.triangulations.clear();
                    for poly in &model.polys {
                        model.triangulations.push(triangulate_ear_clipping(poly));
                    }
                }
            },
            Message::Clear => {
                self.mode.clear();
            }
            Message::Fill => {
                self.mode.fill(&self.gl, width, height);
            }
            Message::Switch => match self.mode {
                Mode::Draw(_) => {
                    self.mode = Mode::Glyph(GlyphModel {
                        polys: vec![],
                        triangulations: vec![],
                        filled_polys: vec![],
                    })
                }
                Mode::Glyph(_) => {
                    self.mode = Mode::Draw(DrawModel {
                        poly: vec![],
                        filled_poly: None,
                        triangulation: None,
                    });
                }
            },
            Message::RandomChar => match self.mode {
                Mode::Glyph(ref mut mode) => {
                    let charset: Vec::<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØa\
                            abcdefghijklmnopqrstuvwxyzæøå\
                            0123456789)(*&^%$#@!~".chars().collect();

                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..charset.len());
                    let polys = ttf::char_to_poly(charset[idx], &self.face, width, height, 1);
                    mode.polys = polys;
                    mode.triangulations.clear();
                    mode.filled_polys.clear();
                }
                _ => {}
            },
        }
    }

    fn view(&self) -> gls::layout::Node<Message> {
        use crate::gls::layout::Length::*;
        use gls::layout::*;

        let has_tri;
        let mut row = Row::new()
            .spacing(5.0)
            .add(Button::new("Triangulate", Some(Message::Triangulate)));
        match self.mode {
            Mode::Glyph(ref mode) => {
                row = row.add(Button::new("Rand Char", Some(Message::RandomChar)));
                has_tri = mode.triangulations.len() > 0;
            },
            Mode::Draw(ref mode) => {
                row = row.add(Button::new("Clear", Some(Message::Clear)));
                has_tri = mode.triangulation != None;
            }
        }

        row = row.add(Button::new("Fill", Some(Message::Fill))
                      .width(Px(80))
                      .disabled(!has_tri))
            .add(Button::new("Switch", Some(Message::Switch))
                 .align_right())
            .add(TextBox::new(None));

        row.into()
    }

    fn handle_sdl_event(&mut self, event: sdl2::event::Event) {
        use sdl2::event::Event;
        match self.mode {
            Mode::Draw(ref mut draw) => match event {
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        draw.poly.push(vector!(x as f32, 800.0 - y as f32));
                    }
                }
                _ => {}
            },
            _ => {}
        };
    }
}

fn create_filled_poly_packed(
    gl: &gl::Gl,
    triang: &Triangulation,
    width: f32,
    height: f32,
    all_colors: &[Color],
) -> gl_lib::objects::polygon::Polygon {
    let mut indices = vec![];
    let mut vertices = vec![];
    let mut colors = vec![];

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

    gl_lib::objects::polygon::Polygon::new(gl, &indices, &vertices, Some(&colors))
}

fn create_filled_poly_indiv_triangles(
    gl: &gl::Gl,
    triang: &Triangulation,
    width: f32,
    height: f32,
) -> gl_lib::objects::polygon::Polygon {
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

    gl_lib::objects::polygon::Polygon::new(gl, &indices, &vertices, Some(&colors))
}

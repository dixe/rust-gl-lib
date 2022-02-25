use gl_lib::na::vector;
use gl_lib::shader::{TransformationShader, ColorShader, Color};
use failure;
use gl_lib::sdl_gui as gls;
use gl_lib::{na, gl};

mod triangulate;
use triangulate::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Triangulate,
    Step,
    Clear
}


fn main() -> Result<(), failure::Error> {

    let width = 1000;
    let height = 800;

    let mut window = gls::window::SdlGlWindow::new("Polygon triangulation", width, height).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();


    let gl = &window.gl().clone();

    let mut model = Model { poly: vec![], triangulation: None };

    let point_square = gl_lib::objects::square::Square::new(gl);
    let mut shader = gl_lib::shader::BasicShader::new(gl).unwrap();

    let line_drawer = LineDrawer { gl, shader: &shader, width: width as f32, height: height as f32, square: &point_square};

    while !window.should_quit() {

        shader.shader.set_used();

        shader.set_color(Color::Rgb(52, 235, 225));
        for p in &model.poly {
            let transform = transform_2d(0.05, *p, width as f32, height as f32);

            shader.set_transform(transform);
            shader.shader.set_mat4(gl,"transform", transform);

            point_square.render(gl);
        }


        if model.poly.len() > 1 {
            shader.set_color(Color::Rgb(0, 0, 0));
            let mut cur = 0;
            let mut next = 1;

            for i in 0..model.poly.len() {

                let next = (i + 1 ) % model.poly.len();

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

        window.update(&mut model);

    }


    Ok(())
}

struct LineDrawer<'a, TShader>
where TShader : gl_lib::shader::TransformationShader {
    gl: &'a gl::Gl,
    width: f32,
    height: f32,
    shader: &'a TShader,
    square: &'a gl_lib::objects::square::Square
}

impl<'a, TShader> LineDrawer<'a, TShader> where TShader : gl_lib::shader::TransformationShader {

    fn draw_line(&self, from: Point, to: Point) {
        let line_transform = gl_lib::objects::square::Square::line_2d_transform(from, to, self.width, self.height, 0.01);
        self.shader.set_transform(line_transform);
        self.square.render(&self.gl);
    }
}


fn transform_2d(scale: f32, location: Point, window_w: f32, window_h: f32) -> na::Matrix4::<f32> {

    let s = na::geometry::Scale3::new(scale, scale, scale);


    let x = (location.x / window_w) * 2.0 - 1.0;
    let y = ((window_h - location.y) / window_h) * 2.0 - 1.0;

    let t = na::geometry::Translation3::new(x, y, 0.0);


    t.to_homogeneous() * s.to_homogeneous()

}

#[derive(Debug, Clone)]
struct Model {
    poly: Polygon,
    triangulation: Option<Triangulation>
}

impl gls::Ui<Message> for Model {

    fn handle_message(&mut self, msg: &Message, _: &gls::window::WindowComponentAccess) {

        match msg {
            Message::Triangulate => {
                self.triangulation = Some(triangulate_ear_clipping(&self.poly))
            },
            Message::Clear => {
                self.triangulation = None;
                self.poly = vec![];
            }
            _ => {}
        }
    }


    fn view(&self) ->gls::layout::Node<Message> {

        use gls::layout::*;


        let mut row = Row::new()
            .spacing(5)
            .add(Button::new("Triangulate", Some(Message::Triangulate)))
            .add(Button::new("Clear", Some(Message::Clear)));


        row.into()

    }

    fn handle_sdl_event(&mut self, event: sdl2::event::Event) {

        use sdl2::event::Event as Event;
        match event {
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.poly.push(vector!(x as f32, y as f32));
                    println!("{:?}", (x, y));
                }
            },
            _ => {},
        }

    }
}

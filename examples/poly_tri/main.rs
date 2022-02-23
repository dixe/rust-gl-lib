use gl_lib::na::vector;
use failure;
use gl_lib::sdl_gui as gls;
use gl_lib::na;

mod triangulate;
use triangulate::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Triangulate,
    Step
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
    let mut point_shader = gl_lib::objects::square::Square::default_shader(gl).unwrap();


    while !window.should_quit() {

        point_shader.set_used();
        for p in &model.poly {
            let transform = transform_2d(0.05, *p, width as f32, height as f32);

            point_shader.set_mat4(gl, "transform", transform);

            point_square.render(gl);
        }

        if let Some(ref tri) = model.triangulation {
            //println!("Triangle: {:?}", tri);
        }

        window.update(&mut model);

    }


    Ok(())
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
            _ => {}
        }
    }


    fn view(&self) ->gls::layout::Node<Message> {

        use gls::layout::*;


        let mut col = Column::new()
            .add(Button::new("Triangulate", Some(Message::Triangulate)));


        col.into()

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

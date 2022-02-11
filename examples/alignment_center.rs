use gl_lib::na;
use gl_lib::sdl_gui as gls;
use failure;



#[derive(Debug, Clone)]
pub enum Message {
    Add,
    Sub,
    Clear
}

fn main() -> Result<(), failure::Error> {
    let width = 1000;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("Button", width, height).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut world = World { total: 0 };

    while !window.should_quit() {

        window.update(&mut world);
    }

    Ok(())
}



struct World {
    pub total: i32
}

impl gls::State<Message> for World {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Add => { self.total += 1; },
            Message::Sub => { self.total -= 1; },
            Message::Clear => { self.total = 0; },
        }
    }


    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;

        use Length::*;
        let btn_width = 20;
        let spacing = 10;

        let col = Column::new()
            .width(Fill)
            .height(Fill)
            .padding(spacing as f32)
            .spacing(spacing as f32)
            .add(Row::new()
                 .width(Fill)
                 .add_attribute(Attribute::Spacing(10.0))
                 .add(Button::new( "Center", Some(Message::Add))
                      .width(Px(btn_width as u32))
                      .width(Fill)
                      .align_center()
                      .height(Fill)));


        col.into()
    }
}

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

impl gls::Ui<Message> for World {

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

        let col = Column::new().width(Fill)
            .padding(10.0)
            .spacing(10.0)
            .height(Fill)
            .add(Row::new()
                 .width(Fill)
                 .add_attribute(Attribute::Spacing(10.0))
                 .add(Button::new( "Add", Some(Message::Add))
                      .width(Fill)
                      .height(Px(50)))
                 .add(Button::new( "Sub", Some(Message::Sub))
                      .width(Fill)
                      .height(Px(50))))
            .add(Button::new( "Clear", Some(Message::Clear))
                 .width(Fill)
                 .height(Px(50)))
            .add(Button::new( &format!("Total = {}", self.total), {
                if self.total < 3 {
                    None
                }
                else {
                    Some(Message::Clear)
                }})
                 .width(Fill)
                 .height(Fill));


        col.into()
    }
}

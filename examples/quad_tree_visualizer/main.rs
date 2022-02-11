use gl_lib::*;
use gl_lib::sdl_gui as gls;

use failure;
use rand::prelude::*;

mod state;

fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("Fps", width, height).unwrap();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut ui = Ui::default();

    let state = state::State::new(window.gl());

    while !window.should_quit() {
        state.render();
        window.update(&mut ui);
    }

    Ok(())
}


#[derive(Debug, Clone)]
pub enum Message {
    Random
}

#[derive(Default, Clone)]
struct Ui {
    next: Option<na::Vector2::<i32>>
}

impl gls::State<Message> for Ui {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Random => {

                let mut rng = rand::thread_rng();
                self.next = Some(na::Vector2::new(rng.gen_range(0..100), rng.gen_range(0..100)));
                println!("{:?}",self.next);
            },
        };
    }

    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;
        use Length::*;

        Column::new()
            .add(Row::new()
                 .width(Fill)
                 .height(FitContent)
                 .padding(5.)
                 .height(FitContent)
                 .add(Button::new("Random", Some(Message::Random))
                      .width(Px(120))
                 )
            ).into()

    }
}

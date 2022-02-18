use gl_lib::na;
use gl_lib::sdl_gui as gls;
use failure;


#[derive(Debug, Clone)]
pub enum Message {
    Toogle,
}

fn main() -> Result<(), failure::Error> {
    let width = 800;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("Button", width, height).unwrap();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut state = World::default();

    while !window.should_quit() {

        let time_ms =  1.0 / window.deltatime();
        window.render_text(&format!("Fps = {}", time_ms));
        window.update(&mut state);

    }

    Ok(())
}



#[derive(Default)]
struct World {
    v_sync_on: bool
}

impl gls::Ui<Message> for World {

    fn handle_message(&mut self, message: &Message, window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Toogle => {
                self.v_sync_on = !self.v_sync_on;
                match self.v_sync_on {
                    false => {
                        window_access.set_swap_interval(0);
                    },
                    true => {
                        window_access.enable_vsync();

                    }
                }
            }
        }
    }


    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;

        use Length::*;

        Button::new( &format!("Toogle") ,Some(Message::Toogle))
            .width(Px(100))
            .height(Px(60))
            .into()
    }
}

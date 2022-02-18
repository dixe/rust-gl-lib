use failure;
/// Example where gl_lib_sdl is only used for UI elements
use gl_lib::na;
use gl_lib::sdl_gui as gls;

mod state;

#[derive(Debug, Clone)]
pub enum Message {
    Log,
}

fn main() -> Result<(), failure::Error> {
    let width = 600;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("BouncyBall", width, height).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));
    window.setup_blend();

    let gl = window.gl();
    let mut state = state::State::new(gl);

    while !window.should_quit() {

        state.render();

        // handle ui and event for state and swap gl
        window.update(&mut state);
    }

    Ok(())
}

impl gls::Ui<Message> for state::State {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {
        match message {
            Message::Log => {
                self.set_color(na::Vector3::new(0.0, 0.0, 0.0));
            }
        };
    }

    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;
        let ui = gls::layout::Column::new().add(Button::new("Reset to black using ui", Some(Message::Log)));

        ui.into()
    }


    fn handle_events(&mut self, event: sdl2::event::Event) {
        self.handle_events(event);
    }
}

/// Example where gl_lib_sdl is only used for UI elements
/// All the rendering physics and input for bouncy_ball is handled outside of gl_lib_sdl.
/// The only interaction needed is the we need to get the sdl event from the window
use gl_lib::na;
use gl_lib::sdl_gui as gls;
use failure;

mod state;


#[derive(Debug, Clone)]
pub enum Message {

}

fn main() -> Result<(), failure::Error> {
    let width = 600;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("BouncyBall", width, height).unwrap();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut ui = Ui::default();
    let gl = window.gl();
    let mut state = state::State::new(gl);

    while !window.should_quit() {
        state.render();

        // handle ui and event for state and swap gl
        window.update_with_handler(&mut ui, |event| state.handle_events(event));
    }

    Ok(())
}


#[derive(Debug, Clone)]
struct Ui {

}

impl Default for Ui {
    fn default() -> Self {
        Self {
        }
    }
}




impl gls::State<Message> for Ui {

    fn handle_message(&mut self, _message: &Message, _window_access: &gls::window::WindowComponentAccess) {

    }



    fn view(&self) -> gls::layout::Node<Message> {

        /*use gls::layout::row::*;
        use gls::layout::column::*;
        use gls::layout::element::*;
        use gls::layout::attributes::*;
        use gls::layout::button::*;

        use Length::*;
         */
        let col = gls::layout::Column::new();
        col.into()
    }
}

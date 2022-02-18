use gl_lib::na;
use gl_lib::sdl_gui as gls;
use failure;

fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("Fps", width, height).unwrap();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut state = State {};

    loop {

        let time_ms =  1.0 / window.deltatime();
        window.render_text(&format!("Fps = {}", time_ms));

        window.update(&mut state);
    }
}


#[derive(Debug, Clone)]
pub enum Message {
}


struct State {

}

impl gls::Ui<Message> for State {

    fn handle_message(&mut self, _message: &Message, _window_access: &gls::window::WindowComponentAccess) {
    }

    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;


        Column::new().into()

    }
}

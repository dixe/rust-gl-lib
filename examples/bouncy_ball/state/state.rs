use gl_lib::{gl, objects::*};


pub struct State {
    square: square::Square,
    gl: gl::Gl
}


impl State {

    pub fn new(gl: &gl::Gl) -> Self {
        Self {
            square: square::Square::new(gl),
            gl: gl.clone()
        }
    }

    pub fn render(&self) {
        self.square.render(&self.gl);
    }


    pub fn handle_events(&mut self, event: sdl2::event::Event) {
        println!("{:?}", event);
    }

}

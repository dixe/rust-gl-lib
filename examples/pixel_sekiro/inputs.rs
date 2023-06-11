use sdl2::event;
use std::time::{
    Instant,
    Duration
};
use gl_lib::imode_gui::ui::*;

#[derive(Default)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    // TODO: make this better
    mouse: Option<Instant>,
    pub space: bool
}


impl Inputs {
    pub fn mouse(&mut self) -> bool {
        if let Some(_) = self.mouse {
            self.mouse = None;
            return true;
        }
        false
    }
}


pub fn handle_inputs(ui: &mut Ui, inputs: &mut Inputs) {

    use event::Event::*;
    use sdl2::keyboard::Keycode::*;


    // update input buffering

    if let Some(inst) = inputs.mouse {
        let dur = inst.elapsed();
        if dur.as_millis() > 5000 {
            inputs.mouse = None
        }
    }

    for e in &ui.frame_events {
        match e {
            KeyDown { keycode: Some(D), ..} => {
                inputs.right = true;
            },

            KeyDown { keycode: Some(Space), ..} => {
                inputs.space = true;
            },

            KeyUp { keycode: Some(Space), ..} => {
                inputs.space = false;
            },

            KeyDown { keycode: Some(A), ..} => {
                inputs.left = true;
            },

            KeyUp { keycode: Some(D), ..} => {
                inputs.right = false;
            },

            KeyUp { keycode: Some(A), ..} => {
                inputs.left = false;
            },

            MouseButtonUp {..} => {
                inputs.mouse = Some(Instant::now());
            }
            _ => {}
        }
    }
}

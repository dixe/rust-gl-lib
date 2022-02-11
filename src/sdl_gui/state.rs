use crate::sdl_gui::layout::Node;
use crate::sdl_gui::window;
use std::fmt;

pub trait State<Message> where Message: fmt::Debug {

    fn handle_message(&mut self, message: &Message, windows_acces: &window::WindowComponentAccess);

    fn view(&self) -> Node<Message>;

}

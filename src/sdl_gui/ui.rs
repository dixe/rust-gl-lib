use crate::sdl_gui::layout::Node;
use crate::sdl_gui::window;
use std::fmt;

/// Implement this to be able to generate ui using view and
/// handle message from interaction with the view.

pub trait Ui<Message> where Message: fmt::Debug {

    /// Handler for user specified message. Will be trigger by components set in view.
    /// An example is a buttom can have a message that is sent on click. This message is then handles here
    fn handle_message(&mut self, message: &Message, windows_acces: &window::WindowComponentAccess);

    /// Define the view of this UI
    fn view(&self) -> Node<Message>;

    fn handle_events(&mut self, event: sdl2::event::Event) {

    }

}

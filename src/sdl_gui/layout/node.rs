use crate::sdl_gui::layout::element::*;

pub type Node<Message> = Box<dyn Element<Message>>;

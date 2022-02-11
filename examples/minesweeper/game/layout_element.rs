use gl_lib::sdl_gui::layout::*;
use gl_lib::{
        gl,
        text_rendering::{ text_renderer::TextRenderer },
    };

use gl_lib::sdl_gui::components::base::*;
use std::fmt;
use crate::game::*;

#[derive(Debug)]
pub struct GameLayout<Message> {
    attributes: Attributes,
    left_clicked_message: fn(Point) -> Message,
    right_clicked_message: fn(Point) -> Message,
    game_info: GameInfo
}




impl<Message> GameLayout<Message> where Message: Clone {
    pub fn new(game_info: GameInfo, left_clicked_message: fn(Point) -> Message, right_clicked_message: fn(Point) -> Message) -> Self {

        Self {
            attributes: Default::default(),
            left_clicked_message,
            right_clicked_message,
            game_info
        }
    }
}



impl<Message> Element<Message> for GameLayout<Message> where Message: 'static + Clone + fmt::Debug {

    fn name(&self) -> String {
        "GameLayout".to_string()
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn content_height(&self, available_space: &RealizedSize, _text_renderer: &TextRenderer) -> f32 {
        available_space.height
    }

    fn content_width(&self, available_space: &RealizedSize, _text_renderer: &TextRenderer) -> f32 {
        available_space.width
    }

    fn create_component(&self, gl: &gl::Gl, comp_base: ComponentBase) -> Option<Component<Message>> {
        let mut game: Component<Message> = GameComponent::new(gl, self.game_info, self.left_clicked_message, self.right_clicked_message);
        game.set_base(comp_base);
        Some(game)
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> where Message: fmt::Debug {
        None
    }

}


impl<Message: 'static> From<GameLayout<Message>> for Node<Message>
where
    Message: Clone + fmt::Debug   {

    fn from(game: GameLayout<Message>) -> Node<Message> {
        Box::new(game)
    }

}

use super::*;
use crate::sdl_gui::components::base::*;
use crate::sdl_gui::components::button as comp_btn;
use crate::sdl_gui::layout::attributes::{Attributes};
use crate::sdl_gui::layout::element::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::gl;
use crate::sdl_gui::layout::node::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Button<Message> {
    content: String,
    attributes: Attributes,
    on_click_msg: Option<Message>
}

impl<Message> Button<Message> where Message: Clone + fmt::Debug {
    pub fn new(content: &str, msg: Option<Message>) -> Self {
        Self {
            content: content.to_string(),
            attributes: Default::default(),
            on_click_msg: msg
        }
    }
}


impl<Message> Element<Message> for Button<Message> where Message: 'static + Clone + fmt::Debug{

    fn name(&self) -> String {
        format!("button ({})", &self.content)
    }
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let max_width = self.contrainted_width(available_space);
        TextRenderer::render_box(text_renderer.font(), &self.content, max_width, 20).total_height
    }

    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let max_width = self.contrainted_width(available_space);
        TextRenderer::render_box(text_renderer.font(), &self.content, max_width, 20).total_width
    }

    fn create_component(&self, gl: &gl::Gl, comp_base: ComponentBase) -> Option<Component<Message>> {
        let mut btn: Component<Message> = comp_btn::Button::new(gl, &self.content, self.on_click_msg.clone());
        btn.set_base(comp_base);
        Some(btn)
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> where Message: fmt::Debug {
        None
    }

}


impl<Message: 'static> From<Button<Message>> for Node<Message>
where
    Message: Clone + fmt::Debug   {

    fn from(button: Button<Message>) -> Node<Message> {
        Box::new(button)
    }

}

use crate::sdl_gui::layout::*;
use crate::sdl_gui::components::base::*;
use crate::sdl_gui::components::text_box as comp_text_box;
use crate::sdl_gui::layout::attributes::Attributes;
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::gl;
use crate::sdl_gui::layout::node::Node;
use std::fmt;


#[derive(Clone, Debug)]
pub struct TextBox<Message> {
    content: String,
    attributes: Attributes,
    on_change: fn(String) -> Message,
}

impl<Message> TextBox<Message> where Message: Clone + fmt::Debug {
    pub fn new(content: &str, msg: fn(String) -> Message) -> Self {
        Self {
            content: content.to_string(),
            attributes: Default::default(),
            on_change: msg
        }
    }
}

impl<Message> Element<Message> for TextBox<Message> where Message: 'static + Clone + fmt::Debug{

    fn name(&self) -> String {
        format!("Textbox ({})", &self.content)
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let max_width = self.contrainted_width(available_space);
        let content_min = TextRenderer::render_box(text_renderer.font(), "TextBox", max_width, 20).total_height;
        let content_h = TextRenderer::render_box(text_renderer.font(), &self.content, max_width, 20).total_height;
        f32::max(content_min, content_h)
    }

    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let max_width = self.contrainted_width(available_space);

        let content_min = TextRenderer::render_box(&text_renderer.font(), "TextBox", max_width, 20).total_width;

        let content_w = TextRenderer::render_box(&text_renderer.font(), &self.content, max_width, 20).total_width;

        f32::max(content_min, content_w)

    }

    fn create_component(&self, gl: &gl::Gl, comp_base: ComponentBase) -> Option<Component<Message>> {
        let mut tb: Component<Message> = comp_text_box::TextBox::new(gl, &self.content, self.on_change);
        tb.set_base(comp_base);
        Some(tb)
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> where Message: fmt::Debug {
        None
    }
}




impl<Message: 'static> From<TextBox<Message>> for Node<Message>
where
    Message: Clone + fmt::Debug   {

    fn from(textbox: TextBox<Message>) -> Node<Message> {
        Box::new(textbox)
    }

}

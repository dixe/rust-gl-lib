use crate::sdl_gui::layout::attributes::{*};
use crate::sdl_gui::layout::element::*;
use crate::sdl_gui::layout::node::*;
use super::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct Column<Message> where Message: fmt::Debug {
    children: VecDeque::<Node<Message>>,
    attributes: Attributes
}


impl<Message> Column<Message> where Message: fmt::Debug {

    pub fn new() -> Self {

        Column {
            children: VecDeque::new(),
            attributes: Default::default(),
        }
    }

    pub fn add<E>(mut self, el: E) -> Self
    where
        E: Into<Node<Message>> {
        self.children.push_back(el.into());
        self
    }
}


impl<Message> Element<Message> for Column<Message> where Message: fmt::Debug {

    fn name(&self) -> String {
        "Column".to_string()
    }

    fn height_children(&self) -> i32 {
        self.children.len() as i32
    }


    fn distribution_dir(&self) -> Direction {
        Direction::Y
    }


    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_height = 0.;
        for c in &self.children {
            abs_height += c.final_height(available_space, text_renderer, OnFill::Shrink);
        }

        let child_spacing_count = self.children.len().max(1) - 1;
        let spacing = self.attributes().spacing;

        let h = abs_height + child_spacing_count as f32 * spacing.y;

        h
    }


    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_width = 0.;
        for c in &self.children {
            abs_width += c.final_width(available_space, text_renderer, OnFill::Shrink);
        }

        abs_width
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> {
        self.children.pop_front()
    }
}

impl<Message: 'static> From<Column<Message>> for Node<Message>
where
    Message: fmt::Debug  {

    fn from(column: Column<Message>) -> Node<Message> {
        Box::new(column)
    }
}

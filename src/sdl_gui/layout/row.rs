use crate::sdl_gui::layout::attributes::{*};
use crate::sdl_gui::layout::element::*;
use super::*;
use crate::sdl_gui::layout::node::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct Row<Message> where Message: fmt::Debug {
    children: VecDeque::<Node<Message>>,
    attributes: Attributes
}


impl<Message> Row<Message> where Message: fmt::Debug {


    pub fn new() -> Self {
        Self {
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

    pub fn add_if<E>(mut self, condition: bool, el: E ) -> Self
    where
        E: Into<Node<Message>> {
        if condition {
            self.children.push_back(el.into());
        }
        self
    }


    pub fn add_option<E>(mut self, el: Option::<E> ) -> Self
    where
        E: Into<Node<Message>> {
        if let Some(e) = el {
            self.children.push_back(e.into());
        }
        self
    }
}


impl<Message> Element<Message> for Row<Message> where Message: fmt::Debug {

    fn name(&self) -> String {
        "row".to_string()
    }
    fn width_children(&self) -> i32 {
        self.children.len() as i32
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
            abs_height = f32::max(abs_height, c.final_height(available_space, text_renderer, OnFill::Shrink))
        }
        abs_height
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



impl<Message: 'static> From<Row<Message>> for Node<Message>
where
    Message: fmt::Debug  {


    fn from(row: Row<Message>) -> Node<Message> {
        Box::new(row)
    }
}

use crate::sdl_gui::layout::attributes::{self, LengthConstraint, Alignment, Padding, Spacing};
use crate::sdl_gui::layout::OnFill;
use crate::sdl_gui::layout::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::gl;
use crate::sdl_gui::components::container;
use std::fmt;


mod internal_types;
use internal_types::*;

mod engine;

#[derive(Debug)]
pub struct AlignedElement<Message> where Message: fmt::Debug {
    pub node: Node<Message>,
    pub realized_size: RealizedSize
}

pub fn align_tree<Message>(root: Node<Message>, output_size: Size, text_renderer: &TextRenderer) -> Vec::<AlignedElement<Message>> where Message: fmt::Debug {

    let mut sized_tree = get_sized_tree(root, output_size, text_renderer);

    engine::align_tree(&mut sized_tree);

    let mut elements = Vec::new();
    map_size_to_aligned(sized_tree, &mut elements);

    elements

}

pub fn add_tree_to_container<Message>(gl: &gl::Gl, container: &mut container::ComponentContainer<Message>, elements: &Vec::<AlignedElement<Message>>) where Message: fmt::Debug + Clone {

    for e in elements {
        let base = e.realized_size.into();
        if let Some(comp) = e.node.create_component(gl, base) {
            container.add_component(comp);
        }
    }
}


fn map_size_to_aligned<Message>(mut sized_node: NodeWithSize<Message>, elements: &mut Vec::<AlignedElement<Message>>) where Message: fmt::Debug {

    while let Some(child) = sized_node.children.pop() {
        map_size_to_aligned(child, elements);
    }

    elements.push(AlignedElement {
        node: sized_node.node,
        realized_size: sized_node.layout.into()
    });

}


fn get_sized_tree<Message>(mut node: Node<Message>, available_space: Size, text_renderer: &TextRenderer) -> NodeWithSize<Message> where Message: fmt::Debug {

    let attribs = *node.attributes();
    let padding = attribs.padding;

    let content_size = Size {
        w: final_width(&node, &attribs, available_space, text_renderer, OnFill::Expand) - padding.right - padding.left,
        h: final_height(&node, &attribs, available_space, text_renderer, OnFill::Expand) - padding.bottom - padding.top,
    };

    let engine_w = match attribs.width {
        attributes::Length::Px(px) => EngineLength::Px(px as f32),
        attributes::Length::Fill => EngineLength::FillPortion(1.0),
        attributes::Length::FillPortion(p) => EngineLength::FillPortion(p as f32),
        attributes::Length::FitContent => EngineLength::Px(node.final_width(&content_size.into(), text_renderer, OnFill::Shrink) as f32),
    };

    let engine_h = match attribs.height {
        attributes::Length::Px(px) => EngineLength::Px(px as f32),
        attributes::Length::Fill => EngineLength::FillPortion(1.0),
        attributes::Length::FillPortion(p) => EngineLength::FillPortion(p as f32),
        attributes::Length::FitContent => EngineLength::Px(node.final_height(&content_size.into(), text_renderer, OnFill::Shrink) as f32),
    };

    //println!("{}.Engine_h = {:?}", node.name(), engine_h);
    let mut children = Vec::new();

    let width_children = node.width_children();

    let height_children = node.height_children();

    while let Some(child) = node.pop_children_front() {
        children.push(get_sized_tree(child, content_size, text_renderer));
    }

    let layout = LayoutElement::new(engine_w, engine_h, attribs, content_size, width_children, height_children);

    NodeWithSize { node, layout, children }


}

fn final_height<Message>(node: &Node<Message>, attributes: &attributes::Attributes, available_space: Size, text_renderer: &TextRenderer, on_fill: OnFill) -> f32 where Message: fmt::Debug {

    let h = match attributes.height {
        attributes::Length::Px(px) => {
            px as f32
        },
        attributes::Length::FitContent => node.final_height(&(available_space.into()), text_renderer, OnFill::Shrink),
        _ => match on_fill {
            OnFill::Expand => available_space.h,
            OnFill::Shrink => node.final_height(&(available_space.into()), text_renderer, OnFill::Shrink),
        }
    };

    let min = attributes.height_constraint.min();
    let max = attributes.height_constraint.max(available_space.h);

    f32::min(max, f32::max(min, h))

}

fn final_width<Message>(node: &Node<Message>, attributes: &attributes::Attributes, available_space: Size, text_renderer: &TextRenderer, on_fill: OnFill) -> f32 where Message: fmt::Debug {

    let w = match attributes.width {
        attributes::Length::Px(px) => {
            px as f32
        },
        attributes::Length::FitContent => node.final_width(&(available_space.into()), text_renderer, OnFill::Shrink),
        _ => match on_fill {
            OnFill::Expand => available_space.w,
            OnFill::Shrink => node.final_width(&(available_space.into()), text_renderer, OnFill::Shrink),
        }
    };

    let min = attributes.width_constraint.min();
    let max = attributes.width_constraint.max(available_space.w);

    f32::min(max, f32::max(min, w))

}


#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32
}


#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: f32,
    pub h: f32
}


impl From<&viewport::Viewport> for Size {

    fn from(viewport: &viewport::Viewport) -> Self {
        Self {
            w: viewport.w as f32,
            h: viewport.h as f32
        }
    }
}

use crate::sdl_gui::layout::engine::*;
use std::fmt;

#[derive(Debug)]
pub struct NodeWithSize<Message> where Message: fmt::Debug {
    pub node: Node<Message>,
    pub layout: LayoutElement,
    pub children: Vec<NodeWithSize<Message>>
}


impl From<LayoutElement> for RealizedSize {
    fn from(layout: LayoutElement) -> Self {
        Self {
            x: layout.position.x,
            y: layout.position.y,
            width: layout.content_size.w,
            height: layout.content_size.h
        }

    }
}



/// Almost the same as a regular length, except that Fit Content has been calculated to a px value
#[derive(Debug, Clone, Copy)]
pub enum EngineLength {
    /// Length equal to given number of pixels
    Px(f32),

    /// Fill a portion
    /// If 1 child has FillPortion 1 and another has FillPortion 3
    /// Than the first child will have 1/4 of the space and the other child the
    /// remainin 3/4
    FillPortion(f32),
}


#[derive(Debug, Clone, Copy)]
pub struct EngineAttributes {
    pub width: EngineLength,
    pub height: EngineLength,
    pub width_constraint: LengthConstraint,
    pub height_constraint: LengthConstraint,
    pub align: Alignment,
    pub padding: Padding,
    pub spacing: Spacing,
    pub children_width_count: i32,
    pub children_height_count: i32,
}




#[derive(Debug, Clone, Copy)]
pub struct LayoutElement {
    pub attributes: EngineAttributes,
    pub content_size: Size,
    pub position: Point,
}

impl LayoutElement {

    pub fn new(width:EngineLength, height: EngineLength, attributes: attributes::Attributes, content_size: Size, children_width_count: i32, children_height_count: i32) -> Self {
        Self {
            attributes: EngineAttributes {
                width,
                height,
                width_constraint: attributes.width_constraint,
                height_constraint: attributes.height_constraint,
                align: attributes.align,
                padding: attributes.padding,
                spacing: attributes.spacing,
                children_width_count,
                children_height_count
            },
            content_size,
            position: Point::default(),
        }
    }

    pub fn width(&self) -> f32 {
        self.content_size.w + self.attributes.padding.top + self.attributes.padding.bottom
    }


    pub fn height(&self) -> f32 {
        self. content_size.h + self.attributes.padding.left + self.attributes.padding.right
    }

}

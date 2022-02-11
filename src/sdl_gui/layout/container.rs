use crate::components::container::*;
use crate::layout::attributes::{*};
use crate::layout::element::*;
use super::*;
use crate::layout::node::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::marker;
use std::fmt;

pub trait Container<Message>: Element<Message> where Message: fmt::Debug {


    fn children_height_info(&self, content_space: &RealizedSize, _text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Height> {
        ChildrenAbsAndFill::<Height> {
            abs_length: content_space.height,
            fill_count: 0,
            child_count: 0,
            _marker: marker::PhantomData::<Height>,
        }
    }

    fn children_width_info(&self, content_space: &RealizedSize, _text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Width> {
        ChildrenAbsAndFill::<Width> {
            abs_length: content_space.width,
            fill_count: 0,
            child_count: 0,
            _marker: marker::PhantomData::<Width>,
        }
    }

    fn calculate_child_spaces(&self, update_info: &UpdateInfo) -> Vec<RealizedSize>;


    fn children(&self) -> &Vec::<Node<Message>>;


}

fn align_child_spaces<Message>(children: &Vec<Node<Message>>, child_spaces: &mut Vec<RealizedSize>, used_space: &UsedSpace , content_space: &RealizedSize) where Message: fmt::Debug {


    let unused_x = f32::max(0.0, content_space.width - used_space.w) ;
    let unused_y = f32::max(0.0, content_space.height - used_space.h);


    match children[0].attributes().align.y {

        AlignmentY::Bottom => {
            println!("");
            println!("space {:?}", content_space);
            println!("{:?}", used_space);
            println!("(Unused_x, unused_y) ({:?},{})", unused_x, unused_y);

            println!("child_size {:?}", child_spaces[0]);
            println!("");
        },
        _ => {}
    };


    //println!("unysed {:#?}", child_spaces_info);
    align_child_spaces_x(children, child_spaces, content_space.width, unused_x);
    align_child_spaces_y(children, child_spaces, content_space.height, unused_y);

    match children[0].attributes().align.y {

        AlignmentY::Bottom => {
            println!("ALIGNED BOTTOM SPACE");
            println!("child_size {:?}", child_spaces[0]);
            println!("");
        },
        _ => {}
    };
}


fn align_child_spaces_x<Message>(children: &Vec::<Node<Message>>, child_spaces: &mut Vec::<RealizedSize>, content_width: f32, mut unused_x: f32) where Message: fmt::Debug {
    let mut center_elements_left = None;
    let mut center_elements_right = 0.0;


    for i in 0..children.len() {
        let c = &children[i];
        let cs = &mut child_spaces[i];


        match c.attributes().align.x {
            AlignmentX::Center => {
                match center_elements_left {
                    None => {
                        center_elements_left = Some(cs.x);
                    },
                    _ => {}// Already set we a previous element
                }

                center_elements_right = cs.x + cs.width;
            },

            AlignmentX::Right => { break }, // when we first align to the right, centering does nothing after
            _ => {}
        }
    }

    let mut center_elements_width = match center_elements_left {
        None => None,
        Some(left) => Some(center_elements_right - left)
    };


    let mut x_offset = 0.0;

    for i in 0..children.len() {
        let c = &children[i];
        let cs = &mut child_spaces[i];


        match c.attributes().align.x {
            AlignmentX::Left => {}, //default is left, do nothing},
            AlignmentX::Center => {
                match center_elements_width {
                    None => {},
                    Some(offset) => {

                        let desired_x = content_width/2.0 - offset/2.0 - center_elements_left.unwrap();
                        let new_offset = f32::max(0.0, desired_x);
                        x_offset += new_offset;
                        unused_x -= new_offset;
                        center_elements_width = None;
                    }
                }
            },
            AlignmentX::Right => {
                // take all remaning space to the right and offset by that
                x_offset += f32::max(0.0, unused_x);

                unused_x = 0.0;
            },

        }


        cs.x += x_offset;
    }

}


fn align_child_spaces_y<Message>(children: &Vec::<Node<Message>>, child_spaces: &mut Vec::<RealizedSize>, content_height: f32, mut unused_y: f32) where Message: fmt::Debug {
    let mut center_elements_top = None;
    let mut center_elements_bottom = 0.0;


    for i in 0..children.len() {

        let c = &children[i];
        let cs = &mut child_spaces[i];
        println!("{:?}", c.name());

        match c.attributes().align.y {
            AlignmentY::Center => {
                match center_elements_top {
                    None => {
                        center_elements_top = Some(cs.y);
                    },
                    _ => {}// Already set we a previous element
                }

                center_elements_bottom = cs.y + cs.height;
            },

            AlignmentY::Bottom => { break }, // when we first align to the bottom, centering does nothing after
            _ => {}
        }
    }

    let mut center_elements_height = match center_elements_top {
        None => None,
        Some(top) => Some(center_elements_bottom - top)
    };




    let mut y_offset = 0.0;

    for i in 0..children.len() {
        let c = &children[i];
        let cs = &mut child_spaces[i];


        match c.attributes().align.y {
            AlignmentY::Top => {},
            AlignmentY::Center => {
                match center_elements_height {
                    None => {},
                    Some(offset) => {

                        let desired_y = content_height/2.0 - offset/2.0 - center_elements_top.unwrap();
                        let new_offset = f32::max(0.0, desired_y);
                        y_offset += new_offset;
                        unused_y -= new_offset;
                        center_elements_height = None;
                    }
                }
            },
            AlignmentY::Bottom => {

                // take all remaning space to the bottom and offset by that
                y_offset += f32::max(0.0, unused_y);
                unused_y = 0.0;
            },

        }

        //println!("y_offset {:?}", y_offset);

        cs.y += y_offset;
    }

}




#[derive(Debug, Clone, Copy)]
struct UsedSpace {
    pub w: f32,
    pub h: f32,
}


pub struct Width {}
pub struct Height {}

#[derive(Debug, Clone, Copy)]
pub struct ChildrenAbsAndFill<T> {
    pub abs_length: f32,
    pub fill_count: u32,
    pub child_count: u32,
    pub _marker: marker::PhantomData<T>,
}


#[derive(Debug, Clone, Copy)]
pub struct NextStart {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Clone)]
pub struct ChildSpaceInfo {
    pub next: NextStart,
    pub child_spaces: Vec<RealizedSize>,
}


pub struct UpdateInfo<'a> {
    pub content_space: &'a RealizedSize,
    pub width_info: &'a ChildrenAbsAndFill<Width>,
    pub height_info: &'a ChildrenAbsAndFill<Height>,
    pub text_renderer: &'a TextRenderer,
    pub next: NextStart,
    pub spacing: &'a Spacing,

    pub dynamic_width: f32,
    pub dynamic_height: f32
}

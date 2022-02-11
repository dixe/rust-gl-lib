use crate::sdl_gui::layout::element::Direction;
use super::*;

pub fn align_tree<Message>(tree: &mut NodeWithSize<Message>) where Message: fmt::Debug {

    let layout = &tree.layout;

    // update children X to with parent padding
    for c in tree.children.iter_mut() {
        c.layout.position.x += layout.position.x + layout.attributes.padding.left;
        c.layout.position.y += layout.position.y + layout.attributes.padding.top;
    }

    distribute_children(tree);
    align_children(tree);

    for c in tree.children.iter_mut() {
        align_tree(c);
    }
}


fn distribute_children<Message>(tree: &mut NodeWithSize<Message>) where Message: fmt::Debug {
    match tree.node.distribution_dir() {
        Direction::X => distribute_children_x(tree),
        Direction::Y => distribute_children_y(tree),
    };
}

fn distribute_children_x<Message>(tree: &mut NodeWithSize<Message>) where Message: fmt::Debug {
    use EngineLength::*;

    if tree.children.len() == 0 {
        return;
    }

    //TODO:  store on layout?

    let fill_count = get_fill_count(tree, Direction::X);

    let mut abs_width = 0.0;
    for c in &tree.children {
        abs_width += match c.layout.attributes.width {
            Px(w) => w,
            _ => 0.0
        };
    }

    let spacing = tree.layout.attributes.spacing;
    let total_x_spacing = spacing.x * (i32::max(1, tree.layout.attributes.children_width_count) - 1) as f32;
    let dynamic_width = f32::max(0.0, (tree.layout.content_size.w - abs_width)  - total_x_spacing);
    let mut next_x_offset = 0.0;

    for c in tree.children.iter_mut() {
        c.layout.content_size.h = match c.layout.attributes.height {
            Px(px) => px as f32,
            _ => c.layout.content_size.h
        };

        c.layout.content_size.w = match c.layout.attributes.width {
            Px(px) => px,
            FillPortion(p) => (dynamic_width / fill_count) * p
        };

        c.layout.position.x += next_x_offset;

        next_x_offset += c.layout.content_size.w + spacing.x;
    }
}

fn distribute_children_y<Message>(tree: &mut NodeWithSize<Message>) where Message: fmt::Debug {
    use EngineLength::*;

    if tree.children.len() == 0 {
        return;
    }

    //TODO:  store on layout?

    //println!("Dist Y, children = {}", tree.children.len());
    let fill_count = get_fill_count(tree, Direction::Y);
    let mut abs_height = 0.0;
    for c in &tree.children {
        abs_height += match c.layout.attributes.height {
            Px(h) => {
                //println!("{:?}.attr = {:#?}", c.node.name(),c.layout);
                h + c.layout.attributes.padding.top + c.layout.attributes.padding.bottom
            },
            _ => 0.0
        };
    }


    let spacing = tree.layout.attributes.spacing;
    let total_y_spacing = spacing.y * (i32::max(1, tree.layout.attributes.children_height_count) - 1) as f32;
    let dynamic_height = f32::max(0.0, (tree.layout.height() - abs_height) - total_y_spacing);

    //println!("{}.dyn_h = {:?}, content_h = {} y_spacing = {} fill_count = {}", tree.node.name(), dynamic_height, tree.layout.content_size.h, total_y_spacing, fill_count);

    let mut next_y_offset = 0.0;
    for c in tree.children.iter_mut() {
        c.layout.content_size.w = match c.layout.attributes.width {
            Px(px) => px,
            FillPortion(_) => c.layout.content_size.w
        };

        c.layout.content_size.h = match c.layout.attributes.height {
            Px(px) => px,
            FillPortion(p) => (dynamic_height / fill_count) * p
        };


        c.layout.position.y += next_y_offset;

        //println!("{}.(h,y) = ({},{})", c.node.name(), c.layout.position.y, c.layout.height());
        next_y_offset += c.layout.height() + spacing.y;
    }
}


fn get_fill_count<Message>(tree: &NodeWithSize<Message>, dir: Direction) -> f32 where Message: fmt::Debug {

    let mut fc = 0.0;
    for c in &tree.children {
        fc += match dir {
            Direction::X => match c.layout.attributes.width {
                EngineLength::FillPortion(p) => p,
                _ => 0.0
            },
            Direction::Y => match c.layout.attributes.height {
                EngineLength::FillPortion(p) => p,
                _ => 0.0
            }
        };
    }

    fc
}


fn align_children<Message>(tree: &mut NodeWithSize<Message>) where Message: fmt::Debug {

    let layout = &tree.layout;
    let content_size = layout.content_size;

    let mut used_width = 0.0;
    let mut used_height = 0.0;


    for c in &tree.children {
        used_width += c.layout.content_size.w;
        used_height += c.layout.content_size.h;
    }


    used_width += layout.attributes.spacing.x * i32::max(0, layout.attributes.children_width_count) as f32;
    used_height += layout.attributes.spacing.y * i32::max(0, layout.attributes.children_height_count) as f32;

    let unused_x = f32::max(0.0, content_size.w - used_width);
    let unused_y = f32::max(0.0, content_size.h - used_height);

    align_children_x(tree, unused_x);
    align_children_y(tree, unused_y);


}

fn align_children_x<Message>(tree: &mut NodeWithSize<Message>, mut unused_x: f32) where Message: fmt::Debug {
    let mut center_elements_left = None;
    let mut center_elements_right = 0.0;

    for c in &tree.children {

        match c.layout.attributes.align.x {
            AlignmentX::Center => {
                match center_elements_left {
                    None => {
                        center_elements_left = Some(c.layout.position.x);
                    },
                    _ => {}// Already set we a previous element
                }

                //panic!("CENTER");

                center_elements_right = c.layout.position.x + c.layout.content_size.w;

            },

            AlignmentX::Right => { break }, // when we first align to the right, centering does nothing after
            _ => {}
        };
    }


    let mut center_elements_width = match center_elements_left {
        None => None,
        Some(left) => Some(center_elements_right - left)
    };

    let mut x_offset = 0.0;

    for c in tree.children.iter_mut() {

        match c.layout.attributes.align.x {
            AlignmentX::Left => {}, //default is left, do nothing},
            AlignmentX::Center => {
                match center_elements_width {
                    None => {},
                    Some(offset) => {
                        let desired_x = tree.layout.content_size.w / 2.0 - offset / 2.0 - center_elements_left.unwrap();
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

        c.layout.position.x += x_offset;
    }


}


fn align_children_y<Message>(tree: &mut NodeWithSize<Message>, mut unused_y: f32) where Message: fmt::Debug {
    let mut center_elements_left = None;
    let mut center_elements_right = 0.0;



    for c in &tree.children {

        match c.layout.attributes.align.y {
            AlignmentY::Center => {
                match center_elements_left {
                    None => {
                        center_elements_left = Some(c.layout.position.y);
                    },
                    _ => {}// Already set we a previous element
                }

                center_elements_right = c.layout.position.y + c.layout.content_size.h;
            },

            AlignmentY::Bottom => { break }, // when we first align to the right, centering does nothing after
            _ => {}
        };
    }


    let mut center_elements_height = match center_elements_left {
        None => None,
        Some(left) => Some(center_elements_right - left)
    };

    let mut y_offset = 0.0;

    for c in tree.children.iter_mut() {
        match c.layout.attributes.align.y {
            AlignmentY::Top => {}, //default is top, do nothing},
            AlignmentY::Center => {
                match center_elements_height {
                    None => {},
                    Some(offset) => {
                        let desired_y = c.layout.content_size.h / 2.0 - offset / 2.0 - center_elements_left.unwrap();
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
        c.layout.position.y += y_offset;

    }
}

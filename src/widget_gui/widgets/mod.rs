use crate::widget_gui::*;
use crate::widget_gui::layout::*;


mod text_widget;
pub use self::text_widget::*;


mod counter_widget;
pub use self::counter_widget::*;


mod container_widgets;
pub use self::container_widgets::*;

mod button_widget;
pub use self::button_widget::*;

mod slider_widget;
pub use self::slider_widget::*;


// TODO: belongs in layout not widgets
fn preprocess_children(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext, flex_dir: FlexDir) -> Option<LayoutResult> {

    // Process unflexible children first
    for &child_id in children {

        match ctx.attributes[child_id].constraint(flex_dir) {
            SizeConstraint::Flex(_) => continue,
            _ => {}
        };


        // Child without flex
        if ctx.layout_geom[child_id] == None {
            // TODO: Maybe not use bc, but create new with width inf, pr https://www.youtube.com/watch?v=UUfXWzp0-DU
            return Some(LayoutResult::RequestChild(child_id, bc.clone()));
        }
    }


    let flex_info = calc_flex_info(bc, children, ctx, flex_dir);


    // Flex children
    if flex_info.sum_flex_factor !=  0 {

        // Process flex children when we have the total flex
        for &child_id in children {

            let flex_factor : Pixel = match ctx.attributes[child_id].constraint(flex_dir) {
                SizeConstraint::Flex(factor) => factor.into(),
                SizeConstraint::Fixed(_) => continue,
                SizeConstraint::NoFlex => continue,
            };


            // Flex children here
            if ctx.layout_geom[child_id] == None {
                let bc_child = match flex_dir {
                    FlexDir::X => BoxContraint::fixed_width(flex_factor * flex_info.space_per_flex, bc.max_h),
                    FlexDir::Y => BoxContraint::fixed_height(bc.max_w, flex_factor * flex_info.space_per_flex)
                };

                return Some(LayoutResult::RequestChild(child_id, bc_child));
            };
        }
    }

    None
}


fn calc_flex_info(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext, flex_dir: FlexDir) -> FlexInfo {

    // calc x and y flex size
    let mut free_space = match flex_dir {
        FlexDir::X => bc.max_w,
        FlexDir::Y => bc.max_h
    };


    let mut sum_flex_factor : Pixel = 0;

    for &child_id in children {

        match ctx.attributes[child_id].constraint(flex_dir) {
            SizeConstraint::NoFlex => {
                free_space -= ctx.layout_geom[child_id].as_ref().unwrap().size.from_flex(flex_dir);
            },
            SizeConstraint::Fixed(px) => {
                free_space -= px;
            },
            SizeConstraint::Flex(factor) => {
                sum_flex_factor += Pixel::from(factor);
            }
        };
    }

    let mut space_per_flex : Pixel = 0;

    if sum_flex_factor != 0 {
        space_per_flex = free_space / sum_flex_factor
    }

    FlexInfo {
        sum_flex_factor,
        space_per_flex
    }
}



fn fill_row(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {

    let flex_dir = FlexDir::X;
    match preprocess_children(bc, children, ctx, flex_dir) {
        Some(lr) => {
            return lr;
        },
        None => {}
    };


    // We now have all children sizes, clone their geometries
    let mut child_geoms = vec![];
    for &id in children.iter() {
        if let Some(geom) = &ctx.layout_geom[id] {
            child_geoms.push((id, geom.clone(), &ctx.attributes[id]));
        }
    }


    // layout the children in a row. With the flexes and with the children alignemnt

    let mut size = Size { pixel_w: 0, pixel_h: 0 };
    let mut offset = 0;

    // So we want to lay out the children (change pos). If all is centered, they should all form around the center
    // the first one should not be centered, and then the rest to the right
    // maybe start out by splitting into left, right and center.


    let mut center_width = 0;
    let mut right_width = 0;


    let mut cur_align_x = AlignmentX::Left;

    let total_w = bc.max_w - bc.min_w;

    for (_id, geom, attrib) in &child_geoms {

        // Update current align based on the input
        match cur_align_x {
            AlignmentX::Left => {
                cur_align_x = attrib.alignment.x;
            },
            AlignmentX::Center => {
                if attrib.alignment.x == AlignmentX::Right {
                    cur_align_x = AlignmentX::Right;
                }
            },
            _ => {}
        };


        match cur_align_x {
            AlignmentX::Left => {
            },
            AlignmentX::Center => {
                center_width += geom.size.pixel_w;

            },
            AlignmentX::Right => {
                right_width += geom.size.pixel_w;
            }
        };
    }

    let center_start_left =  total_w / 2 - center_width / 2;
    let right_start_left = total_w - right_width;

    // if center bleed into right aligned children, center_optimal_start should move to the left.
    // if left bleed into center aligned children, center_optimal_start should move to the right.
    // All sizes should fit inside our container, after we have preprocessed them

    cur_align_x = AlignmentX::Left;

    for (id, child, attrib) in &child_geoms {

        if cur_align_x == AlignmentX::Left && attrib.alignment.x == AlignmentX::Center {
            cur_align_x = AlignmentX::Center;
            offset = center_start_left;
        }

        if cur_align_x != AlignmentX::Right && attrib.alignment.x == AlignmentX::Right {
            offset = right_start_left;
        }

        if let Some(ref mut g) = &mut ctx.layout_geom[*id] {
            g.pos.x += offset;
        }

        size.pixel_w = size.pixel_w + child.size.pixel_w;
        size.pixel_h = Pixel::max(size.pixel_h, child.size.pixel_h);

        offset += child.size.from_flex(flex_dir);
    }

    LayoutResult::Size(size)
}


fn fill_column(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {

    let flex_dir = FlexDir::Y;
    match preprocess_children(bc, children, ctx, flex_dir) {
        Some(lr) => {
            return lr;
        },
        None => {}
    };


    // We now have all children sizes, clone their geometries
    let mut child_geoms = vec![];
    for &id in children.iter() {
        if let Some(geom) = &ctx.layout_geom[id] {
            child_geoms.push((id, geom.clone(), &ctx.attributes[id]));
        }
    }


    // layout the children in a row. With the flexes and with the children alignemnt

    let mut size = Size { pixel_w: 0, pixel_h: 0 };
    let mut offset = 0;

    // So we want to lay out the children (change pos). If all is centered, they should all form around the center
    // the first one should not be centered, and then the rest to the bottom
    // maybe start out by splitting into left, bottom and center.

    let mut center_height = 0;
    let mut bottom_height = 0;


    let mut cur_align_y = AlignmentY::Top;

    let total_h = bc.max_h - bc.min_h;

    for (_id, geom, attrib) in &child_geoms {

        // Update current align based on the input
        match cur_align_y {
            AlignmentY::Top => {
                cur_align_y = attrib.alignment.y;
            },
            AlignmentY::Center => {
                if attrib.alignment.y == AlignmentY::Bottom {
                    cur_align_y = AlignmentY::Bottom;
                }
            },
            _ => {}
        };


        match cur_align_y {
            AlignmentY::Top => {

            },
            AlignmentY::Center => {
                center_height += geom.size.pixel_h;

            },
            AlignmentY::Bottom => {
                bottom_height += geom.size.pixel_h;
            }
        };
    }

    let center_start_top =  total_h / 2 - center_height / 2;
    let bottom_start_top = total_h - bottom_height;

    // if center bleed into bottom aligned children, center_optimal_start should move to the top.
    // if top bleed into center aligned children, center_optimal_start should move to the bottom.
    // All sizes should fit inside our container, after we have preprocessed them

    cur_align_y = AlignmentY::Top;

    for (id, child, attrib) in &child_geoms {

        if cur_align_y == AlignmentY::Top && attrib.alignment.y == AlignmentY::Center {
            cur_align_y = AlignmentY::Center;
            offset = center_start_top;
        }

        if cur_align_y != AlignmentY::Bottom && attrib.alignment.y == AlignmentY::Bottom {
            offset = bottom_start_top;
        }

        if let Some(ref mut g) = &mut ctx.layout_geom[*id] {
            g.pos.add_by_flex(offset, flex_dir);
        }

        size.pixel_w = Pixel::max(size.pixel_w, child.size.pixel_w);
        size.pixel_h = size.pixel_h + child.size.pixel_h;

        offset += child.size.from_flex(flex_dir);
    }

    LayoutResult::Size(size)
}

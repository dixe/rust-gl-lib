use crate::widget_gui::*;

#[derive(Debug, Clone)]
pub struct TextWidget {
     pub text: String
}


impl Widget for TextWidget {
    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], _ctx: &mut LayoutContext) -> LayoutResult {
        println!("TexBox Bc{:?}", bc);
        LayoutResult::Size(Size {
            pixel_w: Pixel::min(bc.max_w, Pixel::max(100, bc.min_w)),
            pixel_h: Pixel::min(bc.max_h, Pixel::max(30, bc.min_h))
        })
    }
}

#[derive(Debug, Clone)]
pub struct RowWidget {
    // info about how to expand maybe??
}



impl Widget for RowWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        fill_container(bc, children, ctx, FlexDir::X)
    }
}


#[derive(Debug, Clone)]
pub struct ColumnWidget {
    // info about how to expand maybe??
}


fn preprocess_children(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext, flex_dir: FlexDir) -> Option<LayoutResult> {

    // Process unflexible children first
    for &child_id in children {

        if ctx.size_constraints[child_id].constraint(flex_dir) != SizeConstraint::NoFlex {
            continue
        }


        // Child without flex
        if ctx.widget_geometry[child_id] == None {
            println!("NO FLEX");
            // TODO: Maybe not use bc, but create new with width inf, pr https://www.youtube.com/watch?v=UUfXWzp0-DU
            return Some(LayoutResult::RequestChild(child_id, bc.clone()));
        }
    }


    let flex_info = calc_flex_info(bc, children, ctx, flex_dir);

    // Flex children
    if flex_info.sum_flex_factor !=  0 {

        // Process flex children when we have the total flex
        for &child_id in children {

            let flex_factor = match ctx.size_constraints[child_id].constraint(flex_dir) {
                SizeConstraint::Flex(factor) => factor,
                SizeConstraint::NoFlex => continue,
            };

            // Flex children here
            if ctx.widget_geometry[child_id] == None {
                // TODO: match on fill dir

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


struct FlexInfo {
    space_per_flex: Pixel,
    sum_flex_factor: Pixel,
}


fn calc_flex_info(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext, flex_dir: FlexDir) -> FlexInfo {

    // calc x and y flex size
    let mut free_space = match flex_dir {
        FlexDir::X => bc.max_w,
        FlexDir::Y => bc.max_h
    };


    let mut sum_flex_factor = 0;

    for &child_id in children {

        match ctx.size_constraints[child_id].constraint(flex_dir) {
            SizeConstraint::NoFlex => {
                free_space -= ctx.widget_geometry[child_id].as_ref().unwrap().size.from_flex(flex_dir);
            },
            SizeConstraint::Flex(factor) => {
                sum_flex_factor += factor;
            }
        };
    }

    let mut space_per_flex = 0;

    if sum_flex_factor != 0 {
        space_per_flex = free_space / sum_flex_factor
    }

    FlexInfo {
        sum_flex_factor,
        space_per_flex
    }
}


fn fill_container(bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext, flex_dir: FlexDir) -> LayoutResult {

    match preprocess_children(bc, children, ctx, flex_dir) {
        Some(lr) => {
            return lr;
        },
        None => {}
    }


    let mut child_geoms = vec![];
    for &id in children.iter() {
        if let Some(geom) = &ctx.widget_geometry[id] {
            child_geoms.push((id, geom.clone()));
        }
    }


    let mut size = Size { pixel_w: 0, pixel_h: 0 };
    let mut offset = 0;

    for (id, child) in child_geoms {
        if let Some(ref mut g) = &mut ctx.widget_geometry[id] {
            g.pos.add_by_flex(offset, flex_dir);
        }


        size.pixel_w = match flex_dir {
            FlexDir::X => size.pixel_w + child.size.pixel_w,
            FlexDir::Y => Pixel::max(size.pixel_w, child.size.pixel_w)
        };

        size.pixel_h = match flex_dir {
            FlexDir::X => Pixel::max(size.pixel_h, child.size.pixel_h),
            FlexDir::Y => size.pixel_h + child.size.pixel_h
        };

        offset += child.size.from_flex(flex_dir);
    }


    LayoutResult::Size(size)
}



impl Widget for ColumnWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        fill_container(bc, children, ctx, FlexDir::Y)
    }
}

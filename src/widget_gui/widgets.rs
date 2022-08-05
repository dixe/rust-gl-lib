use crate::widget_gui::*;

#[derive(Debug, Clone)]
pub struct TextWidget {
     pub text: String
}


impl Widget for TextWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        LayoutResult::Size(Size { pixel_w: usize::min(bc.max_w,100), pixel_h: usize::min(30, bc.max_h) })

    }
}


pub struct ContainerWidget {
    // info about how to expand maybe??
}



impl Widget for ContainerWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {

        // get size for all children, if any is missing return request for child to be calculated
        for &child_id in children {

            if ctx.widget_sizes[child_id] == None {
                return LayoutResult::RequestChild(child_id, bc.clone());
            }
        }

        let child_sizes: Vec::<Size> = children.into_iter().map(|c| ctx.widget_sizes[*c]).filter_map(|e| e).collect();

        let mut size = Size { pixel_w: 0, pixel_h: 0 } ;

        println!("{:#?}", ctx.widget_sizes);

        for child in child_sizes {
            size.pixel_w += child.pixel_w;
            size.pixel_h += child.pixel_h;
        }

        LayoutResult::Size(size)
    }
}

use crate::widget_gui::*;
use crate::widget_gui::render;
use crate::text_rendering::text_renderer::TextRenderer;
use std::cell::RefCell;
use std::rc::Rc;
use num_traits::Num;


#[derive(Debug, Clone)]
pub struct CounterWidget<T: Num + std::fmt::Display> {
    pub count: Rc<RefCell::<T>>
}


impl<T: Num + std::fmt::Display> Widget for CounterWidget<T> {
    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {

        let text_size = TextRenderer::render_box(ctx.font, &format!("{:.2}", self.count.borrow()), bc.max_w as f32, 1.0);

        LayoutResult::Size(Size {
            pixel_w: Pixel::min(bc.max_w, Pixel::max(text_size.total_width as i32, bc.min_w)),
            pixel_h: Pixel::min(bc.max_h, Pixel::max(text_size.total_height as i32, bc.min_h))
        })
    }


    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_text(&format!("{:.2}", self.count.borrow()), 1.0, geom, ctx);
    }
}

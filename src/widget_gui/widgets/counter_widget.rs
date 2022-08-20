use crate::widget_gui::*;
use crate::widget_gui::render;
use crate::text_rendering::text_renderer::TextRenderer;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CounterWidget {
    pub count: Rc<RefCell::<i32>>
}


impl Widget for CounterWidget {
    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {

        let text_size = TextRenderer::render_box(ctx.font, &format!("{}", self.count.borrow()), bc.max_w as f32, 1.0);
        LayoutResult::Size(Size {
            pixel_w: Pixel::min(bc.max_w, Pixel::max(text_size.total_width as i32, bc.min_w)),
            pixel_h: Pixel::min(bc.max_h, Pixel::max(text_size.total_height as i32, bc.min_h))
        })
    }


    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_text(&format!("{}", self.count.borrow()), 1.0, geom, ctx);
    }

    fn dispatcher(&self) -> Dispatcher {
        Box::new(counter_dispatcher)
    }
}


fn counter_dispatcher(event: &event::Event, self_id: Id, queue: &mut DispatcherQueue) {
    use event::Event::*;
    match event {
        TextInput { text, ..} => {
            match text.as_str() {
                " " => {
                    queue.push_back(DispatcherEvent {target_id: self_id, event: Box::new(42i32)});
                },
                _ => {}
            }
        }
        _ => {}
    };

}

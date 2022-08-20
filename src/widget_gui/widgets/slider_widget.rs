use crate::widget_gui::*;
use crate::widget_gui::render;
use crate::text_rendering::text_renderer::TextRenderer;



#[derive(Debug, Clone)]
pub struct SliderWidget {
    pub text_left: Option<String>,
    pub text_right: Option<String>,
}


impl Widget for SliderWidget {

    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        let text_size = TextRenderer::render_box(ctx.font, "a", bc.max_w as f32, 1.0);

        // TODO: Implement infinite max width, here to let layout plugin sa, I don't care aboubt my width, but i want to be as wide as i can get to be
        LayoutResult::Size(Size {
            pixel_w: bc.max_w,
            pixel_h: Pixel::min(bc.max_h, Pixel::max(text_size.total_height as i32, bc.min_h))
        })
    }


    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        println!("{:?}", geom);
        render::render_round_rect(geom, ctx);
    }


    fn dispatcher(&self) -> Dispatcher {
        Box::new(slider_dispatcher)
    }

}


fn slider_dispatcher(event: &event::Event, self_id: Id, queue: &mut DispatcherQueue) {
    use event::Event::*;
    match event {
        MouseButtonUp {..} => {
            queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
        },
        _ => {}
    };
}

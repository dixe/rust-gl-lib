use crate::widget_gui::*;
use crate::widget_gui::render;
use crate::text_rendering::text_renderer::TextRenderer;



#[derive(Debug, Clone)]
pub struct ButtonWidget {
    pub text: String,
    pub text_scale: f32,

}


impl Widget for ButtonWidget {
    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        let text_size = TextRenderer::render_box(ctx.font, &self.text, bc.max_w as f32, 1.0);
        LayoutResult::Size(Size {
            pixel_w: Pixel::min(bc.max_w, Pixel::max(text_size.total_width as i32, bc.min_w)),
            pixel_h: Pixel::min(bc.max_h, Pixel::max(text_size.total_height as i32, bc.min_h))
        })
    }



    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_round_rect(geom, ctx);
        render::render_text(&self.text, 1.0, geom, ctx);
    }

    fn handle_sdl_event(&mut self, event: &event::Event, geom: &Geometry, self_id: Id, queue: &mut DispatcherQueue) {
        use event::Event::*;
        match event {
            MouseButtonUp {..} => {
                // TODO: only on left click
                queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
            },
            _ => {}
        };
    }
}

use crate::widget_gui::*;
use crate::widget_gui::render;
use crate::text_rendering::text_renderer::TextRenderer;


#[derive(Debug, Clone)]
pub struct TextWidget {
    pub text: String,
    pub scale: f32
}


impl Widget for TextWidget {
    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {

        let text_size = TextRenderer::render_box(ctx.font, &self.text, bc.max_w as f32, self.scale);
        LayoutResult::Size(Size {
            pixel_w: Pixel::min(bc.max_w, Pixel::max(text_size.total_width as i32, bc.min_w)),
            pixel_h: Pixel::min(bc.max_h, Pixel::max(text_size.total_height as i32, bc.min_h))
        })
    }


    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_text(&self.text, self.scale, geom, ctx);
    }


}

use crate::widget_gui::*;
use crate::text_rendering::{text_renderer::TextRenderer, font::Font};
use crate::{gl::{self, viewport}, shader::TransformationShader, objects::square, ScreenBox};


pub struct RenderContext<'a> {
    pub gl: &'a gl::Gl,
    pub tr: &'a mut TextRenderer,
    pub viewport: &'a viewport::Viewport
}


pub fn render_ui(state: &UiState, ctx: &mut RenderContext) {


    // render as a list, not as a tree, might have to change that so parent are rendered before children
    // also in future layers with popup on top of other widgets
    // for now just assume all is in 1 layer and we can render in any order
    for id in 0..state.widgets.len() {
        state.widgets[id].render(&state.geom[id], ctx);
    }


}


pub fn transform_to_screen_space(geom: &Geometry, viewport: &viewport::Viewport) -> ScreenBox {
    ScreenBox::new(geom.pos.x as f32,
                   geom.pos.y as f32,
                   geom.size.pixel_w as f32,
                   geom.size.pixel_h as f32,
                   viewport.w as f32,
                   viewport.h as f32)
}



pub fn render_text(text: &str, scale: f32, geom: &Geometry, ctx: &mut RenderContext) {

    let sb = transform_to_screen_space(geom, &ctx.viewport);
    ctx.tr.render_text(ctx.gl, &text,  Default::default(), sb, scale);
}

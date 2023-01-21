use crate::widget_gui::*;
use crate::widget_gui::widgets::*;
use crate::widget_gui::render;


#[derive(Debug, Clone)]
pub struct RowWidget {

}


impl Widget for RowWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        fill_row(bc, children, ctx)
    }

    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_round_rect(geom, ctx);
    }
}


#[derive(Debug, Clone)]
pub struct ColumnWidget {

}

impl Widget for ColumnWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        fill_column(bc, children, ctx)
    }

}

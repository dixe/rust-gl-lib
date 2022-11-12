use crate::widget_gui::*;
use crate::widget_gui::widgets::*;

#[derive(Debug, Clone)]
pub struct RowWidget {

}


impl Widget for RowWidget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        fill_row(bc, children, ctx)
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

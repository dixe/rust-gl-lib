use super::*;

impl<'a> Ui<'a> {
    pub fn label(&mut self, text: &str)
    {
        let scale = 0.7;
        let rb = self.drawer2D.text_render_box(text, scale);

        let mut rect = Rect { x: 10, y: 10, w: rb.total_width as i32, h: rb.total_height as i32 };

        rect = self.layout_rect(rect);

        // draw button

        let text_col = Color::Rgb(0,0,0);

        // TODO: have label color, text size ect in a Style struct on the UI, that we can just read from
        self.drawer2D.render_text_scaled(text, rect.x, rect.y, scale);

    }
}

use super::*;

impl Ui {
    pub fn label(&mut self, text: &str)
    {

        let pxs = self.style.text_styles.body.pixel_size;
        let rb = self.drawer2D.text_render_box(text, pxs);

        let mut rect = Rect { x: 0, y: 0, w: rb.total_width as i32, h: rb.total_height as i32 };

        rect = self.layout_rect(rect);

        // draw button

        let text_col = Color::Rgb(0,0,0);

        // TODO: have label color, text size ect in a Style struct on the UI, that we can just read from
        self.drawer2D.render_text(text, rect.x, rect.y, pxs);

    }
}

use super::*;
use crate::imode_gui::style::TextStyles;

impl Ui {

    pub fn body_text(&mut self, text: &str)
    {
        let pixel_size = self.style.text_styles.body.pixel_size;
        self.text(text, pixel_size, |styles| &styles.body.font_name);
    }

    pub fn small_text(&mut self, text: &str)
    {
        let pixel_size = self.style.text_styles.small.pixel_size;
        self.text(text, pixel_size, |styles| &styles.small.font_name);
    }


    pub fn text(&mut self, text: &str, pixel_size: i32, font_name_fn: fn(&TextStyles) -> &str) {

        let font_name = font_name_fn(&self.style.text_styles);

        let rb = self.drawer2D.text_render_box_with_font_name(text, pixel_size, font_name);

        let mut rect = Rect { x: 0, y: 0, w: rb.total_width as i32, h: rb.total_height as i32 };

        rect = self.layout_rect(rect);

        // Required to call again, with let, so we overwrite the old, and does not cause lifetime issues with
        let font_name = font_name_fn(&self.style.text_styles);
        self.drawer2D.render_text_from_font_name(text, rect.x, rect.y, pixel_size, font_name);
    }
}

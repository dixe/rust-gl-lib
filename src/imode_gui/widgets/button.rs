use super::*;

impl<'a> Ui<'a> {
    pub fn button(&mut self, text: &str) -> bool {

        // figure out button layout


        let id = self.next_id();
        let mut res = false;

        let scale = 0.6;
        let rb = self.drawer2D.text_render_box(text, scale);
        let mut rect = Rect {
            x: 10, y: 10, w: rb.total_width  as i32 + 3, h: rb.total_height as i32
        };


        rect = self.layout_rect(rect);

        // Calc state with hot and active
        // Hot is hover, active is we pressed down

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_active(id) {
            if self.mouse_up {
                if self.is_hot(id) {
                    res = true;
                }
                self.set_not_active();

            }
        }
        else if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }


        // draw button

        let mut color = Color::Rgb(109, 156, 116);
        let mut text_color = Color::Rgb(0,0,0);
        if self.is_hot(id) {
            color = Color::Rgb(111, 135, 114);
        }

        if self.is_active(id) {
            color = Color::Rgb(114, 214, 126);
        }

        let x_off = if self.is_active(id) {0} else {0};
        let y_off = if self.is_active(id) {0} else {0};

        if !self.is_active(id) {
            self.drawer2D.rounded_rect_color(rect.x - 1, rect.y - 1 , rect.w + 2, rect.h + 2, Color::Rgb(0,0,0));
        }


        self.drawer2D.rounded_rect_color(rect.x + x_off , rect.y + y_off, rect.w, rect.h, color);

        self.drawer2D.render_text_scaled(text, rect.x, rect.y, scale);

        // done

        return res;
    }

}

use super::*;

pub enum ButtonContent<'a> {
    Text(&'a str),
    None,
    //Img
}

impl Ui{


    pub fn button_at_empty(&mut self, rect: Rect) -> bool {
        self.button_base(ButtonContent::None, rect)
    }

    pub fn button_at_text(&mut self, text: &str, x: i32, y: i32) -> bool {

        let pxs = self.style.text_styles.button.pixel_size;
        let rb = self.drawer2D.text_render_box(text, pxs);

        let pad_r = self.style.padding.right;
        let pad_l = self.style.padding.left;
        let pad_b = self.style.padding.bottom;
        let pad_t = self.style.padding.top;

        // border box, with space for padding for text content
        let mut rect = Rect {
            x: x,
            y: y,
            w: rb.total_width as i32 + pad_l + pad_r,
            h: rb.total_height as i32  + pad_t + pad_b
        };

        self.button_base(ButtonContent::Text(text), rect)
    }

    pub fn button(&mut self, text: &str) -> bool {

        let pxs = self.style.text_styles.button.pixel_size;
        let rb = self.drawer2D.text_render_box(text, pxs);


        let pad_r = self.style.padding.right;
        let pad_l = self.style.padding.left;
        let pad_b = self.style.padding.bottom;
        let pad_t = self.style.padding.top;

        // border box, with space for padding for text content
        let mut rect = Rect {
            x: 0,
            y: 0,
            w: rb.total_width as i32 + pad_l + pad_r,
            h: rb.total_height as i32  + pad_t + pad_b
        };

        rect = self.layout_rect(rect);
        self.button_base(ButtonContent::Text(text), rect)
    }

    fn button_base(&mut self, content: ButtonContent, rect: Rect) -> bool {

        // figure out button layout

        let id = self.next_id();
        let mut res = false;

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

        let mut color = self.style.button.color;
        let mut text_color = self.style.button.text_color;
        if self.is_hot(id) {
            color = self.style.button.hover_color;
        }

        if self.is_active(id) {
            color = self.style.button.active_color;
        }

        let x_off = if self.is_active(id) {0} else {0};
        let y_off = if self.is_active(id) {0} else {0};


        // black border
        if !self.is_active(id) {
            let z = self.drawer2D.z;
            self.drawer2D.z = z - 0.1;
            self.drawer2D.rounded_rect_color(rect.x - 1, rect.y - 1 , rect.w + 2, rect.h + 2, Color::Rgb(0,0,0));
            self.drawer2D.z = z;
        }



        let pad_l = self.style.padding.left;
        let pad_t = self.style.padding.top;

        self.drawer2D.rounded_rect_color(rect.x + x_off , rect.y + y_off, rect.w, rect.h, color);

        match content {
            ButtonContent::Text(text) => {
                let pxs = self.style.text_styles.button.pixel_size;
                let font_name = &self.style.text_styles.button.font_name;
                self.drawer2D.render_text_from_font_name(text, rect.x + pad_l, rect.y + pad_t, pxs, font_name);
            },
            ButtonContent::None => {

            }
        };

        // done

        return res;
    }

}

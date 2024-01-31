 use super::*;

impl Ui{

    pub fn pixel_grid_at_empty(&mut self, rect: Rect) -> bool {
        self.pixel_grid_base(PixelGridContent::None, rect)
    }


    pub fn pixel_grid_at_text_fixed(&mut self, text: &str, rect: Rect) -> bool {
        // TODO check that text is inside rect
        self.pixel_grid_base(PixelGridContent::Text(text), rect)

    }

    pub fn pixel_grid_at_text(&mut self, text: &str, x: i32, y: i32) -> bool {

        let pxs = self.style.text_styles.pixel_grid.pixel_size;
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

        self.pixel_grid_base(PixelGridContent::Text(text), rect)
    }

    pub fn pixel_grid(&mut self, text: &str) -> bool {

        let pxs = self.style.text_styles.pixel_grid.pixel_size;
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
        self.pixel_grid_base(PixelGridContent::Text(text), rect)
    }

    fn pixel_grid_base(&mut self, content: PixelGridContent, rect: Rect) -> bool {

        // figure out pixel_grid layout

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


        // draw pixel_grid

        let mut color = self.style.pixel_grid.color;
        let mut text_color = self.style.pixel_grid.text_color;

        let r = self.style.pixel_grid.radius.get(rect);

        if self.is_active(id) {
            color = self.style.pixel_grid.active_color;
        }

        // outline
        if self.is_hot(id) {
            self.drawer2D.rounded_rect_color(rect.x, rect.y , rect.w, rect.h, r, self.style.pixel_grid.hover_color);

        }

        let pad_l = self.style.padding.left;
        let pad_t = self.style.padding.top;

        // background
        self.drawer2D.rounded_rect_color(rect.x + 1 , rect.y + 1, rect.w - 2, rect.h - 2, r,  color);


        // done

        return res;
    }

}

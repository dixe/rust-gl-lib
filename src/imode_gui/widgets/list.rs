use std::fmt::Display;
use super::*;


impl Ui {
    /// Display a list of items, with drag and drop support for rearanging items in the list
    /// returns the index of the currently dragged item
    pub fn list_horizontal<T: Display>(&mut self, items: &mut Vec::<T>) -> Option<usize> {

        let id = self.next_id();

        let len = items.len() as i32;

        let scale = self.style.text_styles.button.text_scale;
        let text = format!("{}", items[0]);
        let rb = self.drawer2D.text_render_box(&text, scale);

        let pad_r = self.style.padding.right;
        let pad_l = self.style.padding.left;
        let pad_b = self.style.padding.bottom;
        let pad_t = self.style.padding.top;

        // border box, with space for padding for text content

        // space between items
        let spacing_x = self.style.spacing.x;

        let elm_w = f32::max(rb.total_height, rb.total_width) as i32;
        let w = pad_l + pad_r +  (spacing_x * len - 1)  + elm_w * len;

        let mut rect = Rect {
            x: 0,
            y: 0,
            w: w,
            h: rb.total_height as i32
        };

        rect = self.layout_rect(rect);


        // Calc state with hot and active
        // Hot is hover, active is we pressed down
        let mut active_item_index = items.len() + 1;

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }
        if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }



        let mut positions = vec![];

        // calc positions and register item being dragged
        let mut x_off = 0;
        let mut has_floating = None;
        for i in 0..items.len() {

            let mut x = rect.x + x_off;
            let mut y = rect.y;

            let mut is_active_item = false;

            let h = rb.total_height as i32;
            let r = Rect { x, y, w: elm_w, h };
            if self.is_active(id) {
                if self.mouse_down_in_rect(&r) {
                    is_active_item = true;
                }
            } else if self.is_hot(id) {
                if self.mouse_in_rect(&r) {
                    is_active_item = true;
                }
            }

            if is_active_item && self.is_active(id) {
                x = self.mouse_pos.x;
                y = self.mouse_pos.y;
                has_floating = Some(i);
            }

            positions.push(ElmPos {
                x,
                y,
                active: is_active_item,
                idx: i
            });

            x_off += elm_w + spacing_x;
        }



        if self.is_active(id) {
            if self.mouse_up {

                if let Some(idx) = has_floating {
                    let mut insert_idx = 0;
                    // calc the pos where we need to insert
                    for i in 0..positions.len() {
                        if positions[idx].x > positions[i].x {
                            insert_idx = i;
                        }
                    }

                    if idx != insert_idx {
                        let item = items.remove(idx);
                        items.insert(insert_idx, item);
                    }
                }
                self.set_not_active();
            }
        }


        // draw items
        for i in 0..items.len() {
            let mut color = self.style.button.color;

            if positions[i].active {
                color = self.style.button.hover_color;
            }

            let mut x = positions[i].x;
            let y = positions[i].y;

            if let Some(idx) = has_floating {
                // push items to the left
                if idx < i && positions[idx].x > positions[i].x {
                    x -= elm_w + spacing_x;
                }
                // push items right
                if idx > i && positions[idx].x < (positions[i].x + elm_w) {
                    x += elm_w + spacing_x;
                }
            }

            self.drawer2D.rounded_rect_color(x, y, elm_w, rb.total_height as i32, color);
            self.drawer2D.render_text_scaled(&format!("{}", items[i]), x, y, scale);

        }

        has_floating
    }
}


struct ElmPos {
    idx: usize,
    active: bool,
    x: i32,
    y: i32
}

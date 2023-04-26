use super::*;
use core::ops::Range;

impl<'a> Ui<'a> {

    pub fn slider<T>(&mut self, item: &mut T, min: T, max: T) where T : Numeric {


        // figure out button layout

        let id = self.next_id();
        let mut res = false;

        let mut rect = Rect {
            x: 10, y: 10, w:100, h: 20
        };

        rect = self.layout_rect(rect);

        let knob_width = rect.h - 2;


        // state mangement

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        let min_f64 = min.to_f64();
        let max_f64 = max.to_f64();
        let item_f64 = item.to_f64();

        if self.is_active(id) {

            let mut v = (self.mouse_pos.x - rect.x) as f64 / rect.w as f64 * max_f64;

            v = f64::max(min_f64, f64::min(max_f64, v));
            *item = T::from_f64(v);
            if self.mouse_up {
                self.set_not_active();
            }
        }



        // Draw

        let min_f64 = min.to_f64();
        let max_f64 = max.to_f64();
        let item_f64 = item.to_f64();

        let item_in_range = f64::min(f64::max(min_f64, item_f64), max_f64);

        let x_f64 = ((item_in_range - min_f64) / (max_f64 - min_f64)) * ((rect.w - knob_width) as f64);

        let x = x_f64.round() as i32;

        // Slider background
        let mut bg_color = Color::RgbA(40, 98, 118, 128);

        self.drawer2D.rounded_rect_color(rect.x, rect.y + rect.h/ 2 - 1,  rect.w, 2, bg_color);


        // slider knob
        let mut color = Color::RgbA(49, 172, 181, 128);
        if self.is_hot(id) {
            color = Color::RgbA(49, 130, 100, 128);
        }
        self.drawer2D.rounded_rect_color(rect.x + x, rect.y, knob_width, rect.h , color);
    }
}

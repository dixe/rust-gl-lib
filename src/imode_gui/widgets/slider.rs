use super::*;
use crate::imode_gui::numeric::Numeric;
use core::ops::Range;

impl Ui {

    pub fn slider<T>(&mut self, item: &mut T, min: T, max: T) where T : Numeric {

        // figure out button layout

        let id = self.next_id();

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


    fn manage_state(&mut self, rect: &Rect, id: u64) -> f64 {

        let mut res = 0.0;
        // state mangement
        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        if self.is_active(id) {
            res = self.mouse_diff.x as f64 / 500.0;

            if self.mouse_up {
                self.set_not_active();
            }
        }

        res
    }

    pub fn combo_box<T>(&mut self, item: &mut T, min: T, max: T) where T : Numeric + std::fmt::Display {

        let id = self.next_id();


        let pixel_size = self.style.text_styles.small.pixel_size;
        let font_name = &self.style.text_styles.small.font_name;
        let text_box = self.drawer2D.text_render_box_with_font_name(&format!("{max:.2}"), pixel_size, font_name);

        let rect_w = text_box.total_width as i32 + self.style.padding.x() * 2;
        let mut rect = Rect {
            x: 0,
            y: 0,
            w: rect_w,
            h: text_box.total_height as i32 + self.style.padding.y()
        };

        rect = self.layout_rect(rect);

        let change = self.manage_state(&rect, id);

        *item = T::from_f64(f64::max(min.to_f64(), f64::min(max.to_f64(), item.to_f64() + change)));

        // Draw box

        self.drawer2D.rounded_rect_color(rect.x, rect.y, rect.w, rect.h, Color::Rgb(200, 179, 171));

        // TODO: maybe just use text renderercenter alignemnt
        let x = rect.x + self.style.padding.x();
        let y = rect.y;

        let font_name = &self.style.text_styles.small.font_name;
        self.drawer2D.render_text_from_font_name(&format!("{item:.2}"), x, y, pixel_size, font_name);

    }
}

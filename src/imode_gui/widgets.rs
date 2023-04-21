use super::*;
use core::ops::Range;

impl<'a> Ui<'a> {
    pub fn button(&mut self, text: &str) -> bool {

        // figure out button layout

        let id = self.next_id();
        let mut res = false;

        let mut rect = Rect {
            x: 10, y: 0, w:30, h: 30
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

        // done

        return res;
    }



    fn to_range(&self, val: i32, rect: &Rect, knob_width: i32, min: i32, max: i32) -> i32 {

        let item_in_range = i32::min(i32::max(min, val), max) as f64;
        (((item_in_range - min as f64) / (max as f64)) * (rect.w as f64 - knob_width as f64)) as i32
    }


    pub fn slider(&mut self, item: &mut i32, min: i32, max: i32) {


        // figure out button layout

        let id = self.next_id();
        let mut res = false;

        let mut rect = Rect {
            x: 00, y: 00, w:100, h: 20
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

        if self.is_active(id) {

            let mut v = ((self.mouse_pos.x - rect.x) as f64 / rect.w as f64 * max as f64) as i32;

            v = i32::max(min, i32::min(max, v));
            *item = v;
            if self.mouse_up {
                self.set_not_active();
            }
        }


        // Draw

        let item_in_range = i32::min(i32::max(min, *item), max) as f64;

        let x = (((item_in_range - min as f64) / (max as f64)) * (rect.w as f64 - knob_width as f64)) as i32;

        // Slider background
        let mut bg_color = Color::RgbA(40, 98, 118, 128);
        self.drawer2D.rounded_rect_color(rect.x, rect.y, rect.w, rect.h, bg_color);


        // slider knob
        let mut color = Color::RgbA(49, 172, 181, 128);
        self.drawer2D.rounded_rect_color(rect.x + x, rect.y + 1, knob_width, rect.h - 2, color);


    }

}

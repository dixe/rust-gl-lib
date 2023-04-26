use super::*;

impl<'a> Ui<'a> {
    pub fn checkbox(&mut self, val: &mut bool) {
        let id = self.next_id();

        let mut rect = Rect {
            x: 10, y: 10, w: 20, h: 20
        };

        rect = self.layout_rect(rect);

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_active(id) {
            if self.mouse_up {
                if self.is_hot(id) {
                    *val = !*val;
                }
                self.set_not_active();

            }
        }
        else if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        // Draw it
        let mut color = Color::Rgb(200, 200, 200);
        //
        // bg color
        self.drawer2D.rounded_rect_color(rect.x , rect.y, rect.w, rect.h, color);

        color = Color::Rgb(150, 150, 150);

        if self.is_hot(id) {
            color = Color::Rgb(100, 100, 100);
        }

        if self.is_active(id) {
            color = Color::Rgb(40, 40, 40);
        }

        if *val {
            color = Color::Rgb(5, 5, 5);
        }

        self.drawer2D.rounded_rect_color(rect.x + 2 , rect.y + 2, rect.w -4 , rect.h -4, color);




    }
}

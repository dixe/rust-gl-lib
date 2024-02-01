use super::*;

impl Ui {

    pub fn checkbox(&mut self, checked: &mut bool) -> bool {
        let id = self.next_id();
        let mut changed = false;

        let mut rect = Rect {
            x: 0 , y: 0 , w: 20, h: 20
        };

        rect = self.layout_rect(rect);

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_active(id) {
            if self.mouse_up {
                if self.is_hot(id) {
                    *checked = !*checked;
                    changed = true;
                }
                self.set_not_active();

            }
        }
        else if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        // Draw it, same style as button
        let r = self.style.button.radius.get(rect);
        let mut color = self.style.button.color;
        let mut thickness = 1.3;


        if self.is_active(id) {
            color = self.style.button.active_color;
            // also update outline maybe
            thickness = 1.6;
        }

        // outline
        if self.is_hot(id) {
            self.drawer2D.rounded_rect_color(rect.x, rect.y , rect.w, rect.h, r, self.style.button.hover_color);
        }

        // background
        self.drawer2D.rounded_rect_color(rect.x + 1 , rect.y + 1, rect.w - 2, rect.h - 2, r,  color);

        // checkmark
        if *checked {
            color = Color::Rgb(5, 5, 5);
            //self.drawer2D.rounded_rect_color(rect.x + 2 , rect.y + 2, rect.w -4 , rect.h -4, rect.h / 2, color);


            // maybe try to do a check with two line
            self.drawer2D.line(rect.x + 5 , rect.y + 10, rect.x + 10, rect.y + 16, thickness);

            self.drawer2D.line(rect.x + 10, rect.y + 16, rect.x + 16 , rect.y + 4, thickness);
        }

        changed
    }
}

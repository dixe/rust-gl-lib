use super::*;

impl Ui {

    pub fn drag_point(&mut self, pos: &mut Pos) {

        // figure out button layout
        let id = self.next_id();


        let center = na::Vector2::new(pos.x as f64, pos.y as f64);
        let mp = na::Vector2::new(self.mouse_pos.x as f64, self.mouse_pos.y as f64);


        let size = 10.0;
        let in_rect = (center - mp).magnitude() < size;

        // state mangement
        if in_rect {
            self.set_hot(id);
        }

        if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        if self.is_active(id) {
            *pos = self.mouse_pos;
            if self.mouse_up {
                self.set_not_active();
            }
        }

        // Draw
        self.drawer2D.circle(pos.x, pos.y, size as i32);
    }
}

use super::*;

impl Ui {

    /// returns true when mouse down on drag point, so when activated.
    pub fn drag_point(&mut self, pos: &mut Pos, r: f64) -> bool {

        let (_, res) = self.drag_point_no_draw(pos, r);

        // Draw
        self.drawer2D.circle(pos.x, pos.y, r as i32,self.style.button.color);
        res
    }

    pub fn drag_point_txt(&mut self, pos: &mut Pos, txt: &str) -> bool {
        // TODO: use ui style for text scale
        let pxs = 16;
        let text_box = self.drawer2D.text_render_box(txt, pxs);

        let r = text_box.total_width.min(text_box.total_height) as f64;

        let (status, activated) = self.drag_point_no_draw(pos, r);

        let color = match status {
            WidgetStatus::Inactive => Color::Rgb(200,10, 200),
            WidgetStatus::Hot => Color::Rgb(10, 200, 200),
            WidgetStatus::Active => Color::Rgb(200, 200, 10),
        };

        self.drawer2D.circle(pos.x, pos.y, r as i32, color);

        self.drawer2D.render_text(txt, pos.x - (r/2.0) as i32, pos.y - r as i32, pxs);

        activated

    }


    pub fn drag_point_no_draw(&mut self, pos: &mut Pos, r: f64) -> (WidgetStatus, bool) {

        // figure out button layout
        let id = self.next_id();

        let center = na::Vector2::new(pos.x as f64, pos.y as f64);
        let mp = na::Vector2::new(self.mouse_pos.x as f64, self.mouse_pos.y as f64);

        let in_rect = (center - mp).magnitude() < r;

        let mut status = WidgetStatus::Inactive;
        // state mangement
        if in_rect {
            self.set_hot(id);
            status = WidgetStatus::Hot;
        }

        let mut activated = false;

        if self.is_hot(id) {
            if self.mouse_down {
                activated = true;
                self.set_active(id);
            }
        }

        if self.is_active(id) {
            *pos = self.mouse_pos;
            status = WidgetStatus::Active;
            if self.mouse_up {
                self.set_not_active();
            }
        }

        (status, activated)
    }
}

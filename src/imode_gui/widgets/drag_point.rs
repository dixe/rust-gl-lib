use crate::math::*;
use crate::math::numeric::Numeric;
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

    pub fn angle_drag_point<T1, T2, T3, T4>(&mut self, center_t: &na::Vector2::<T1>, angle_t: &mut T2, r: T3, thickness: T4)
    where T1: Numeric + std::fmt::Debug, // debug required for .x and .y to worko
          T2: Numeric,
          T3: Numeric,
          T4: Numeric
    {

        let id = self.next_id();

        let center = na::Vector2::new(center_t.x.to_f64(), center_t.y.to_f64());

        let dist = 40.0;

        let angle = std::f64::consts::PI/2.0 + angle_t.to_f64();
        let dir = na::Vector2::new(angle.cos(), -angle.sin());


        let pos = (center.v2f64() + dir * dist).v2i();

        let r_inner = dist - thickness.to_f64();


        // outline
        let color = Color::Rgb(0, 0, 0);
        self.drawer2D.circle_outline(center.x, center.y, dist, r_inner, color);

        // drag point
        self.drawer2D.circle(pos.x, pos.y, r, self.style.button.color);

    }
}

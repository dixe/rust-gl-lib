use super::*;
use crate::collision2d::polygon::{Polygon};
use crate::collision2d::gjk::Shape;

type V2 = na::Vector2::<f32>;

impl Ui {

    /// draw a polygon mapping vertecies to screen coordinates
    /// returns true if dragpoint was activated
    pub fn view_raw_polygon(&mut self, polygon: &mut Polygon, draggable: bool, show_idx: bool, show_pos: bool, color: Color) -> bool {

        let len = polygon.vertices.len();

        let mut offset = na::Vector2::new(0, 0);

        let mut res = false;
        if draggable {
            (res, offset) = self.drag(polygon);
        }

        for i in 0..len {
            let v = &mut polygon.vertices[i];

            if show_pos {
                self.drawer2D.render_text(&format!("({:?})", v), v.x as i32, v.y as i32 + 20, 14);
            }

            if show_idx {
                self.drawer2D.render_text(&format!("{i}"), v.x as i32, v.y as i32, 20);
            }


            v.x += offset.x as f32;
            v.y += offset.y as f32;

            let mut r = 8.0;

            self.drawer2D.circle(v.x, v.y, r, color);

            if i < len - 1 {
                let p1 = polygon.vertices[i];
                let p2 = polygon.vertices[i + 1];
                self.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
            }


        }

        if len > 2 {
            let p1 = polygon.vertices[len - 1];
            let p2 = polygon.vertices[0];
            self.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
        }
        res

    }


    /// edit polygon mapping vertices to screen cooridinates
    pub fn edit_raw_polygon(&mut self, polygon: &mut Polygon, show_idx: bool, show_pos: bool) {

        let (res, offset) = self.drag(polygon);
        let len = polygon.vertices.len();

        for i in 0..len {
            let v = &mut polygon.vertices[i];

            if show_pos {
                self.drawer2D.render_text(&format!("({:?})", &v), v.x as i32, v.y as i32 + 20, 14);
            }

            if show_idx {
                self.drawer2D.render_text(&format!("{i}"), v.x as i32, v.y as i32, 20);
            }

            let mut r = 8.0;

            let mut drag  = na::Vector2::new(v.x as i32, v.y as i32);
            self.drag_point(&mut drag, 8.0);

            // not adding as i32, will accumulat the float error/difference and make whole polygon flaot
            v.x = (drag.x + offset.x) as f32;
            v.y = (drag.y + offset.y) as f32;


            if i < len - 1 {
                let p1 = polygon.vertices[i];
                let p2 = polygon.vertices[i + 1];
                self.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
            }


        }

        if len > 2 {
            let p1 = polygon.vertices[len - 1];
            let p2 = polygon.vertices[0];
            self.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 3.0);
        }

        render_intersect(self, polygon);
    }


    fn drag(&mut self, polygon: &mut Polygon) -> (bool, na::Vector2::<i32>) {
        let mut offset = na::Vector2::new(0, 0);

        if polygon.vertices.len() == 0 {
            return (false, offset);
        }

        let center = polygon.center();
        let mut drag = na::Vector2::new(center.x as i32, center.y as i32);
        let res = self.drag_point(&mut drag, 15.0);
        drag.x -= center.x as i32;
        drag.y -= center.y as i32;

        (res, drag)

    }
}

fn render_intersect(ui: &mut Ui, polygon: &Polygon) {
    // go over all side by side pairs and compare with every other side by side pair

    if polygon.vertices.len() <= 3 {
        return;
    }

    for p in polygon.intersections() {
        ui.drawer2D.circle(p.x, p.y, 7.0, Color::Rgb(250, 5, 5));
    }

}

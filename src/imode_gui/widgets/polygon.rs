use super::*;
use crate::collision2d::polygon::{Polygon};
use crate::collision2d::gjk::Shape;

type V2 = na::Vector2::<f32>;

impl Ui {

    /// draw a polygon such that the center is drawn at center parameter
    /// returns true if dragpoint was activated
    pub fn view_polygon(&mut self, polygon: &Polygon, center: &mut V2, color: Color) -> bool {

        let id = self.next_id();

        let len = polygon.vertices.len();


        let mut drag  = na::Vector2::new(center.x as i32, center.y as i32);
        let res = self.drag_point(&mut drag, 15.0);

        center.x = drag.x as f32;
        center.y = drag.y as f32;

        let poly_center : V2 = polygon.center();

        for i in 0..len {
            let v: &V2 = &polygon.vertices[i];

            let p = v - poly_center + *center;

            let mut r = 8.0;

            self.drawer2D.circle(p.x, p.y, r, color);

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
}

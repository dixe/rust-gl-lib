use super::*;
use crate::collision2d::polygon::{Polygon, PolygonTransform};
use crate::collision2d::gjk::Shape;
use std::collections::HashSet;


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
    /// returns true when interacted with
    pub fn edit_raw_polygon(&mut self, polygon: &mut Polygon, show_idx: bool, show_pos: bool, selected: &mut Option<usize>) -> Option<Interaction> {

        let (res, offset) = self.drag(polygon);
        let mut len = polygon.vertices.len();

        let mut interaction = None;
        if res {
            interaction = Some(Interaction::Center);
        }

        if self.ctrl {
            use event::Event::*;
            use sdl2::keyboard::Keycode;

            for e in &self.frame_events {
                match e {
                    KeyUp { keycode: Some(Keycode::Z), ..} => {
                        if len > 0 {
                            polygon.vertices.pop();
                            len = polygon.vertices.len();
                            // clear selected if len < current selected
                        }
                    },

                    KeyUp { keycode: Some(Keycode::A), keymod, ..} => {
                        // add before selected (left)
                        if let Some(i) = selected {

                            let before = polygon.vertices[(len + *i - 1) % len];
                            let v = polygon.vertices[*i];
                            let new_v = (before + v)/2.0;

                            polygon.vertices.insert(*i, new_v);

                            // selected is +1
                            *selected = Some(*i + 1);

                        }
                    },

                    KeyUp { keycode: Some(Keycode::D), keymod, ..} => {
                        // add after selected (left)
                        if let Some(i) = selected {

                            let v = polygon.vertices[*i];
                            let after = polygon.vertices[(*i + 1) % len];
                            let new_v = (after + v)/2.0;

                            polygon.vertices.insert(*i + 1, new_v);


                        }
                    },
                    _ => {},
                }
            }

            // assume always "active" when edit raw, regarding new vertices
            if self.mouse_up {
                let new = V2::new(self.mouse_pos.x as f32, self.mouse_pos.y as f32);
                if len == 0 {
                    polygon.vertices.push(new);
                } else if let Some(i) = selected  {
                    polygon.vertices.insert(*i, new);
                } else {
                    polygon.vertices.push(new);
                }
            }
        }



        for i in 0..len {
            let v = &mut polygon.vertices[i];

            if show_pos {
                self.drawer2D.render_text(&format!("({:?})", &v), v.x as i32, v.y as i32 + 20, 14);
            }

            if show_idx {
                self.drawer2D.render_text(&format!("{i}"), v.x as i32, v.y as i32, 20);
            }

            let mut r = 8.0;

            if let Some(sel) = *selected {
                if i == sel {
                    r += 2.0;
                }
            }

            let mut drag  = na::Vector2::new(v.x as i32, v.y as i32);
            if self.drag_point(&mut drag, r) {
                *selected = Some(i);
                interaction = Some(Interaction::Vertex(i));
            }

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

        interaction
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

    pub fn view_polygon(&mut self, polygon: &Polygon, transform: &PolygonTransform) {

        let len = polygon.vertices.len();
        let color = Color::Rgb(0, 0, 0);
        for i in 0..len {
            let v = transform.map(polygon.vertices[i]);

            let mut r = 1.0;
            self.drawer2D.circle(v.x, v.y, r, color);

            if i < len - 1 {
                let p1 = transform.map(polygon.vertices[i]);
                let p2 = transform.map(polygon.vertices[i + 1]);
                self.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 2.0);
            }
        }

        if len > 2 {
            let p1 = transform.map(polygon.vertices[len - 1]);
            let p2 = transform.map(polygon.vertices[0]);
            self.drawer2D.line(p1.x, p1.y, p2.x, p2.y, 2.0);
        }
    }

    pub fn polygon_editor(&mut self, orig_polygon: &mut Polygon, options: &mut PolygonOptions) {
        let id = self.next_id();

        options.transform_to_screenspace(&orig_polygon);

        let polygon = &mut options.tmp_polygon;

        let active = self.is_active(id);

        let interacted = self.edit_raw_polygon(polygon, false, false, &mut options.selected) ;

        if let Some(Interaction::Vertex(i)) = interacted {
            options.selected.insert(i);
        }

        render_intersect(self, polygon);

        options.transform_from_screenspace(orig_polygon);

    }
}


#[derive(Default)]
pub struct PolygonOptions {
    selected: Option<usize>,
    pub transform: PolygonTransform,
    tmp_polygon: Polygon
}



impl PolygonOptions {

    /// apply transformations to polygon vertices and into screenspace, puts result in tmp polygon
    fn transform_to_screenspace(&mut self, polygon: &Polygon) {
        self.tmp_polygon.vertices.clear();
        for i in 0..polygon.vertices.len() {
            self.tmp_polygon.vertices.push(self.transform.map(polygon.vertices[i]))
        }
    }

    /// apply inverse transformations to tmp_polygon vertices, puts result in polygon
    fn transform_from_screenspace(&self, polygon: &mut Polygon) {
        polygon.vertices.clear();

        for i in 0..self.tmp_polygon.vertices.len() {
            let v = self.transform.inverse_map(self.tmp_polygon.vertices[i]);
            polygon.vertices.push(v);
        }
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


#[derive(Debug, Clone, Copy)]
pub enum Interaction {
    Center,
    Vertex(usize)
}

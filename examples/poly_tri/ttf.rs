use ttf_parser;
use crate::triangulate::*;
use gl_lib::na;

/// Returns a list of polygons,
pub fn char_to_poly(chr: char, face: &ttf_parser::Face, screen_w: f32, screen_h: f32, samples: i32) -> Vec::<Polygon> {


    let g_id = face.glyph_index(chr).unwrap();

    let mut builder = ConturBuilder(vec![]);
    let _bbox = face.outline_glyph(g_id, &mut builder).unwrap();

    let scale = 0.4;
    let mut res = vec![];
    let mut cur_p = Point::new(0.0, 0.0);

    let mut current_poly = vec![];
    let x_offset = screen_w / 2.0;
    let y_offset = screen_h / 2.0;
    for segment in &builder.0 {
        match segment {
            Segment::Start(p) => {
                add_scaled(p, &mut current_poly, scale, x_offset, y_offset);
                cur_p = *p;
            },
            Segment::LineTo(p) => {
                add_scaled(p, &mut current_poly, scale, x_offset, y_offset);
                cur_p = *p;
            },
            Segment::Curve(p0, _p1, p2) => {

                add_scaled(p0, &mut current_poly, scale, x_offset, y_offset);
                for _i in 1..=samples {
                    todo!("Do correct correct CUBE BEZIER (Curve)");
                }

                cur_p = *p2;

            },
            Segment::Quad(p1, p2) => {
                // Start and endpoint should not be included since they are already added
                for i in 1..samples {
                    let t = i as f32 / samples as f32;

                    let p = p1 + (1.0 - t)*(1.0 -t) * (cur_p - p1) + (t * t) * (p2 - p1);
                    add_scaled(&p, &mut current_poly, scale, x_offset, y_offset);
                }
                add_scaled(p2, &mut current_poly, scale, x_offset, y_offset);
                cur_p = *p2;
            },
            Segment::End => {
                current_poly.pop();
                res.push(current_poly.clone());
                current_poly = vec![];
            },
        }
    }

    res
}


fn add_scaled(p: &Point, res: &mut Polygon, scale: f32, x_offset: f32, y_offset: f32) {
    res.push(na::Vector2::new((p.x + x_offset) * scale , (p.y + y_offset) * scale));
}

pub type Point = na::Vector2<f32>;

#[derive(Debug, Clone, Copy)]
enum Segment {
    Start(Point),
    LineTo(Point),
    Curve(Point, Point, Point),
    Quad(Point, Point),
    End,
}

struct ConturBuilder(Vec<Segment>);


impl ttf_parser::OutlineBuilder for ConturBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.push(Segment::Start(Point::new( x, y)));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.push(Segment::LineTo(Point::new( x, y)));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0
            .push(Segment::Quad(Point::new(x1, y1), Point::new(x, y)));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0
            .push(Segment::Curve(Point::new(x1, y1), Point::new(x2, y2), Point::new(x, y)));
    }

    fn close(&mut self) {
        self.0.push(Segment::End);
    }

}

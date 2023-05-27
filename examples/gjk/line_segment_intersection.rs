use super::{V2, Polygon};

impl Polygon {

    pub fn intersections(&self) -> IntersectIter {

        IntersectIter {
            polygon: self,
            i: 0,
            j: 2,
        }
    }
}

pub struct IntersectIter<'a> {
    polygon: &'a Polygon,
    i: usize,
    j: usize
}

impl<'a> Iterator for IntersectIter<'a> {

    type Item = V2;

    fn next(&mut self) -> Option<Self::Item> {
        let mut first = true;
        let len = self.polygon.vertices.len();
        for i in self.i..(len - 1) {
            let a = self.polygon.vertices[i];
            let b = self.polygon.vertices[i + 1];

            let mut j_start = i + 2;
            if first {
                j_start = self.j;
            }

            for j in j_start..len {

                let c = self.polygon.vertices[j % len];
                let d = self.polygon.vertices[(j + 1) % len];

                if let Some(p) = line_segment_intersect(a,b,c,d) {
                    self.i = i;
                    self.j = j + 1;
                    return Some(p);
                }
            }
        }

        None
    }
}


pub fn line_segment_intersect(a: V2, b: V2, c: V2, d: V2) -> Option<V2> {
    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect

    // line 1 is a to b, or a + g e
    // line 2 is c to d or c + h f

    let e = b - a;
    let f = d - c;
    let p_h = V2::new(-e.y, e.x);
    let p_g = V2::new(-f.y, f.x);

    // check if lines are parallel
    if f.dot(&p_h) == 0.0 {
        return None;
    }

    let h = (a-c).dot(&p_h) / f.dot(&p_h);
    let g = (c-a).dot(&p_g) / e.dot(&p_g);

    if h > 0.0 && h < 1.0 && g > 0.0 && g < 1.0 {
        return Some(c + f * h);
    }

    None
}


#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub a: V2,
    pub dir: V2
}


/// Return the points where the line intersect the line segment, if any, can be anywhere on line
/// but has to be between c and d on line segment
pub fn line_line_segment_intersect(line: Line, c: V2, d: V2) -> Option<V2> {
    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect

    // line 1 is a to b, or a + g e
    // line 2 is c to d or c + h f

    let e = line.dir;
    let f = d - c;
    let p_h = V2::new(-e.y, e.x);


    // check if lines are parallel
    if f.dot(&p_h) == 0.0 {
        return None;
    }

    let h = (line.a-c).dot(&p_h) / f.dot(&p_h);

    if h > 0.0 && h < 1.0 {
        return Some(c + f * h);
    }

    None
}

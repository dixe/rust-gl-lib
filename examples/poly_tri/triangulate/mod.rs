use gl_lib::na;


mod ear_clipping;
pub use self::ear_clipping::*;
mod node_list;

pub type Point = na::Vector2<f32>;


pub type Polygon = Vec<Point>;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangulation {
    pub triangles: Vec<Triangle>,
    pub polygon: Polygon,
    pub dir: Direction
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left
}


/// Indexes into a points vec
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub p0: usize,
    pub p1: usize,
    pub p2: usize,
}


trait ToVec3 {
    fn to_vec3(&self) -> na::Vector3::<f32>;
}

impl ToVec3 for Point {
    fn to_vec3(&self) -> na::Vector3::<f32> {
        na::Vector3::<f32>::new(self.x, self.y, 0.0)
    }
}


fn tri(p0: usize, p1: usize, p2: usize) -> Triangle {
    Triangle { p0, p1, p2 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sign {
    None,
    Positive,
    Negative,
}

fn is_convex(poly: &Polygon) -> bool {
    // A convex has 2 sign changes in x and 2 in y. A non convex has more

    let mut x_changes = 0;
    let mut y_changes = 0;

    let mut x_sign = Sign::None;
    let mut y_sign = Sign::None;

    for i in 0..poly.len() {
        let p0 = poly[i % poly.len()];
        let p1 = poly[(i + 1) % poly.len()];

        let v1: na::Vector2<f32> = (p1 - p0).into();

        if v1.x > 0.0 && x_sign != Sign::Positive {
            x_sign = Sign::Positive;
            x_changes += 1;
        }

        if v1.x < 0.0 && x_sign != Sign::Negative {
            x_sign = Sign::Negative;
            x_changes += 1;
        }

        if v1.y > 0.0 && y_sign != Sign::Positive {
            y_sign = Sign::Positive;
            y_changes += 1;
        }

        if v1.y < 0.0 && x_sign != Sign::Negative {
            y_sign = Sign::Negative;
            y_changes += 1;
        }
    }

    x_changes == 2 && y_changes == 2
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_convex_test() {
        let square =  vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![1.0, 1.0],
                vector![1.0, 0.0],
            ];

        let poly = vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![0.0, 2.0],
                vector![2.0, 2.0],
                vector![1.0, 1.0],
                vector![2.0, 0.0],
            ];

        assert_eq!(true, is_convex(&square));
        assert_eq!(false, is_convex(&poly));
    }
}

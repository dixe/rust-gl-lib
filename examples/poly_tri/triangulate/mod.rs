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

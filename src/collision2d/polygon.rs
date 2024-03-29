use nalgebra as na;
use crate::collision2d::lsi;
use crate::collision2d::gjk;
use crate::imode_gui::drawer2d;
use serde::{Serialize, Deserialize};
use crate::math;
use crate::math::Homogeneous;
type V2 = na::Vector2::<f32>;
type V3 = na::Vector3::<f32>;


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub vertices: Vec::<V2>
}

impl Polygon {
    pub fn set_center(&mut self, center: V2) {
        let c = self.center();
        for p in &mut self.vertices {
            *p += -c + center;
        }
    }

    pub fn center(&self) -> V2 {
        let mut c = V2::new(0.0, 0.0);

        for v in &self.vertices {
            c += v;
        }

        c / self.vertices.len() as f32
    }


    pub fn bounding_box_size(&self) -> V2 {

        let mut min_x = self.vertices[0].x;
        let mut min_y = self.vertices[0].y;
        let mut max_x = self.vertices[0].x;
        let mut max_y = self.vertices[0].y;

        for v in &self.vertices {
            min_x = f32::min(min_x, v.x);
            min_y = f32::min(min_y, v.y);

            max_x = f32::max(max_x, v.x);
            max_y = f32::max(max_y, v.y);
        }

        V2::new(max_x - min_x, max_y - min_y)
    }

    pub fn interpolate(target_poly: &mut Polygon, polygon_1: &Polygon, t_1: &PolygonTransform, polygon_2: &Polygon, t_2: &PolygonTransform, t: f32) -> Option<PolygonTransform> {

        let len = polygon_1.vertices.len();
        if len != polygon_2.vertices.len() {
            return None;
        }


        target_poly.vertices.clear();

        for i in 0..len {
            let p1 = t_1.map(polygon_1.vertices[i]);

            let p2 = t_2.map(polygon_2.vertices[i]);
            let p = p1.lerp(&p2, t);
            target_poly.vertices.push(p)
        }

        Some(t_1.lerp(t_2, t))
    }

    pub fn direction(&self) -> Dir {
        direction(&self)
    }

}



#[derive(Default, Clone, Copy, Debug)]
pub struct PolygonTransform {
    pub translation: V2,
    pub rotation: f32,
    pub scale: f32,
    pub flip_y: bool
}

impl PolygonTransform {


    pub fn mat3(&self) -> na::Matrix3::<f32> {
        let mut scale = na::Matrix3::<f32>::identity();
        scale[0] = self.scale;
        scale[4] = self.scale;

        let y_flip = if self.flip_y { -1.0 } else { 1.0 };
        let mut flip = na::Matrix3::identity();
        flip[0] = y_flip;

        let trans: na::Translation2::<f32> = self.translation.into();
        let rot = na::Rotation2::new(self.rotation);

        trans.to_homogeneous() * rot.to_homogeneous() * flip * scale

    }

    pub fn map(&self, mut v: V2) -> V2 {
        v *= self.scale;
        v += self.translation;
        v
    }

    pub fn inverse_map(&self, mut v: V2) -> V2 {
        v -= self.translation;
        v *= 1.0 / self.scale;
        v
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            translation: self.translation.lerp(&other.translation, t),
            rotation: math::lerp(self.rotation, other.rotation, t),
            scale: math::lerp(self.scale, other.scale, t),
            flip_y: other.flip_y // can't realy interpolate flip_y
        }
    }
}





#[derive(Debug)]
pub struct SubPolygon<'a> {
    pub polygon: &'a Polygon,
    pub indices: Vec::<usize>,
}

impl<'a> SubPolygon<'a> {

    /// assume i in [0;len]
    pub fn vertex(&self, i: usize) -> V2 {
        let idx = self.indices[i];
        self.polygon.vertices[idx]
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

}




pub fn calculate_subdivision(polygon: &mut Polygon) -> Vec::<SubPolygon> {
    for _ in polygon.intersections() {
        return vec![]
    }

    let dir = direction(&polygon);
    match dir {
        Dir::Left => {
            polygon.vertices.reverse();
        },
        _ => {
        }
    }

    let _dir = direction(&polygon);

    let mut input_sub_p = SubPolygon {
        polygon,
        indices: vec![]
    };

    for i in 0..polygon.vertices.len() {
        input_sub_p.indices.push(i);
    }

    let mut to_be_checked = vec![input_sub_p];

    let mut res = vec![];
    while let Some(sub_p) = to_be_checked.pop() {
        // search from left to right for the first polygon that does not produce any intersections
        // and does not make a line outside the polygon

        if let Some(wide_idx) = first_wide(&sub_p) {
            let connection = match find_valid_connection(&sub_p, wide_idx) {
                Ok(c) => c,
                Err(e) => {
                    println!("{:?}", e);
                    return vec![sub_p];
                }
            };

            // connection and wide_idx is indices into indices of sub_p
            let(s1,s2) = split_polygon(sub_p, wide_idx, connection);
            to_be_checked.push(s1);
            to_be_checked.push(s2);
        } else {
            res.push(sub_p);
        }
    }

    res
}

fn split_polygon(sub_p: SubPolygon, from: usize, to: usize) -> (SubPolygon, SubPolygon) {

    // connection and wide_idx is indices into indices of sub_p

    // first ploygon is from 0 to min, and from max to max
    // second is from min to max

    let mut s1 = SubPolygon {
        polygon: sub_p.polygon,
        indices: vec![]
    };


    // for s1 take all indices in sub_p.indices from 0..=from then take all in to..sub_p.indices.len
    // for s0sub_p.indices from 0..=from then take all in to..sub_p.indices.len

    let min = from.min(to);
    let max = from.max(to);

    for i in 0..=min {
        s1.indices.push(sub_p.indices[i]);
    }


    for i in max..sub_p.indices.len() {
        s1.indices.push(sub_p.indices[i]);
    }

    let mut s2 = SubPolygon {
        polygon: sub_p.polygon,
        indices: vec![]
    };


    for i in min..=max {
        s2.indices.push(sub_p.indices[i]);
    }

    (s1, s2)
}

fn find_valid_connection(sub_p: &SubPolygon, idx: usize) -> Result::<usize, failure::Error> {
    let cur_p = sub_p.vertex(idx);

    let len = sub_p.len();

    for conn_idx in 0..len {
        let mut valid = true;
        if conn_idx == (len + idx - 1) % len || conn_idx == idx || conn_idx == (idx + 1) % len {
            // skip0 neighbours and same index
            continue;
        }

        let connection_point = sub_p.vertex(conn_idx);

        // now check all lines in polygon
        'inner: for i in 0..len {
            let idx1 = i;
            let idx2 = (i + 1) % len;
            if idx1 == idx || idx1 == conn_idx
                || idx2 == idx || idx2 == conn_idx {
                    continue;
                }
            let c = sub_p.vertex(idx1);
            let d = sub_p.vertex(idx2);

            if let Some(_) = lsi::line_segment_intersect(cur_p, connection_point, c,d) {
                // line conn
                valid = false;
                break 'inner;
            }
        }


        if valid && is_inside(idx, conn_idx, &sub_p) {
            return Ok(conn_idx);
        }
    }

    failure::bail!("Could not find a valid loop connection in loop search")
}


fn is_inside(idx1: usize, idx2: usize, sub_p: &SubPolygon) -> bool {

    let len = sub_p.len();
    let from = sub_p.vertex((len + idx1 - 1) % len);
    let p = sub_p.vertex(idx1);
    let to = sub_p.vertex((idx1 + 1) % len);

    let mut from_dir = from - p;
    from_dir.y *= -1.0; // sdl inverse coords, neeed to be fixed when using atan2
    let mut to_dir = to - p;
    to_dir.y *= -1.0;



    // find angle 1 as a positive angle


    // between -pi and pi has to be in 0 to tau
    let mut v1 = from_dir.y.atan2(from_dir.x);

    if v1 < 0.0 {
        v1 += std::f32::consts::TAU;
    }

    // find angle 2, so that is is create than angle 1, since we are right polygon, and v1 <  v2

    let mut v2 = to_dir.y.atan2(to_dir.x);
    while v2 < v1 {
        v2 += std::f32::consts::TAU;
    }

    // find new_angle, so that is is greater than angle 1

    let mut new_dir = sub_p.vertex(idx2) - sub_p.vertex(idx1);
    new_dir.y *= -1.0;
    let mut new_v = (new_dir.y).atan2(new_dir.x);


    while new_v < v1 {
        new_v += std::f32::consts::TAU;
    }

    new_v < v2
}


fn first_wide(sub_p: &SubPolygon) -> Option<usize> {
    let len = sub_p.indices.len();

    for i in 0..len {
        let before = sub_p.vertex((len + i - 1) % len);
        let pi = sub_p.vertex(i);
        let after = sub_p.vertex((i + 1) % len);

        if is_wide_angle(vec3(before), vec3(pi), vec3(after)) {
            return Some(i);
        }
    }

    None
}

pub fn direction(polygon: &Polygon) -> Dir {

    let _num_wide = 0;

    // assume right, and calc interier and exterier angle.
    // Interier should be smaller, so if not, then we are left

    let mut int_angle = 0.0;
    let mut ext_angle = 0.0;

    for i in 1..polygon.vertices.len() {
        let v1_i = (i + 1) % polygon.vertices.len();
        let v2_i = (i + 2) % polygon.vertices.len();

        let v0 = vec3(polygon.vertices[i]);
        let v1 = vec3(polygon.vertices[v1_i]);
        let v2 = vec3(polygon.vertices[v2_i]);


        let mut from_dir = v0 - v1;
        let mut to_dir = v2 - v1;
        // sdl inverse coords, neeed to be fixed when using atan2
        from_dir.y *= -1.0;
        to_dir.y *= -1.0;

        // angle between from vec and to vec
        let mut a1 = from_dir.y.atan2(from_dir.x);

        if a1 < 0.0 {
            a1 += std::f32::consts::TAU;
        }

        let mut a2 = to_dir.y.atan2(to_dir.x);
        while a2 < a1 {
            a2 += std::f32::consts::TAU;
        }


        let angle = a2 - a1;

        let inv_angle = std::f32::consts::TAU - angle;
        int_angle += angle;
        ext_angle += inv_angle;
    }

    if int_angle < ext_angle {
        return Dir::Right;
    } else {
        return Dir::Left;
    }

}

fn vec3(v: V2) -> V3 {
    V3::new(v.x, v.y, 0.0)
}

// The triangles are always right handed. So when the cross product is below 0 between the two edges the angle is > 180 deg
fn is_wide_angle(v0: na::Vector3::<f32>, v1: na::Vector3::<f32>, v2: na::Vector3::<f32>) -> bool {
    (v1 - v0).cross(&(v2-v1)).z < 0.0
}


#[derive(Debug, PartialEq)]
pub enum Dir {
    Left,
    Right
}

#[derive(Debug)]
pub struct ComplexPolygon<'a> {
    pub polygon: &'a Polygon,
    pub indices: &'a Vec::<usize>,
    pub transform: &'a na::Matrix3::<f32>,
}

impl<'a> ComplexPolygon<'a> {

    fn transformed(&self, idx: usize) -> V2 {
        // No it should not be self.indice[idx],
        let transformed : V3 = self.transform * self.polygon.vertices[idx].homogeneous();

        transformed.xy() / transformed.z
    }
}

impl<'a> gjk::Shape for ComplexPolygon<'a> {

    fn support(&self, d: V2) -> V2 {

        let mut p = self.transformed(0);

        let mut val = p.dot(&d);
        for idx in self.indices {

            let v = self.transformed(*idx);
            let dot_val = v.dot(&d);
            if dot_val > val {
                val = dot_val;
                p = v;
            }
        }
        p
    }


    fn center(&self) -> V2 {
        let mut c = V2::new(0.0, 0.0);

        for idx in self.indices {
            let v = self.transformed(*idx);
            c += v;
        }

        c / self.indices.len() as f32
    }
}

impl<'a> drawer2d::ConvexPolygon for ComplexPolygon<'a> {
    fn set_vertices(&self, buffer: &mut Vec::<f32>, viewport_height: f32, z: f32) {
        for &i in self.indices {
            let v = self.transformed(i);
            buffer.push(v.x);
            buffer.push(viewport_height - v.y);
            buffer.push(z);
        }
    }
}

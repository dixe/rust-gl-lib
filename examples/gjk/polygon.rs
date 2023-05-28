use super::*;

#[derive(Default, Debug)]
pub struct Polygon {
    pub vertices: Vec::<V2>
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
    for p in polygon.intersections() {
        return vec![];
    }

    let dir = direction(&polygon);
    match dir {
        Dir::Left => {
            polygon.vertices.reverse();
        },
        _ => {
        }
    }

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
            let connection = find_valid_connection(&sub_p, wide_idx);

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

fn find_valid_connection(sub_p: &SubPolygon, idx: usize) -> usize {
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
            return conn_idx;
        }
    }

    panic!("Should always find a valid one in loop search");
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

fn wide_indices(polygon: &Polygon) -> Vec::<usize> {
    let mut res = vec![];

    let len = polygon.vertices.len();

    for i in 0..polygon.vertices.len() {
        let before = polygon.vertices[(len + i - 1) % len];
        let pi = polygon.vertices[i];
        let after = polygon.vertices[(i + 1) % len];

        if is_wide_angle(vec3(before), vec3(pi), vec3(after)) {
            res.push(i);
        }
    }

    res

}


fn direction(polygon: &Polygon) -> Dir {

    let mut num_wide = 0;

    // assume right, and if not return left

    for i in 1..polygon.vertices.len() {
        let v1_i = (i + 1) % polygon.vertices.len();
        let v2_i = (i + 2) % polygon.vertices.len();

        let v0 = vec3(polygon.vertices[i]);
        let v1 = vec3(polygon.vertices[v1_i]);
        let v2 = vec3(polygon.vertices[v2_i]);

        if is_wide_angle(v0, v1, v2) {
            num_wide += 1;
        }
    }


    if num_wide > (polygon.vertices.len()  / 2 ) {
        return Dir::Left;
    }

    return Dir::Right;
}

fn vec3(v: V2) -> V3 {
    V3::new(v.x, v.y, 0.0)
}

// The triangles are always right handed. So when the cross product is below 0 between the two edges the angle is > 180 deg
fn is_wide_angle(v0: na::Vector3::<f32>, v1: na::Vector3::<f32>, v2: na::Vector3::<f32>) -> bool {
    (v1 - v0).cross(&(v2-v1)).z < 0.0
}


enum Dir {
    Left,
    Right
}

pub fn test1() -> bool {

    let polygon = Polygon {

            vertices: vec![V2::new(440.0, 217.0),
                           V2::new(647.0, 527.0),
                           V2::new(332.0, 563.0),
                           V2::new(520.0, 382.0),

            ]
    };

    let sub_p =  SubPolygon {
        polygon: &polygon,
        indices: vec![0,1,2,3]
    };

    let res = find_valid_connection(&sub_p, 3);


    res == 1
}

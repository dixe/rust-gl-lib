use gl_lib::na;
use crate::triangulate::*;
use crate::triangulate::node_list::*;

fn find_ears(poly: &NodeList) -> Vec<usize> {
    let points = &poly.points;

    let mut ears = vec![];

    let start = poly.head;
    let mut cur = poly.head;
    for node_i in poly.into_iter() {
        if is_ear_tip(poly, node_i) {
            ears.push(node_i);
        }
    }

    ears
}

fn is_ear_tip(poly: &NodeList, index: usize) -> bool {

    let total = poly.points.len();
    let left = poly.nodes[index].prev;
    let middle = index;
    let right = poly.nodes[index].next;

    let triangle = tri(left, middle, right);

    let points = &poly.points;

    if is_wide_angle(&points[left].to_vec3(), &points[middle].to_vec3(), &points[right].to_vec3()) {
        return false;
    }

    for i in 0..poly.points.len() {

        if i == left || i == middle || i == right {

            continue;
        }

        if is_inside_triangle(&triangle, poly, points[i]) {

            return false;
        }
    }

    true
}

//https://blackpawn.com/texts/pointinpoly/
fn is_inside_triangle(tri: &Triangle, poly: &NodeList, point: Point)  -> bool {

    let p0 = poly.points[tri.p0];
    let p1 = poly.points[tri.p1];
    let p2 = poly.points[tri.p2];

    let v0 = &p0.to_vec3();
    let v1 = &p1.to_vec3();
    let v2 = &p2.to_vec3();

    let p = &point.to_vec3();


    let res = same_side(p, v0, v1, v2) && same_side(p, v1, v2, v0) && same_side(p, v2, v0, v1);

    res
}

// The triangles are always right handed. So when the cross product is above 0 between the two edges the angle is > 180 deg
fn is_wide_angle(v0: &na::Vector3::<f32>, v1: &na::Vector3::<f32>, v2: &na::Vector3::<f32>) -> bool {
    (v1 - v0).cross(&(v2-v1)).z > 0.0
}

fn same_side(point: &na::Vector3::<f32>, ref_point: &na::Vector3::<f32>, a: &na::Vector3::<f32>, b: &na::Vector3::<f32>) -> bool {

    let cross1 = (b-a).cross(&(point - a));
    let cross2 = (b-a).cross(&(ref_point - a));

    cross1.dot(&cross2) >= 0.0

}

// https://www.geometrictools.com/Documentation/TriangulationByEarClipping.pdf
/// Assume right handed polygon
pub fn triangulate_ear_clipping(input_poly: &Polygon) -> Triangulation {

    let mut polygon = input_poly.clone();
    let mut num_wide = 0;

    let mut dir = Direction::Right;
    for i in 1..polygon.len() {

        let v1_i = (i + 1) % polygon.len();
        let v2_i = (i + 2) % polygon.len();

        let v0 = polygon[i].to_vec3();
        let v1 = polygon[v1_i].to_vec3();
        let v2 = polygon[v2_i].to_vec3();

        if is_wide_angle(&v0, &v1, &v2) {
              num_wide += 1;
        }

    }


    if num_wide > (polygon.len()  / 2) {
        dir = Direction::Left;
        polygon.reverse();
    }


    let mut list = NodeList::new(&polygon);

    let ears = find_ears(&list);

    let mut triangles = vec![];


    while list.len() >= 3 && list.nodes[list.head].next != list.head  {

        let mut ears = find_ears(&list);

        triangles.push(to_ear_tri(&list, ears[0]));

        list.remove_at(ears[0]);
    }

    Triangulation { polygon, triangles, dir }
}

fn to_ear_tri(poly: &NodeList, index: usize) -> Triangle {

    let left = poly.nodes[index].prev;
    let middle = index;
    let right = poly.nodes[index].next;

    tri(left, middle, right)

}






#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn wide_angle() {


        let v1 = na::Vector3::new(2.0, 2.0, 0.0);
        let v2 = na::Vector3::new(1.0, 1.0, 0.0);
        let v3 = na::Vector3::new(2.0, 0.0, 0.0);


        assert!(is_wide_angle(&v1, &v2, &v3));

        let v1 = na::Vector3::new(0.0, 2.0, 0.0);
        let v2 = na::Vector3::new(2.0, 2.0, 0.0);
        let v3 = na::Vector3::new(1.0, 1.0, 0.0);

        assert!(!is_wide_angle(&v1, &v2, &v3));
    }

    #[test]
    fn is_inside() {

        let square = vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![1.0, 1.0],
                vector![1.0, 0.0],
            ];

        assert_eq!(false, is_inside_triangle(&tri(3,0,1),  &(&square).into(),  vector![1.0, 1.0]));
    }

    #[test]
    fn ear_tips() {

        let square = vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![1.0, 1.0],
                vector![1.0, 0.0],
            ];

        assert_eq!(vec![0,1,2,3], find_ears(&(&square).into()));

        let poly = vec![
                vector![0.0, 0.0],
                vector![0.0, 2.0],
                vector![2.0, 2.0],
                vector![0.5, 1.0],
                vector![2.0, 0.0],
            ];

        let mut nl = (&poly).into();
        let ears = find_ears(&nl);

        assert_eq!(vec![2, 4], ears);

        nl.remove_at(2);

        let ears = find_ears(&nl);

        assert_eq!(vec![1, 4], ears);
    }


    #[test]
    fn ear_clipping_square() {
        let square = vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![1.0, 1.0],
                vector![1.0, 0.0],
            ];

        let expected = Triangulation {
            triangles: vec![tri(3, 0, 1), tri(3, 1, 2)],
            polygon: &square,
        };

        let mono = triangulate_ear_clipping(&square);

        assert_eq!(expected, mono);

    }
}

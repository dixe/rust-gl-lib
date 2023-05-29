use super::*;

#[derive(Debug)]
pub struct ComplexPolygon<'a> {
    pub polygon: &'a Polygon,
    pub indices: &'a Vec::<usize>,
}


pub fn gjk_intersection<T1 : Shape, T2: Shape>(p: &T1, q: &T2) -> bool {

    // TODO: handle same shape, so d = 0 return true
    let mut d = (q.center() - p.center()).normalize();
    let mut simplex = Simplex::default();
    simplex.add(support(p, q, d));
    d = -simplex.first();

    loop {
        let a = support(p, q, d);

        if a.dot(&d) < 0.0 {
            return false;
        }

        simplex.add(a);

        if handle_simplex(&mut simplex, &mut d) {
            return true;
        }
    }
}


fn triple_prod(a: V2, b: V2, c: V2) -> V2 {
    let r = (a.v3().cross(&b.v3())).cross(&c.v3());
    r.xy()
}


fn line_case(s: &mut Simplex, d: &mut V2) -> bool {
    let [a,b,_] = s.data;
    let ab = b - a;
    let ao = -a;

    let ab_perp = triple_prod(ab, ao, ab);

    *d = ab_perp;

    return false;
}

fn triangle_case(s: &mut Simplex, d: &mut V2) -> bool {

    let [a,b,c] = s.data;

    let ab = b-a;
    let ac = c-a;
    let ao = -a;
    let ab_perp = triple_prod(ac,ab,ab);
    let ac_perp = triple_prod(ab, ac, ac);

    if ab_perp.dot(&ao) > 0.0 { //Region AB
        s.remove(Remove::C); // remove c
        *d = ab_perp;
        return false;
    } else if ac_perp.dot(&ao) > 0.0 { // Region AC
        s.remove(Remove::B); // remove b
        *d = ac_perp;
        return false;
    }

    true
}

fn handle_simplex(s: &mut Simplex, d: &mut V2) -> bool {
    if s.len == 2 {
        return line_case(s,d);
    }
    return triangle_case(s,d)
}

fn support<T1 : Shape, T2: Shape>(p: &T1, q: &T2, d: V2) -> V2 {
    p.support(d) - q.support(-d)
}


#[derive(Default)]
struct Simplex {
    len: usize,
    data: [V2;3]
}


impl Simplex {

    /// latest added point is first in array
    fn add(&mut self, p: V2) {

        if self.len == 3 {
            panic!("Should not add to triangel case")
        }

        if self.len == 0 {
            self.data[0] = p;
            self.len = 1;
        }
        else if self.len == 1 {
            self.data[1] = self.data[0];
            self.data[0] = p;
            self.len = 2;
        }
        else if self.len == 2 {
            self.data = [p, self.data[0], self.data[1]];
            self.len = 3;
        }

    }

    fn remove(&mut self, r: Remove) {
        assert_eq!(self.len, 3);

        let [a,b,c] = self.data;

        self.len = 2;
        match r {
            Remove::B => {
                // make C to B
                self.data[1] = self.data[2]
            },
            Remove::C => {}, // just decresing the len is enough
        };
    }

    fn first(&self) -> V2 {
        self.data[0]
    }
}

pub trait Shape {

    fn support(&self, dir: V2) -> V2;

    fn center(&self) -> V2;

}


trait ToV3 {
    fn v3(&self)-> V3;
}


impl ToV3 for V2 {
    fn v3(&self)-> V3 {
        V3::new(self.x, self.y, 0.0)
    }
}

enum Remove {
    B,C
}

/*
impl Shape for Polygon {

    fn support(&self, d: V2) -> V2 {

        let mut p = self.vertices[0];
        let mut val = p.dot(&d);

        for v in &self.vertices {

            let dot_val = v.dot(&d);
            if dot_val > val {
                val = dot_val;
                p = *v;
            }
        }

        p

    }

    fn center(&self) -> V2 {
        let mut c = V2::new(0.0, 0.0);

        for v in &self.vertices {
            c += v;
        }

        c / self.vertices.len() as f32
    }
}
*/

impl<'a> Shape for ComplexPolygon<'a> {

    fn support(&self, d: V2) -> V2 {

        let mut p = self.polygon.vertices[self.indices[0]];
        let mut val = p.dot(&d);

        for idx in self.indices {

            let v = self.polygon.vertices[*idx];
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
            let v = self.polygon.vertices[*idx];
            c += v;
        }

        c / self.indices.len() as f32
    }
}

use crate::buffer;
use crate::gl;
use crate::na;

pub struct Bezier {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
}

#[derive(Debug, Clone, Copy)]
pub struct Curve {
    pub p0: na::Vector2::<f32>,
    pub p1: na::Vector2::<f32>,
    pub p2: na::Vector2::<f32>,
}

// TODO: look at this http://commaexcess.com/articles/6/vector-graphics-on-the-gpu
// TODO: and here https://www.microsoft.com/en-us/research/wp-content/uploads/2005/01/p1000-loop.pdf

impl Bezier {

    pub fn new(gl: &gl::Gl, curve: Curve) -> Bezier {


        let p1_trans = curve.p1 - curve.p0;
        let p2_trans = curve.p2 - curve.p0;

        let uv = na::Vector2::new(p1_trans.x / p2_trans.x, p1_trans.y / p2_trans.y);
        println!("p1_t {:?}", p1_trans);
        println!("p2_t {:?}", p2_trans);
        println!("uv {:?}", uv);


        let vertices: Vec<f32> = vec![
            // positions		UV
            curve.p0.x, curve.p0.y, 	0.0, 0.0,
            curve.p1.x, curve.p1.y, 	uv.x, uv.y,
            curve.p2.x, curve.p2.y,	1.0, 1.0,
        ];


        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            let stride = 4;
            // 3.
            gl.VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);


            gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);

        }

        vbo.unbind();
        vao.unbind();

        Bezier {
            vao,
            _vbo: vbo,
        }
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.vao.bind();
        unsafe {
            // draw
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }

        self.vao.unbind();
    }
}


/*
fn bezier_quad(p0: na::Vector2::<f32>, p1: na::Vector2::<f32>, p2: na::Vector2::<f32>, t: f32) -> na::Vector2::<f32> {
p1 + (1.0 - t)* (1.0 - t) * (p0 - p1) + t*t * (p2-p1)
}

*/
/*
fn bezier_cube(curve: &Curve, t: f32) -> na::Vector2::<f32> {
(1.0 -t) * bezier_quad(curve.p0, curve.p1, curve.p2, t) + t * bezier_quad(curve.p1, curve.p2, curve.p3, t)
}
*/

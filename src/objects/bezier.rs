use crate::buffer;
use crate::gl;
use crate::na;

pub struct Bezier {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    lines: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Curve {
    pub p0: na::Vector3::<f32>,
    pub p1: na::Vector3::<f32>,
    pub p2: na::Vector3::<f32>,
    pub p3: na::Vector3::<f32>,

}

// TODO: look at this http://commaexcess.com/articles/6/vector-graphics-on-the-gpu
// TODO: and here https://www.microsoft.com/en-us/research/wp-content/uploads/2005/01/p1000-loop.pdf

impl Bezier {

    pub fn new(gl: &gl::Gl, curve: Curve, samples: u32) -> Bezier {

        let mut vertices = Vec::new();
        vertices.push(curve.p0);
        for ut in 0..=samples {
            let t = (ut as f32) / ( samples as f32);


            let p = bezier_cube(&curve, t);
            vertices.push(p);
        }


        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            // 3.
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);
        }

        vbo.unbind();
        vao.unbind();



        Bezier {
            vao,
            _vbo: vbo,
            lines: vertices.len() as i32,
        }
    }



    pub fn render(&self, gl: &gl::Gl) {
        self.vao.bind();
        unsafe {
            // draw
            gl.DrawArrays(
                gl::LINE_STRIP,
                0,
                self.lines
            );
        }

        self.vao.unbind();
    }
}


fn bezier_quad(p0: na::Vector3::<f32>, p1: na::Vector3::<f32>, p2: na::Vector3::<f32>, t: f32) -> na::Vector3::<f32> {
    p1 + (1.0 - t)* (1.0 - t) * (p0 - p1) + t*t * (p2-p1)
}

fn bezier_cube(curve: &Curve, t: f32) -> na::Vector3::<f32> {
    (1.0 -t) * bezier_quad(curve.p0, curve.p1, curve.p2, t) + t * bezier_quad(curve.p1, curve.p2, curve.p3, t)
}

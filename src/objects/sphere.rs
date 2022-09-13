use crate::buffer;
use crate::gl;

// Shpere with radius 1, thus the input pos is also the normals
pub struct Sphere {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    elements: i32
}


impl Sphere {

    pub fn new(gl: &gl::Gl, mut slices: u32, mut lines: u32) -> Sphere {

        // there is slices + 2 rows, since we always use 2 points for the poles

        let mut vertices: Vec<f32> = vec![];
        // push north pole
        vertices.push(0.0); // x
        vertices.push(0.0); // y
        vertices.push(1.0); // z

        let mut indices: Vec<u32> = vec![ ];

        lines = u32::max(lines, 3);


        // indices for connecting to north pole
        for i in 0..lines {
            indices.push(0);
            indices.push(i + 1);
            indices.push(1 + ( i + 1) % lines);
        }



        // http://www.songho.ca/opengl/gl_sphere.html
        // theta range is 0..360 degrees, phi is -90..90
        for slice in 1..=slices {
            let phi = std::f32::consts::PI / 2.0 - std::f32::consts::PI * (slice as f32 / (slices + 1) as f32);

            for theta_step in 1..=lines {
                let theta = 2.0 * std::f32::consts::PI * (theta_step as f32 / lines as f32);

                let x = phi.cos() * theta.cos();
                let y = phi.cos() * theta.sin();
                let z = phi.sin();

                vertices.push(x);
                vertices.push(y);
                vertices.push(z);

                let k1 = (slice - 1) * lines;
                let k2 = k1 + lines + 1;
            }
        }



        // indices for connection regular square slices
        for slice in 0..(slices - 1) {

            let first_index = slice * lines + 1;

            for i in 0..lines {

                let i0 = first_index + (i % lines);
                let i1 = first_index + ( (i+1) % lines);
                indices.push(i0);
                indices.push(i1 );
                indices.push(i1 + lines);

                indices.push(i0 + lines);
                indices.push(i0);
                indices.push(i1 + lines);



            }
        }


        // indices for connecting to south pole
        let last_index = slices * lines + 1;
        for i in 0..lines {
            indices.push(last_index);
            indices.push(last_index - 1 - i);
            let i2 = last_index - 1 -  ( i + 1) % lines;
            indices.push(i2);

        }


        // push south pole
        vertices.push(0.0); // x
        vertices.push(0.0); // y
        vertices.push(-1.0); // z

        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 3;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            // 3
            ebo.bind();
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);


            // 4.
            // vertecies
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);
        }

        vbo.unbind();
        vao.unbind();


        Sphere {
            vao,
            _vbo: vbo,
            elements: indices.len() as i32
        }
    }

    pub fn render(&self, gl: &gl::Gl) {

        self.vao.bind();
        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                self.elements,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}

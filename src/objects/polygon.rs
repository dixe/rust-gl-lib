use crate::buffer;
use crate::color::Color;
use crate::gl;

pub struct Polygon {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    elements: i32,
}

impl Polygon {
    /// Vertices is 3d points
    /// Colors is
    pub fn new(
        gl: &gl::Gl,
        indices: &Vec<u32>,
        vertices: &Vec<f32>,
        colors: Option<&Vec<Color>>,
    ) -> Polygon {
        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        // maybe use Sub data to avoid this copy of data. But on the other hand we can aford this one
        // time memory usage

        let mut data = vec![];

        let mut stride = 3;
        let mut has_color = false;

        let mut data_ref = vertices;
        if let Some(ref c) = colors {
            has_color = true;
            stride += 4;
            assert_eq!(
                vertices.len() / 3,
                c.len(),
                "Color and vertices does not match"
            );

            assert_eq!(0, vertices.len() % 3);

            for i in 0..(vertices.len() / 3) {
                let idx = i * 3;
                // vertices
                data.push(vertices[idx]);
                data.push(vertices[idx + 1]);
                data.push(vertices[idx + 2]);

                // Colors

                let col = c[i].as_vec4();
                data.push(col[0]);
                data.push(col[1]);
                data.push(col[2]);
                data.push(col[3]);
            }
        }

        if data.len() > 0 {
            data_ref = &data;
        }

        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(data_ref);

            // 3
            ebo.bind();
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // 4.
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

            // Color if any
            if has_color {
                // Use asnwer for subData maybe
                gl.VertexAttribPointer(
                    1,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                    (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
                );
                gl.EnableVertexAttribArray(1);
            }
        }

        vbo.unbind();
        vao.unbind();

        Polygon {
            vao,
            _vbo: vbo,
            _ebo: ebo,
            elements: indices.len() as i32,
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
                0 as *const gl::types::GLvoid,
            );
        }

        self.vao.unbind();
    }
}

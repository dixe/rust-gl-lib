use crate::buffer;
use crate::gl;


pub struct Mesh {
    pub vao: buffer::VertexArray,
    pub vbo: buffer::ArrayBuffer,
    pub ebo: buffer::ElementArrayBuffer,
    pub elements: i32
}


impl Mesh {

     pub fn empty(gl: &gl::Gl) -> Self {

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);

        Mesh {
            vao,
            vbo,
            ebo,
            elements: 0
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
        self.vao.unbind();
    }
}

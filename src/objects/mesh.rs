use crate::buffer;
use crate::gl;


pub struct Mesh {
    pub vao: buffer::VertexArray,
    pub _vbo: buffer::ArrayBuffer,
    pub _ebo: buffer::ElementArrayBuffer,
    pub elements: i32
}


impl Mesh {
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

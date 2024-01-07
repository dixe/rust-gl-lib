use crate::buffer;
use crate::gl;
use super::RenderObject;

pub struct TextureQuad {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer
}

impl TextureQuad {

    pub fn new(gl: &gl::Gl) -> TextureQuad {

        let vertices: Vec<f32> = vec![
            // positions      // texture coords
            0.5,  0.5, 0.0,   1.0, 1.0,
            -0.5,  0.5, 0.0,  0.0, 1.0,
            -0.5, -0.5, 0.0,  0.0, 0.0,
            0.5, -0.5, 0.0,   1.0, 0.0,
        ];

        // tr tl bl   bl br tr
        let indices: Vec<u32> = vec![
            0,1,2,
            2,3,0];

        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 5;

        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.dynamic_draw_data(&vertices);

            // 3
            ebo.bind();
            ebo.static_draw_data(&indices);

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


            // 5.
            // Texture coord
            gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);

        }

        vbo.unbind();
        vao.unbind();

        TextureQuad {
            vao,
            vbo,
            _ebo: ebo,
        }
    }



    pub fn sub_data(&self, pos_data: &[f32; 12], tex_data: &[f32; 8]) {

        let data: Vec<f32> = vec![
            // positions                             // texture coords
            pos_data[0], pos_data[1], pos_data[2],    tex_data[0], tex_data[1],
            pos_data[3], pos_data[4], pos_data[5],    tex_data[2], tex_data[3],
            pos_data[6], pos_data[7], pos_data[8],    tex_data[4], tex_data[5],
            pos_data[9], pos_data[10], pos_data[11],  tex_data[6], tex_data[7],
        ];

        self.vbo.bind();
        self.vbo.sub_data(&data, 0);
        self.vbo.unbind();
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.vao.bind();
        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }

        self.vao.unbind();
    }
}


impl RenderObject for TextureQuad {
    fn render(&self, gl: &gl::Gl) {
        self.vao.bind();
        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }

        self.vao.unbind();
    }
}

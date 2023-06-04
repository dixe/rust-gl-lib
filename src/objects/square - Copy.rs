use crate::buffer;
use crate::gl;
use crate::shader::BaseShader;
use nalgebra as na;
use na::vector;


use failure;


pub struct SpriteSheetSquare {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
}

impl SpriteSheetSquare {

    pub fn new(gl: &gl::Gl) -> Self {

        let vertices: Vec<f32> = vec![
            // positions      // texture coords
            0.5,  0.5, 0.0,   1.0, 0.0,
            -0.5,  0.5, 0.0,  0.0, 0.0,
            -0.5, -0.5, 0.0,  0.0, 1.0,
            0.5, -0.5, 0.0,   1.0, 1.0,


        ];

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

        Self {
            vao,
            vbo,
            _ebo: ebo,
        }
    }


    pub fn sub_texture_coords(&self, left: f32, right: f32, top: f32, bottom: f32) {


         let data: Vec<f32> = vec![
            // positions      // texture coords
            0.5,  0.5, 0.0,  right, bottom,
            -0.5,  0.5, 0.0,  left, bottom,
            -0.5, -0.5, 0.0,  left, top,
            0.5, -0.5, 0.0,   right, top,


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

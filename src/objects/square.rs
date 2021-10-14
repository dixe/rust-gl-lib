use crate::buffer;
use crate::gl;
use crate::shader::Shader;

use failure;


pub struct Square {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer
}

impl Square {

    pub fn new(gl: &gl::Gl) -> Square {

        let vertices: Vec<f32> = vec![
            // positions
            0.5,  0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0,
        ];

        let indices: Vec<u32> = vec![
            0,1,3,
            1,2,3];


        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);


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

        Square {
            vao,
            _vbo: vbo,
            _ebo: ebo,
        }
    }

    /// Creates a basic default shader that takes a mat4 transformation uniform transform
    pub fn default_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

        // default program for square
        let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

void main()
{
    gl_Position = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

        let frag_source = r"#version 330 core
                    out vec4 FragColor;
                    void main()
                    {
                        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
                    }";

        Shader::new(gl, vert_source, frag_source)
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

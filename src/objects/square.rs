use crate::buffer;
use crate::gl;
use crate::shader::BaseShader;
use nalgebra as na;
use na::vector;


use failure;


pub struct Square {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
}

impl Square {

    pub fn new(gl: &gl::Gl) -> Square {


        let vertices: [f32; 3*4] = [
            // positions
            0.5, -0.5, 0.0,
            0.5,  0.5, 0.0,
            -0.5,  0.5, 0.0,
            -0.5, -0.5, 0.0,
        ];

        let indices: Vec<u32> = vec![
            0,1,3,
            1,2,3];

        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 3;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.dynamic_draw_data(&vertices);

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
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
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

    /// Only works for dynamic draw I think
    pub fn sub_data(&self, gl: &gl::Gl, left: f32, right: f32, top: f32, bottom: f32) {

        let data = [
            right, bottom, 0.0,
            right, top, 0.0,
            left, top, 0.0,
            left, bottom, 0.0
        ];

        self._vbo.bind();
        unsafe {
            gl.BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                3* 4 * std::mem::size_of::<f32>() as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid
            );
        }
        self._vbo.unbind();
    }

    /// Creates a basic default shader that takes a mat4 transformation uniform transform
    pub fn default_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

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

        BaseShader::new(gl, vert_source, frag_source)
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.vao.bind();
        self._vbo.bind();
        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
        self._vbo.unbind();
        self.vao.unbind();
    }

    pub fn line_2d_transform(from_world: na::Vector2::<f32>, to_world: na::Vector2::<f32>, screen_w: f32, screen_h: f32, width: f32) -> na::Matrix4::<f32> {
        // use X axis as the long axis, scale y and z to width
        let screen_from = na::Vector2::new(from_world.x/screen_w * 2.0 - 1.0, (screen_h - from_world.y)/screen_h * 2.0 - 1.0);
        let screen_to = na::Vector2::new(to_world.x/screen_w * 2.0 - 1.0, (screen_h - to_world.y)/screen_h * 2.0 - 1.0);


        let diff = screen_to - screen_from;
        let dist = diff.magnitude();

        let s = na::geometry::Scale3::new(dist, width, width);

        let trans = (screen_to + screen_from) / 2.0;


        let t = na::geometry::Translation3::new(trans.x, trans.y * - 1.0, 0.0);

        let angle = -(diff.y / diff.x).atan();
        let r = na::geometry::Rotation3::new(vector![0.0, 0.0, angle]);

        t.to_homogeneous() * r.to_homogeneous() * s.to_homogeneous()


    }

}

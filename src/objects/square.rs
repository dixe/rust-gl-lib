use crate::buffer;
use crate::gl;
use crate::shader::BaseShader;
use nalgebra as na;
use na::vector;
use super::RenderObject;

use failure;


pub struct Square {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
}


impl RenderObject for Square {
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


impl Square {

    pub fn new(gl: &gl::Gl) -> Self {

        let vertices: [f32; 2 * 4] = [
            // positions
            0.5, -0.5,
            0.5,  0.5,
            -0.5,  0.5,
            -0.5, -0.5,
        ];

        let indices: Vec<u32> = vec![
            0,1,3,
            1,2,3];

        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 2;
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
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);
        }

        vbo.unbind();
        vao.unbind();

        Self {
            vao,
            vbo,
            _ebo: ebo,
        }
    }

    /// Only works for dynamic draw I think
    pub fn sub_data(&self, left: f32, right: f32, top: f32, bottom: f32) {

        let data = [
            right, bottom,
            right, top,
            left, top,
            left, bottom,
        ];

        self.vbo.bind();
        self.vbo.sub_data(&data, 0);
        self.vbo.unbind();
    }

    pub fn sub_data_all(&self, data: &[f32; 8]) {

        self.vbo.bind();
        self.vbo.sub_data(data, 0);
        self.vbo.unbind();
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

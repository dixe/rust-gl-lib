use crate::buffer;
use crate::gl;
use crate::shader::BaseShader;
use nalgebra as na;
use na::vector;


use failure;


pub struct Square {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
}

const ELEMENTS: usize = 6;
const STRIDE: usize = 2;

impl Square {

    pub fn new(gl: &gl::Gl) -> Square {

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            assert_eq!(48, (std::mem::size_of::<f32>()   * ELEMENTS *  STRIDE) as u32);
            vbo.dynamic_draw_size((std::mem::size_of::<f32>()   * ELEMENTS *  STRIDE) as u32);


            // Position
            gl.VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (STRIDE * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);




        }

        vbo.unbind();
        vao.unbind();

        Square {
            vao,
            vbo,
        }
    }

    /// Only works for dynamic draw I think
    pub fn sub_data(& self, gl: &gl::Gl, left: f32, right: f32, top: f32, bottom: f32) {


        let data : [f32; ELEMENTS * STRIDE] = [
            right, top,
            left, top,
            right, bottom,

            right, bottom,
            left, top,
            left, bottom,
        ];

        self.vao.bind();
        self.vbo.bind();
        assert_eq!(data.len(), ELEMENTS * STRIDE);
        unsafe {
            gl.BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid
            );

        }
        self.vbo.unbind();
        self.vao.unbind();
    }



    pub fn render(&self, gl: &gl::Gl) {
        self.vao.bind();

        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, ELEMENTS as i32);
        }

        self.vao.unbind();

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

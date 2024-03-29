use crate::buffer;
use crate::gl;
use crate::shader::BaseShader;
use crate::objects::mesh::Mesh;
use nalgebra as na;
use na::vector;


use failure;

pub struct Plane;

impl Plane {

    pub fn new(gl: &gl::Gl) -> Mesh {

        let vertices: Vec<f32> = vec![
            // positions
            0.5, -0.5, 0.0,       0.0, 0.0, 1.0,
            0.5,  0.5, 0.0,       0.0, 0.0, 1.0,
            -0.5,  0.5, 0.0,      0.0, 0.0, 1.0,
            -0.5, -0.5, 0.0,      0.0, 0.0, 1.0,
        ];

        let indices: Vec<u32> = vec![
            0,1,3,
            1,2,3];

        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 6;
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
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);


            // 5.
            // Normals
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);

        }

        vbo.unbind();
        vao.unbind();

        Mesh {
            vao,
            vbo,
            ebo,
            elements: 6
        }
    }

    /// Creates a basic default shader that takes a mat4 transformation uniform transform
    pub fn default_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

        // default program for plane
        let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

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

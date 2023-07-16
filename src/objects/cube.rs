use crate::buffer;
use crate::{gl, na};
use crate::objects::mesh::Mesh;
use crate::collision3d::CollisionBox;

pub struct Cube;


impl Cube {

    pub fn new(gl: &gl::Gl) -> Mesh {

        let vertices : Vec::<f32> = vec![

            // TOP FACE
            0.5,	0.5,	0.5,   0.0, 0.0, 1.0,
            -0.5,	0.5,	0.5,   0.0, 0.0, 1.0,
            -0.5,        -0.5, 0.5,      0.0, 0.0, 1.0,

            -0.5,        -0.5, 0.5,      0.0, 0.0, 1.0,
            0.5,        -0.5, 0.5,      0.0, 0.0, 1.0,
            0.5,        0.5, 0.5,      0.0, 0.0, 1.0,


            // BACK FACE
            -0.5, 0.5, -0.5,    0.0, 1.0, 0.0,
            -0.5, 0.5, 0.5,      0.0, 1.0, 0.0,
            0.5, 0.5, 0.5,     0.0, 1.0, 0.0,

            -0.5, 0.5, -0.5,    0.0, 1.0, 0.0,
            0.5, 0.5, 0.5,      0.0, 1.0, 0.0,
            0.5, 0.5, -0.5,     0.0, 1.0, 0.0,


            // LEFT SIDE
            0.5, -0.5, -0.5,      1.0, 0.0, 0.0,
            0.5, 0.5, -0.5,      1.0, 0.0, 0.0,
            0.5, 0.5, 0.5,      1.0, 0.0, 0.0,

            0.5, -0.5, -0.5,      1.0, 0.0, 0.0,
            0.5, 0.5, 0.5,      1.0, 0.0, 0.0,
            0.5, -0.5, 0.5,      1.0, 0.0, 0.0,


            // FRONT FACE
            -0.5, -0.5, 0.5,      0.0, -1.0, 0.0,
            -0.5, -0.5, -0.5,    0.0, -1.0, 0.0,
            0.5, -0.5, 0.5,     0.0, -1.0, 0.0,


            0.5, -0.5, -0.5,     0.0, -1.0, 0.0,
            0.5, -0.5, 0.5,      0.0, -1.0, 0.0,
            -0.5, -0.5, -0.5,    0.0, -1.0, 0.0,


            // RIGHT SIDE
            -0.5, -0.5, -0.5,      -1.0, 0.0, 0.0,
            -0.5, 0.5, 0.5,      -1.0, 0.0, 0.0,
            -0.5, 0.5, -0.5,      -1.0, 0.0, 0.0,

            -0.5, 0.5, 0.5,      -1.0, 0.0, 0.0,
            -0.5, -0.5, -0.5,      -1.0, 0.0, 0.0,
            -0.5, -0.5, 0.5,      -1.0, 0.0, 0.0,


            // BOTTOM FACE
            0.5,	-0.5,	-0.5,   0.0, 0.0, -1.0,
            -0.5,	0.5,	-0.5,   0.0, 0.0, -1.0,
            0.5,	0.5,	-0.5,   0.0, 0.0, -1.0,


            0.5,	-0.5,	-0.5,   0.0, 0.0, -1.0,
            -0.5,	-0.5,	-0.5,   0.0, 0.0, -1.0,
            -0.5,	0.5,	-0.5,   0.0, 0.0, -1.0,
        ];


        // TODO: HOW, vetices.len also include normal data??
        let indices: Vec<u32> =  (0..(36 as u32)).collect();

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
            // vertecies
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
            elements: indices.len() as i32
        }
    }


    pub fn from_collision_box(collision_box: CollisionBox, clr: na::Vector3<f32>, gl: &gl::Gl) -> Mesh {


        let vertices: Vec<f32> = vec![
            // vertecies             // colors          //normal

            // TOP FACE
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,

            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,


            // BACK FACE
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,

            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,


            // LEFT SIDE
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,

            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,


            // RIGHT SIDE
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,

            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,


            // FRONT FACE
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,

            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,


            // BOTTOM FACE
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,    clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,

            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
        ];

        let indices: Vec<u32> =  (0..(36 as u32)).collect();

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);

        let stride = 9;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            ebo.bind();
            ebo.static_draw_data(&indices);

            // 4.
            // vertecies
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

            // colors
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(1);

            // normals
            gl.VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(2);

        }

        vbo.unbind();
        vao.unbind();


        Mesh {
            vao,
            vbo,
            ebo,
            elements: indices.len() as i32
        }
    }
}

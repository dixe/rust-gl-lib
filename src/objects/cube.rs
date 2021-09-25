use crate::buffer;
use crate::gl;


pub struct Cube {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
}


impl Cube {

    pub fn new(gl: &gl::Gl) -> Cube {


        // https://programming.vip/docs/vao-and-ebo-representation-of-cube-in-opengl.html
        let vertices: Vec<f32> = vec![
            //                                             X    Y     Z
            -0.5,	0.5,	0.5,   1.0, 0.0, 0.0,	// Left Front Top		- Red	- 0
	    0.5,	0.5,	0.5,   0.0, 1.0, 0.0,	// Right Front Top		- Green	- 1
	    0.5,	0.5,	-0.5,   0.0, 0.0, 1.0,	// Right Front Bottom		- Blue	- 2
	    -0.5,	0.5,	-0.5,   0.0, 1.0, 1.0,	// Left Front Bottom		- Cyan	- 3
	    -0.5,	-0.5,	0.5,   1.0, 0.0, 1.0,	// Left Back Top		- Pink	- 4
	    0.5,	-0.5,	0.5,   1.0, 1.0, 0.0,	// Right Back Top		- Yellow- 5
	    0.5,	-0.5,	-0.5,   0.1, 0.1, 0.1,	// Right Back Bottom		- White - 6
	    -0.5,	-0.5,	-0.5,   1.0, 1.0, 1.0,	// Let Back Bottom		- Gray  - 7
        ];

        // https://programming.vip/docs/vao-and-ebo-representation-of-cube-in-opengl.html
        let _vertices: Vec<f32> = vec![
            -0.5, -0.5, 0.5, 1.0, 0.0, 0.0,	// Front Top Left		- Red	- 0
	    0.5,  -0.5, 0.5, 0.0, 1.0, 0.0,	// Front Top Right		- Green	- 1
	    0.5, -0.5, -0.5, 0.0, 0.0, 1.0,	// Front Bottom Right		- Blue	- 2
	    -0.5, -0.5, -0.5, 0.0, 1.0, 1.0,	// Front Bottom Left		- Cyan	- 3
	    -0.5, 0.5, 0.5, 1.0, 0.0, 1.0,	// Back Top Left		- Pink	- 4
	    0.5,  0.5, 0.5, 1.0, 1.0, 0.0,	// Back Top Right		- Yellow- 5
	    0.5, 0.5, -0.5, 0.1, 0.1, 0.1,	// Back Bottom Right		- White - 6
	    -0.5, 0.5, -0.5, 1.0, 1.0, 1.0,	// Back Bottom Left		- Gray  - 7
        ];


        let indices: Vec<u32> = vec![
	    3,2,6,	//Bottom
	    6,7,3,
            0,3,2,	//Front
	    2,1,0,
            1,5,6,	//Right
	    6,2,1,
	    5,4,7,	//Left
	    7,6,5,
	    4,7,3,	//Back
	    3,0,4,
	    4,5,1,	//Top
	    1,0,4,
        ];



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

            // 4.
            // Colors
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


        Cube {
            vao,
            _vbo: vbo,
        }
    }

    pub fn render(&self, gl: &gl::Gl) {

        self.vao.bind();
        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                36,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}

use crate::buffer;
use crate::color::Color;
use crate::gl;
use crate::shader::BaseShader;

pub struct Polygon {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    ebo: buffer::ElementArrayBuffer,
    elements: i32,
    has_color: bool,
    vbo_size: usize,
    ebo_size: usize,
}

impl Polygon {
    /// Vertices is 3d points
    /// Colors is
    pub fn new(
        gl: &gl::Gl,
        indices: &[u32],
        vertices: &[f32],
        colors: Option<&[Color]>
    ) -> Polygon {

        let buffer_data = instanciate_data_buffers(gl, indices, vertices);
        buffer_data.vao.unbind();

        let mut has_color = false;
        if let Some(ref c) = colors {
            has_color = true;
            // TODO: Maybe use another vbo for colors, and not the same as vertices, so they can be replaced seperatly
            // or just make init code simpler, since they don't have to be interlaced
            todo!("Fixed color buffer");
        }

        Polygon {
            vao: buffer_data.vao,
            vbo: buffer_data.vbo,
            ebo: buffer_data.ebo,
            elements: buffer_data.elements,
            has_color,
            ebo_size: indices.len(),
            vbo_size: vertices.len(),
        }

    }



    /// Only works for dynamic draw I think
    pub fn sub_data(&mut self, gl: &gl::Gl, indices: &[u32], vertices: &[f32], colors: Option<&[Color]>) {
        // check ebo and vbo size and recreate buffers if too small
        if indices.len() > self.ebo_size || vertices.len() > self.vbo_size {

            let buffer_data = instanciate_data_buffers(gl, indices, vertices);

            // new vbo and ebo
            self.vao = buffer_data.vao;
            self.vbo = buffer_data.vbo;
            self.ebo = buffer_data.ebo;
            self.elements = buffer_data.elements;
            self.ebo_size = indices.len();
            self.vbo_size = vertices.len();

        } else {
            setup_data(gl, self, indices, vertices, colors);
        }

    }

    pub fn render(&self, gl: &gl::Gl) {
        self.vao.bind();

        self.ebo.bind();

        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                self.elements,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
        self.ebo.unbind();
        self.vao.unbind();
    }

    pub fn create_shader(gl: &gl::Gl) ->  Result<BaseShader, failure::Error> {
        let vert_source = include_str!("../../assets/shaders/objects/polygon.vert");
        let frag_source = include_str!("../../assets/shaders/objects/polygon.frag");
        BaseShader::new(gl, vert_source, frag_source)

    }
}


fn setup_data(gl: &gl::Gl, polygon: &mut Polygon, indices: &[u32], vertices: &[f32], colors: Option<&[Color]>) {
    let mut data = vec![];

    let has_color = polygon.has_color || match colors {
        Some(_) => true,
        None => false
    };

    if has_color && colors.is_none() {
        panic!("Sub data with colors now without");
    }

    let stride = if has_color {3 + 4} else {3};

    let mut data_ref = vertices;

    if let Some(ref c) = colors {
        todo!();
        assert_eq!(
            vertices.len() / 3,
            c.len(),
            "Color and vertices does not match"
        );

        assert_eq!(0, vertices.len() % 3);

        for i in 0..(vertices.len() / 3) {
            let idx = i * 3;
            // vertices
            data.push(vertices[idx]);
            data.push(vertices[idx + 1]);
            data.push(vertices[idx + 2]);

            // Colors

            let col = c[i].as_vec4();
            data.push(col[0]);
            data.push(col[1]);
            data.push(col[2]);
            data.push(col[3]);
        }
        data_ref = &data;
    }

    polygon.vbo.bind();
    polygon.vbo.sub_data(data_ref,0);
    polygon.vbo.unbind();

    polygon.ebo.bind();
    polygon.ebo.sub_data(indices, 0);

    polygon.ebo.unbind();

    polygon.elements = indices.len() as i32;

}


fn instanciate_data_buffers(gl: &gl::Gl, indices: &[u32], vertices: &[f32]) -> BufferData {

    // new vbo and ebo
    let vao = buffer::VertexArray::new(gl);
    let vbo = buffer::ArrayBuffer::new(gl);
    let ebo = buffer::ElementArrayBuffer::new(gl);
    unsafe {
        vao.bind();
        vbo.bind();
        vbo.dynamic_draw_data(vertices);

        ebo.bind();
        ebo.dynamic_draw_data(indices);

        // 4. vertices
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
            0 as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(0);
    }

    vbo.unbind();
    vao.unbind();


    let elements = indices.len() as i32;

    BufferData {
        vao,
        vbo,
        ebo,
        elements
    }
}


struct BufferData {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    ebo: buffer::ElementArrayBuffer,
    elements: i32,
}

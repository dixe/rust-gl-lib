use crate::buffer;
use crate::gl;
use crate::shader::BaseShader;
use crate::color::Color;

use failure;


#[derive(Debug, Copy, Clone)]
pub struct SquareColors {
    pub top_left: Color,
    pub top_right: Color,
    pub bottom_left: Color,
    pub bottom_right: Color
}

pub struct ColorSquare {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    colors: SquareColors
}

impl ColorSquare {

    pub fn new(gl: &gl::Gl) -> Self {

        let vertices: [f32; 6 * 4] = [
            // positions   // Color
            0.5, -0.5,     1.0, 0.0, 0.0, 1.0,
            0.5,  0.5,     1.0, 1.0, 1.0, 1.0,
            -0.5,  0.5,    0.0, 1.0, 1.0, 1.0,
            -0.5, -0.5,    0.0, 0.0, 0.0, 1.0
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
            vbo.dynamic_draw_data(&vertices);

            // 3
            ebo.bind();
            ebo.static_draw_data(&indices);

            // 4. positions
            gl.VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

            // 5. colors
            gl.VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);
        }

        vbo.unbind();
        vao.unbind();

        Self {
            vao,
            vbo,
            _ebo: ebo,
            colors: SquareColors {
                bottom_right: Color::Rgb(255, 0, 0),
                bottom_left: Color::Rgb(0, 0, 0),
                top_left: Color::Rgb(0, 255, 255),
                top_right: Color::Rgb(255, 255, 255),
            }
        }
    }

    pub fn colors(&self) -> SquareColors {
        self.colors
    }

    /// Only works for dynamic draw I think
    pub fn sub_data(&self, gl: &gl::Gl, left: f32, right: f32, top: f32, bottom: f32,  colors: SquareColors) {

        let c_b_r = colors.bottom_right.as_vec4();
        let c_b_l = colors.bottom_left.as_vec4();
        let c_t_r = colors.top_right.as_vec4();
        let c_t_l = colors.top_left.as_vec4();

        let data = [
            right, bottom, c_b_r.x, c_b_r.y, c_b_r.z, c_b_r.w,
            right, top, c_t_r.x, c_t_r.y, c_t_r.z, c_t_r.w,
            left, top, c_t_l.x, c_t_l.y, c_t_l.z, c_t_l.w,
            left, bottom, c_b_l.x, c_b_l.y, c_b_l.z, c_b_l.w,
        ];

        self.vbo.bind();
        unsafe {
            gl.BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                6 * 4 * std::mem::size_of::<f32>() as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid
            );
        }
        self.vbo.unbind();
    }


    /// Creates a basic default shader that takes a mat4 transformation uniform transform
    pub fn default_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

        // default program for square
        let vert_source = include_str!("../../assets/shaders/objects/color_square_default.vert");

        let frag_source = include_str!("../../assets/shaders/objects/color_square_default.frag");

        BaseShader::new(gl, vert_source, frag_source)
    }


    /// Shader for displaying a hsv H slider in a square
    pub fn h_line_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

        // default program for square
        let vert_source = include_str!("../../assets/shaders/objects/hsv_h_line_shader.vert");

        let frag_source = include_str!("../../assets/shaders/objects/hsv_h_line_shader.frag");

        BaseShader::new(gl, vert_source, frag_source)
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

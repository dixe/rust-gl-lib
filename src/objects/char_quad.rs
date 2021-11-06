use crate::buffer;
use crate::gl;
use crate::text_rendering::font;
use image;


const BUFFER_SIZE: usize = 4098;
// two triangle no EBO
const ELEMENTS: usize = 6;
const STRIDE: usize = 4;

pub struct CharQuad {
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    buffer: [f32; ELEMENTS * BUFFER_SIZE * STRIDE]

}


pub struct ImageInfo {
    pub height: f32,
    pub width: f32
}

impl From<&image::RgbaImage> for ImageInfo {

    fn from(image: &image::RgbaImage) -> Self {
        Self {
            height: image.height() as f32,
            width: image.width() as f32,
        }
    }
}



impl CharQuad {

    pub fn new(gl: &gl::Gl) -> CharQuad {
        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        unsafe {

            vao.bind();
            vbo.bind();

            vbo.dynamic_draw_data((std::mem::size_of::<f32>() * ELEMENTS * BUFFER_SIZE * STRIDE) as u32);

            // Position
            gl.VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

            // Texture coords
            gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (STRIDE * std::mem::size_of::<f32>()) as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);


            vbo.unbind();
            vao.unbind();

        }

        Self {
            vbo: vbo,
            vao: vao,
            buffer: [0.0; ELEMENTS * BUFFER_SIZE * STRIDE]
        }
    }

    pub fn buffer_size(&self) -> usize {
        BUFFER_SIZE
    }

    pub fn update_char(&mut self, buffer_index: usize, x: f32, y: f32, scale_x: f32, scale_y: f32, &chr: &font::PageChar, image_info: ImageInfo) {
        let padding = 0.0;
        // Texture coords
        let t_left = chr.x  / image_info.width - padding;
        let t_right = (chr.x + chr.width)  / image_info.width + padding;
        let t_top = (chr.y  / image_info.height) - padding;
        // We subtract chr.height, since the texture is loaded and flipped.
        let t_bottom = (chr.y  - chr.height) / (image_info.height ) + padding;

        // quad coords
        let x_l = x + chr.x_offset * scale_x;
        let x_r = x + chr.width  * scale_x + chr.x_offset * scale_x;
        let y_t = y  - chr.y_offset * scale_y;
        let y_b = y - chr.height  * scale_y  - chr.y_offset * scale_y;

        let start_index = buffer_index * ELEMENTS * STRIDE;

        // TRIANGLE 0
        // RIGHT TOP
        self.buffer[start_index] = x_r;
        self.buffer[start_index + 1 ] = y_t;
        self.buffer[start_index + 2 ] = t_right;
        self.buffer[start_index + 3 ] = t_top;


        // RIGHT BOTTOM
        self.buffer[start_index + 4] = x_r;
        self.buffer[start_index + 5 ] = y_b;
        self.buffer[start_index + 6 ] = t_right;
        self.buffer[start_index + 7 ] = t_bottom;

        // LEFT TOP
        self.buffer[start_index + 8] = x_l;
        self.buffer[start_index + 9 ] = y_t;
        self.buffer[start_index + 10 ] = t_left;
        self.buffer[start_index + 11 ] = t_top;



        // TRIANGLE 1
        // RIGHT BOTTOM
        self.buffer[start_index + 12] = x_r;
        self.buffer[start_index + 13] = y_b;
        self.buffer[start_index + 14 ] = t_right;
        self.buffer[start_index + 15 ] = t_bottom;


        // LEFT BOTTOM
        self.buffer[start_index + 16] = x_l;
        self.buffer[start_index + 17] = y_b;
        self.buffer[start_index + 18 ] = t_left;
        self.buffer[start_index + 19 ] = t_bottom;

        // LEFT TOP
        self.buffer[start_index + 20] = x_l;
        self.buffer[start_index + 21] = y_t;
        self.buffer[start_index + 22 ] = t_left;
        self.buffer[start_index + 23 ] = t_top;

    }



    pub fn render(&self, gl: &gl::Gl, chars: usize) {
        self.vao.bind();
        unsafe {
            self.vbo.bind();
            gl.BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (chars * ELEMENTS * STRIDE * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.buffer.as_ptr() as *const gl::types::GLvoid
            );
        }
        unsafe {
            // draw

            gl.DrawArrays(gl::TRIANGLES, 0, (chars * ELEMENTS) as i32);
        }

        self.vao.unbind();

    }
}

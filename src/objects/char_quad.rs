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

            vbo.dynamic_draw_size((std::mem::size_of::<f32>() * ELEMENTS * BUFFER_SIZE * STRIDE) as u32);

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

    pub fn render_full_texture(&mut self, buffer_index: usize) {

        // full texture
        let t_left = 0.0;
        let t_right = 1.0;
        let t_top = 1.0;
        let t_bottom = 0.0;


        // quad coords in middle for now
        let x_l = -0.5;
        let x_r = 0.5;
        let y_t = 0.5;
        let y_b = -0.5;


        let start_index = buffer_index * ELEMENTS * STRIDE;

        // TRIANGLE 0
        // RIGHT TOP
        self.buffer[start_index] = x_r;
        self.buffer[start_index + 1 ] = y_t;
        self.buffer[start_index + 2 ] = t_right;
        self.buffer[start_index + 3 ] = t_top;


        // LEFT TOP
        self.buffer[start_index + 4] = x_l;
        self.buffer[start_index + 5 ] = y_t;
        self.buffer[start_index + 6 ] = t_left;
        self.buffer[start_index + 7 ] = t_top;


        // RIGHT BOTTOM
        self.buffer[start_index + 8] = x_r;
        self.buffer[start_index + 9 ] = y_b;
        self.buffer[start_index + 10 ] = t_right;
        self.buffer[start_index + 11 ] = t_bottom;


        // TRIANGLE 1
        // RIGHT BOTTOM
        self.buffer[start_index + 12] = x_r;
        self.buffer[start_index + 13] = y_b;
        self.buffer[start_index + 14 ] = t_right;
        self.buffer[start_index + 15 ] = t_bottom;

        // LEFT TOP
        self.buffer[start_index + 16 ] = x_l;
        self.buffer[start_index + 17 ] = y_t;
        self.buffer[start_index + 18 ] = t_left;
        self.buffer[start_index + 19 ] = t_top;


        // LEFT BOTTOM
        self.buffer[start_index + 20 ] = x_l;
        self.buffer[start_index + 21 ] = y_b;
        self.buffer[start_index + 22 ] = t_left;
        self.buffer[start_index + 23 ] = t_bottom;
    }

    pub fn update_char_pixels(&mut self, buffer_index: usize, x: f32, y: f32, scale: f32, &chr: &font::PageChar, image_info: ImageInfo) {
        let t_left = chr.x  / image_info.width;
        let t_right = (chr.x + chr.width)  / image_info.width;
        let t_top = (chr.y)  / image_info.height;
        // 0.0 is bottom left, to chr.y = top, and bottom is then top - height => chr.y - chr.height
        let t_bottom = (chr.y - chr.height)  / image_info.height;

        //println!("{:?}",(t_top, t_bottom));
        let x_w = chr.width * scale;
        let x_o = chr.x_offset * scale;

        let y_h = chr.height * scale;
        let y_o = chr.y_offset * scale;

        // quad coords pixeles
        let x_l = (x + x_o).round();
        let x_r = (x + x_w + x_o).round();

        let y_t = y + y_o;
        let y_b = y + y_h + y_o;

        let start_index = buffer_index * ELEMENTS * STRIDE;

        // TRIANGLE 0
        // RIGHT TOP
        self.buffer[start_index] = x_r as f32;
        self.buffer[start_index + 1 ] = y_t as f32;
        self.buffer[start_index + 2 ] = t_right as f32;
        self.buffer[start_index + 3 ] = t_top as f32;

        // LEFT TOP
        self.buffer[start_index + 4] = x_l as f32;
        self.buffer[start_index + 5 ] = y_t as f32;
        self.buffer[start_index + 6 ] = t_left as f32;
        self.buffer[start_index + 7 ] = t_top as f32;

        // RIGHT BOTTOM
        self.buffer[start_index + 8] = x_r as f32;
        self.buffer[start_index + 9 ] = y_b as f32;
        self.buffer[start_index + 10 ] = t_right as f32;
        self.buffer[start_index + 11 ] = t_bottom as f32;


        // TRIANGLE 1
        // RIGHT BOTTOM
        self.buffer[start_index + 12] = x_r as f32;
        self.buffer[start_index + 13] = y_b as f32;
        self.buffer[start_index + 14 ] = t_right as f32;
        self.buffer[start_index + 15 ] = t_bottom as f32;

        // LEFT TOP
        self.buffer[start_index + 16 ] = x_l as f32;
        self.buffer[start_index + 17 ] = y_t as f32;
        self.buffer[start_index + 18 ] = t_left as f32;
        self.buffer[start_index + 19 ] = t_top as f32;


        // LEFT BOTTOM
        self.buffer[start_index + 20 ] = x_l as f32;
        self.buffer[start_index + 21 ] = y_b as f32;
        self.buffer[start_index + 22 ] = t_left as f32;
        self.buffer[start_index + 23 ] = t_bottom as f32;
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

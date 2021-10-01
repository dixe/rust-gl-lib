use crate::buffer;
use crate::gl;
use crate::text_rendering::font;
use image;


pub struct CharQuad {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer
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

    pub fn new(gl: &gl::Gl, x: f32, y: f32, scale: f32, &chr: &font::PageChar, image_info: ImageInfo) -> CharQuad {

        let padding = 0.0;
        let left = chr.x  / image_info.width - padding;
        let right = (chr.x + chr.width)  / image_info.width + padding;

        let top = (chr.y  / image_info.height) - padding;
        // We subtract chr.height, since the texture is loaded and flipped.
        let bottom = (chr.y  - chr.height) / (image_info.height ) + padding;

        /*
        println!("(Left, Top) = ({:?} {})",chr.x, chr.y);
        println!("(Right, Bottom) = ({:?},{})",chr.x + chr.width, chr.y - chr.height);
        println!("(left, right, top, bottom) ({}, {}, {}, {})" , left, right, top, bottom);
         */
        // let all chars have height 1 and then set the x to widht/ height
        let x_l = x + chr.x_offset * scale;
        let x_r = x + chr.width  * scale + chr.x_offset * scale;
        let y_t = y  - chr.y_offset * scale;
        let y_b = y - chr.height  * scale  - chr.y_offset * scale;

        //println!("(x_l, x_r, y_t, y_b) ({}, {}, {}, {})", x_l, x_r, y_t, y_b);
        let vertices: Vec<f32> = vec![
            // positions	  // texture coordinates
            x_r,  y_t,		right, top,  // Right Top
            x_r, y_b,		right, bottom,  // Right Bottom
            x_l, y_b,		left, bottom,  // Left Bottom
            x_l,  y_t,		left, top,  // Left Top
        ];

        /*
        let x = (c.width  / c.height );

        let vertices: Vec<f32> = vec![
        // positions	  // texture coordinates
        x,  y_pos,		right, top,  // Right Top
        x, 0.0,		right, bottom,  // Right Bottom
        0.0, 0.0,		left, bottom,  // Left Bottom
        0.0,  y_pos,		left, top,  // Left Top
    ];
         */

        let indices: Vec<u32> = vec![
            0,1,3,
            1,2,3];


        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 4;
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


            // 4. Positions
            gl.VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

            // 5. Texture coords
            gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);
        }

        vbo.unbind();
        vao.unbind();

        CharQuad {
            vao,
            _vbo: vbo,
            _ebo: ebo,
        }
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

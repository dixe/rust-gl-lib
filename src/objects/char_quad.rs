use crate::buffer;
use crate::gl;
use crate::text_rendering::font;

pub struct CharQuad {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer
}

impl CharQuad {

    pub fn new(gl: &gl::Gl, unicode_id: u32, x: f32, y: f32, scale: f32, font: &font::Font) -> CharQuad {
        // TODO: Return a result when invalid char is given

        let c = font.get_char(unicode_id as usize).unwrap();

        let padding = 0.0;
        let left = (c.x as f32) / (font.image.width() as f32) - padding;
        let right = (c.x as f32 + c.width as f32)  / (font.image.width() as f32) + padding;

        let top = (c.y as f32 )  / (font.image.height() as f32) - padding;
        // We subtract c.height, since the texture is loaded and flipped.
        let bottom = (c.y as f32 - c.height as f32 )  / (font.image.height() as f32) + padding;
        /*
        println!("(Left, Top) = ({:?} {})",c.x, c.y);
        println!("(Right, Bottom) = ({:?},{})",c.x + c.width, c.y - c.height);
        println!("(left, right, top, bottom) ({}, {}, {}, {})" , left, right, top, bottom);
         */
        // let all chars have height 1 and then set the x to widht/ height
        let x_l = x;
        let x_r = x + c.width as f32 * scale;
        let y_t = y;
        let y_b = y - c.height as f32 * scale;

        println!("(x_l, x_r, y_t, y_b) ({}, {}, {}, {})", x_l, x_r, y_t, y_b);
        let vertices: Vec<f32> = vec![
            // positions	  // texture coordinates
            x_r,  y_t,		right, top,  // Right Top
            x_r, y_b,		right, bottom,  // Right Bottom
            x_l, y_b,		left, bottom,  // Left Bottom
            x_l,  y_t,		left, top,  // Left Top
        ];

        /*
        let x = (c.width as f32 / c.height as f32);

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

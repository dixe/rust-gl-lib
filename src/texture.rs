use crate::gl;
use image;

pub type TextureId = u32;

pub fn gen_texture(gl: &gl::Gl, img: &image::RgbImage) -> TextureId {

    let mut id: gl::types::GLuint = 0;

    unsafe {
        gl.GenTextures(1, &mut id);

        gl.BindTexture(gl::TEXTURE_2D, id);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img.width() as i32, img.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, img.as_ptr() as *const gl::types::GLvoid);
    }

    id
}



pub fn set_texture(gl: &gl::Gl, texture_id: TextureId) {
    unsafe {
        gl.BindTexture(gl::TEXTURE_2D, texture_id);
    }
}

pub fn bitmap_texture(gl: &gl::Gl, bytes: &[u8], width: i32, height: i32) -> Result<u32, failure::Error> {


    let mut obj: gl::types::GLuint = 0;
    unsafe {
        gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl.GenTextures(1, &mut obj);
        gl.BindTexture(gl::TEXTURE_2D, obj);

        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            width,
            height,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            bytes.as_ptr() as *const gl::types::GLvoid);


        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);


    }

    Ok(obj as u32)
}

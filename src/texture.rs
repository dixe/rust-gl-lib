//! Functions to generat and set textures

use crate::gl;
use image;


/// Wrapper of u32 as texture id
pub type TextureId = u32;

/// Generate an RGB texture using GL_CLAMP_TO_BORDER and GL_LINEAR
/// Return the texture id (u32)
pub fn gen_texture_rgb(gl: &gl::Gl, img: &image::RgbImage) -> TextureId {

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

/// Generate an RGBA texture using GL_CLAMP_TO_BORDER and GL_LINEAR
/// Return the texture id (u32)
pub fn gen_texture_rgba(gl: &gl::Gl, img: &image::RgbaImage) -> TextureId {

    let mut id: gl::types::GLuint = 0;

    unsafe {
        gl.GenTextures(1, &mut id);

        gl.BindTexture(gl::TEXTURE_2D, id);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, img.width() as i32, img.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const gl::types::GLvoid);
    }

    id
}



pub fn gen_texture_depth(gl: &gl::Gl, width: i32, height: i32) -> TextureId {

    let mut id = 0;

    unsafe {
        gl.GenTextures(1, &mut id);

        gl.BindTexture(gl::TEXTURE_2D, id);


        gl.TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, width, height, 0, gl::DEPTH_COMPONENT, gl::FLOAT, 0 as *const gl::types::GLvoid);


        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);


        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

        let border_colors : [f32; 4] = [1.0; 4];
        gl.TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_colors.as_ptr() as *const gl::types::GLfloat) ;
    }

    id
}


/// Wrapper of BindTexture
pub fn set_texture(gl: &gl::Gl, texture_id: TextureId) {

    unsafe {
        gl.BindTexture(gl::TEXTURE_2D, texture_id);
    }
}

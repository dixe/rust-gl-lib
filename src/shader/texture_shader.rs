use std::fmt;
use failure;
use crate::gl;
use crate::texture::{self, TextureId};
use super::*;

#[derive( Clone)]
pub struct TextureShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for TextureShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextureShader")
            .finish()
    }
}

impl TextureShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

    pub fn setup(&self, uni: Uniforms) {

        // sampler 0 since there is only 1 sampler
        self.shader.set_i32(&self.gl, "text_map", 0);
        self.shader.set_mat4(&self.gl, "transform", uni.transform);

        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
            texture::set_texture(&self.gl, uni.texture_id);
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Uniforms {
    pub texture_id: TextureId,
    pub transform: na::Matrix4::<f32>
}



/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/image.vert");

    let frag_source = include_str!("../../assets/shaders/objects/image.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

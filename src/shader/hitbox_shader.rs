use std::fmt;
use failure;
use crate::gl;
use super::*;
use crate::typedef::*;


#[derive( Clone)]
pub struct HitboxShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for HitboxShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HitboxShader")
            .finish()
    }
}

impl HitboxShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

    pub fn set_uniforms(&self, uni: Uniforms) {
        self.shader.set_mat4(&self.gl, "projection", uni.projection);
        self.shader.set_mat4(&self.gl, "view", uni.view);
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Uniforms {
    pub projection: Mat4,
    pub view: Mat4,
}


/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/hitbox.vert");
    let frag_source = include_str!("../../assets/shaders/objects/hitbox.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

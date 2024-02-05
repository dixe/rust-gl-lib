use std::fmt;
use failure;
use crate::gl;
use super::*;

#[derive( Clone)]
pub struct RoundedRectInstancedShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for RoundedRectInstancedShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoundedRectInstancedShader")
            .finish()
    }
}

impl RoundedRectInstancedShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

}

impl TransformationShader for RoundedRectInstancedShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}

/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/rounded_rect_instanced.vert");

    let frag_source = include_str!("../../assets/shaders/objects/rounded_rect_instanced.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

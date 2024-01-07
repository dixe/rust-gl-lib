use std::fmt;
use failure;
use crate::gl;
use super::*;

#[derive( Clone)]
pub struct ViewportShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for ViewportShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ViewportShader")
            .finish()
    }
}

impl ViewportShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

    pub fn setup(&self, uni: Uniforms) {
        self.shader.set_vec4(&self.gl, "u_color", uni.color.as_vec4());
        self.shader.set_mat4(&self.gl, "transform", uni.transform);
    }
}



#[derive(Clone, Debug, Copy)]
pub struct Uniforms {
    pub color: Color,
    pub transform: na::Matrix4::<f32>,
}



fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/viewport.vert");

    let frag_source = include_str!("../../assets/shaders/objects/viewport.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

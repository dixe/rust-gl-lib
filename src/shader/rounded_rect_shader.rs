use std::fmt;
use failure;
use crate::gl;
use super::*;

#[derive( Clone)]
pub struct RoundedRectShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for RoundedRectShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoundedRectShader")
            .finish()
    }
}

impl RoundedRectShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

    pub fn set_uniforms(&self, uni: Uniforms) {

    let c = uni.color.as_vec4();
        self.shader.set_vec4(&self.gl, "u_color", uni.color.as_vec4());

        self.shader.set_f32(&self.gl, "pixel_height", uni.pixel_height);

        self.shader.set_f32(&self.gl, "pixel_width", uni.pixel_width);

        self.shader.set_f32(&self.gl, "radius", uni.radius);
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Uniforms {
    pub color: Color,
    pub pixel_height : f32,
    pub pixel_width: f32,
    pub radius: f32
}

impl TransformationShader for RoundedRectShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}





/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/rounded_rect.vert");

    let frag_source = include_str!("../../assets/shaders/objects/rounded_rect.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

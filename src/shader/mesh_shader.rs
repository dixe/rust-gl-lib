use std::fmt;
use failure;
use crate::gl;
use super::*;
use crate::typedef::*;


#[derive( Clone)]
pub struct MeshShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for MeshShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeshShader")
            .finish()
    }
}

impl MeshShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

    pub fn set_uniforms(&self, uni: Uniforms) {
        self.shader.set_vec3(&self.gl, "lightPos", uni.light_pos);
        self.shader.set_vec3(&self.gl, "lightColor",  uni.light_color.as_vec4().xyz());
        self.shader.set_vec3(&self.gl, "viewPos", uni.view_pos);
        self.shader.set_mat4(&self.gl, "projection", uni.projection);
        self.shader.set_mat4(&self.gl, "view", uni.view);
        self.shader.set_mat4(&self.gl, "model", uni.model);
        self.shader.set_slice_mat4(&self.gl, "uBones", uni.bones);
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Uniforms<'a> {
    pub light_pos: V3,
    pub light_color: Color, // TODO: this should be uniform, for now hard coded to 1.0, 1.0, 1.0
    pub projection: Mat4,
    pub view_pos: V3, // should be camera pos
    pub view: Mat4,
    pub model: Mat4,
    pub bones: &'a [na::Matrix4<f32>],
}


/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = include_str!("../../assets/shaders/objects/mesh_shader.vert");

    let frag_source = include_str!("../../assets/shaders/objects/mesh_shader.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

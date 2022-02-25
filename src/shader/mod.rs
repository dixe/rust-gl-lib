use nalgebra as na;
use crate::gl;

mod program;
mod shader;

pub use self::shader::*;
use self::program::*;

//TODO: Maybe move to other place
pub enum Color {
    Rgb(u8, u8, u8),
    RgbA(u8, u8, u8, u8)
}


pub trait TransformationShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>);
}


pub trait ColorShader {
    fn set_color(&self, color: Color);
}


pub struct BasicShader {
    pub shader: Shader,
    gl: gl::Gl
}

impl BasicShader {
    /// Creates a basic default shader
    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {

           let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

void main()
{
    gl_Position = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

        let frag_source = r"#version 330 core
                    uniform vec4 uColor;
                    out vec4 FragColor;
                    void main()
                    {
                        FragColor = uColor;
                    }";

        Shader::new(gl, vert_source, frag_source).map(|s| BasicShader { gl: gl.clone(), shader:s})
    }
}

impl TransformationShader for BasicShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}

impl ColorShader for BasicShader {
    fn set_color(&self, color: Color) {
        let col = match color {
            Color::Rgb(r,g,b) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0),
            Color::RgbA(r,g,b,a) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0),
        };

        self.shader.set_vec4(&self.gl, "uColor", col);
    }
}

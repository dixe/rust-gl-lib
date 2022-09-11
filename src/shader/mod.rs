use nalgebra as na;
use crate::gl;
use crate::color::*;

mod program;
mod shader;
pub mod rounded_rect_shader;
pub mod circle_shader;

pub use self::shader::*;
use self::program::*;

//TODO: Maybe move to other place

pub trait TransformationShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>);
}


pub trait ColorShader {
    fn set_color(&self, color: Color);
}


pub struct PosShader {
    pub shader: BaseShader,
    gl: gl::Gl
}

impl PosShader {
    /// Creates a basic shader with only position data from
    /// Can set uniform color
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

        BaseShader::new(gl, vert_source, frag_source).map(|s| PosShader { gl: gl.clone(), shader:s})
    }
}

impl TransformationShader for PosShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}

impl ColorShader for PosShader {
    fn set_color(&self, color: Color) {
        let col = color.as_vec4();

        self.shader.set_vec4(&self.gl, "uColor", col);
    }
}


pub struct PosColorShader {
    pub shader: BaseShader,
    gl: gl::Gl
}

impl PosColorShader {
    /// Creates shader with position and color from vertices data
    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {

           let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;

uniform mat4 transform;
//
out VS_OUTPUT {
        vec4 Color;
    } OUT;

void main()
{
    OUT.Color = aColor;
    gl_Position = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

        let frag_source = r"#version 330 core
                    out vec4 FragColor;

in VS_OUTPUT {
        vec4 Color;
    } IN;

                    void main()
                    {
                        vec4 c = IN.Color;
                        FragColor = IN.Color;
                    }";

        BaseShader::new(gl, vert_source, frag_source).map(|s| PosColorShader { gl: gl.clone(), shader:s})
    }
}

impl TransformationShader for PosColorShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}

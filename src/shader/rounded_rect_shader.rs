use std::fmt;
use failure;
use crate::gl;
use super::*;

#[derive( Clone)]
pub struct RoundedRectShader {
    gl: gl::Gl,
    pub shader: Shader,
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

        self.shader.set_f32(&self.gl, "color_scale", uni.color_scale);

        self.shader.set_f32(&self.gl, "h_half", uni.h_half);

        self.shader.set_f32(&self.gl, "w_half", uni.w_half);

        self.shader.set_f32(&self.gl, "radius", uni.radius);
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Uniforms {
    pub color_scale: f32,
    pub h_half : f32,
    pub w_half: f32,
    pub radius: f32
}

impl TransformationShader for RoundedRectShader {
  fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}





/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

    // default program for square
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

out VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} OUT;

void main()
{
    vec4 pos = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    OUT.FragPos = aPos.xy;
    OUT.Pos = aPos.xy;
    gl_Position = pos;

}";

    let frag_source = r"
#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float w_half;
uniform float h_half;

uniform float radius;

uniform float color_scale;

float roundedRectangle(vec2 uv, vec2 size, float radius, float thickness)
{
  float d = length(max(abs(uv), size) - size) - radius;
  return smoothstep(0.66, 0.33, d / thickness);
}



void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // multiply by 2 to get -1.0...1.0 range
    float u = IN.FragPos.x * 2.0;
    float v = IN.FragPos.y * 2.0;


    float aspect = w_half / h_half;

    vec2 uv = vec2(u * aspect, v);

    vec3 col = vec3(.8, 0.8, .8) * color_scale;

    // size = aspect - radius, 1.0 - radius
    vec2 size = vec2(aspect - radius, 1.0 - radius);

    // higher is more blur, and also thicker corners
    float aa = 0.05;
    float dist = roundedRectangle(uv, size, radius, aa);
    col =  col * dist;

    FragColor = vec4(col, smoothstep(0.9, 1.0, dist));

}";

    Shader::new(gl, vert_source, frag_source)
}

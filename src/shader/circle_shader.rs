use std::fmt;
use failure;
use crate::gl;
use super::*;

#[derive( Clone)]
pub struct CircleShader {
    gl: gl::Gl,
    pub shader: BaseShader,
}

impl fmt::Debug for CircleShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CircleShader")
            .finish()
    }
}

impl CircleShader {

    pub fn new(gl: &gl::Gl) -> Result<Self, failure::Error> {
        create_shader(gl).map(|s| Self { gl: gl.clone(), shader:s })
    }

    pub fn set_uniforms(&self, uni: Uniforms) {

        self.shader.set_f32(&self.gl, "color_scale", uni.color_scale);

        self.shader.set_f32(&self.gl, "pixel_height", uni.pixel_height);

        self.shader.set_f32(&self.gl, "pixel_width", uni.pixel_width);

        self.shader.set_f32(&self.gl, "radius", uni.radius);
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Uniforms {
    pub color_scale: f32,
    pub pixel_height : f32,
    pub pixel_width: f32,
    pub radius: f32
}

impl TransformationShader for CircleShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}





/// Creates a basic default shader that takes a mat4 transformation uniform transform
fn create_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

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

uniform float pixel_width;
uniform float pixel_height;

uniform float radius;

uniform float color_scale;

float circle(vec2 p, float radius)
{
   return length(p) - radius;
}

void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs
    // or https://iquilezles.org/articles/distfunctions

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // multiply by 2 to get -1.0...1.0 range
    float u = IN.FragPos.x * 2.0 ;
    float v = IN.FragPos.y * 2.0;


    // uv but in screen space.
    vec2 uv =  vec2(u * pixel_width , v * pixel_height);

    vec3 col = vec3(.8, 0.8, .8) * color_scale;


    // higher is more blur, and also thicker corners
    float dist = circle(uv , radius) ;


    float alpha =  (1.0 - smoothstep(0.0, 1.0, dist));
    FragColor = vec4(col, alpha);

}";

    BaseShader::new(gl, vert_source, frag_source)
}

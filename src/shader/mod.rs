use nalgebra as na;
use crate::gl;
use crate::color::*;

mod program;
mod shader;

pub mod rounded_rect_shader;
pub mod rounded_rect_instanced_shader;
pub mod viewport_shader;
pub mod circle_shader;
pub mod circle_outline_shader;
pub mod texture_shader;
pub mod hitbox_shader;
pub mod mesh_shader;
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


uniform mat4 transform;
//
out VS_OUTPUT {
        vec4 Color;
    } OUT;

void main()
{
    OUT.Color = vec4(0.0, 0.0, 0.0, 1.0);
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
                        FragColor = vec4(0.3, 0.4,0.3, 1.0);
                    }";

        BaseShader::new(gl, vert_source, frag_source).map(|s| PosColorShader { gl: gl.clone(), shader:s})
    }
}

impl TransformationShader for PosColorShader {
    fn set_transform(&self, transform: na::Matrix4::<f32>) {
        self.shader.set_mat4(&self.gl, "transform", transform);
    }
}



pub fn load_object_shader(name: &str, gl: &gl::Gl) -> Result::<BaseShader, failure::Error> {
    let vp = format!("assets/shaders/objects/{name}.vert");
    let fp = format!("assets/shaders/objects/{name}.frag");
    let vert_shader_path = std::path::Path::new(&vp);
    let vert_source = std::fs::read_to_string(vert_shader_path)
        .expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new(&fp);
    let frag_source = std::fs::read_to_string(frag_shader_path)
        .expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    shader::BaseShader::new(gl, &vert_source, &frag_source)
}

pub fn reload_object_shader(name: &str, gl: &gl::Gl, shader: &mut BaseShader) {
    let vp = format!("assets/shaders/objects/{name}.vert");
    let fp = format!("assets/shaders/objects/{name}.frag");
    let vert_shader_path = std::path::Path::new(&vp);
    let vert_source = std::fs::read_to_string(vert_shader_path)
        .expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new(&fp);
    let frag_source = std::fs::read_to_string(frag_shader_path)
        .expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    match shader::BaseShader::new(gl, &vert_source, &frag_source) {
        Ok(s) => {
            println!("Reloaded {name}");
            *shader = s;
        },
        Err(e) => {
            println!("{:?}",e);
        }
    }
}

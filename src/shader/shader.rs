use std::fmt;
use failure;
use crate::gl;

use crate::na as na;
use crate::shader::*;

/// A shader that has a vertex and fragment shader.
/// This is also entry point for setting uniforms.
#[derive(Clone)]
pub struct Shader {
    program: Program,
}

impl fmt::Debug for Shader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("A shader")
    }
}

impl Shader {

    /// A new shader from vertex and fragment sources
    pub fn new(gl: &gl::Gl, vert_shader: &str, frag_shader: &str) -> Result<Shader, failure::Error> {

        let program = match Program::from_text(gl, vert_shader, frag_shader) {
            Ok(p) => p,
            Err(msg) => {
                panic!("Error compiling shader {:#?}", msg);
            }
        };

        Ok(Shader {
            program
        })
    }

    pub fn program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

    pub fn set_used(&self) {
        self.program.set_used();
    }

    pub fn set_vec2(&self, gl: &gl::Gl, name: &str, data: na::Vector2<f32>) {
        self.program.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);
            gl.Uniform2f(proj_loc, data.x, data.y);
        }
    }

    pub fn set_vec3(&self, gl: &gl::Gl, name: &str, data: na::Vector3<f32>) {
        self.program.set_used();


        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);


            gl.Uniform3f(proj_loc, data.x, data.y, data.z);
        }
    }


    pub fn set_vec4(&self, gl: &gl::Gl, name: &str, data: na::Vector4<f32>) {
        self.program.set_used();


        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);


            gl.Uniform4f(proj_loc, data.x, data.y, data.z, data.w);
        }
    }


    pub fn set_mat4(&self, gl: &gl::Gl, name: &str, data: na::Matrix4<f32>) {

        self.program.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(self.program.id(), proj_str.as_ptr() as *mut gl::types::GLchar);

            gl.UniformMatrix4fv(proj_loc, 1, gl::FALSE, data.as_slice().as_ptr() as *const f32);
        }
    }


    pub fn set_i32(&self, gl: &gl::Gl, name: &str, data: i32) {

        self.program.set_used();
        unsafe {
            let name_str = std::ffi::CString::new(name).unwrap();

            let loc = gl.GetUniformLocation( self.program.id(), name_str.as_ptr() as *mut gl::types::GLchar);

            gl.Uniform1i(loc, data);
        }
    }

    pub fn set_f32(&self, gl: &gl::Gl, name: &str, data: f32) {

        self.program.set_used();
        unsafe {
            let name_str = std::ffi::CString::new(name).unwrap();

            let loc = gl.GetUniformLocation( self.program.id(), name_str.as_ptr() as *mut gl::types::GLchar);

            gl.Uniform1f(loc, data);
        }
    }




    /// a default shader for rendering a bezier curve
    pub fn bezier_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {
        // default program for square
        let vert_source = r"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 uv;

out VS_OUTPUT {
    smooth vec2 uv;
} OUT;


void main()
{
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);

   OUT.uv = uv;
}";

        let frag_source = r"#version 330 core
out vec4 FragColor;

in VS_OUTPUT {
    smooth vec2 uv;
} IN;

void main()
{
    float u = IN.uv.s;
    float v = IN.uv.t;

    if ((u * u) - v > 0)
    {
        discard;
    }


    FragColor = vec4(1.0f , 0.5f, 0.2f, 1.0);
}";
        Shader::new(gl, vert_source, frag_source)

    }
}

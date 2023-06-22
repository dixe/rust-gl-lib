use std::fmt;
use failure;
use crate::gl;
use std::collections::HashMap;
use crate::na as na;
use crate::shader::*;

/// A shader that has a vertex and fragment shader.
/// This is also entry point for setting uniforms.
#[derive(Clone)]
pub struct BaseShader {
    program: Program,
    locations: HashMap::<String, i32>
}

impl fmt::Debug for BaseShader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("A shader")
    }
}


impl BaseShader {

    /// A new shader from vertex and fragment sources
    pub fn new(gl: &gl::Gl, vert_shader: &str, frag_shader: &str) -> Result<BaseShader, failure::Error> {

        let program = Program::from_text(gl, vert_shader, frag_shader)?;

        Ok(BaseShader {
            program,
            locations: Default::default()
        })
    }


    pub fn set_used(&self) {
        self.program.set_used();
    }

    pub fn program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

    pub fn get_location(&self, name: &str) -> i32{

        if let Some(&loc) = self.locations.get(name) {
            return loc;
        };

        return -1;
    }

    pub fn gl(&self) -> &gl::Gl {
        &self.program.gl

    }

    pub fn set_locations(&mut self, gl: &gl::Gl, name: &str) {

        let name_str =  std::ffi::CString::new(name).unwrap();
        let loc : i32;
        unsafe {
            loc = gl.GetUniformLocation(
                self.program_id(),
                name_str.as_ptr() as *mut gl::types::GLchar,
            );

            self.locations.insert(name.to_string(), loc);
        }
    }



    /// a default shader for rendering a bezier curve
    pub fn bezier_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {
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
        BaseShader::new(gl, vert_source, frag_source)
    }
}


pub trait Shader  {

    fn program_id(&self) -> gl::types::GLuint;

    fn set_used(&self);

    fn set_vec2(&self, gl: &gl::Gl, name: &str, data: na::Vector2<f32>) {
        self.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program_id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);
            gl.Uniform2f(proj_loc, data.x, data.y);
        }
    }

    fn set_vec3(&self, gl: &gl::Gl, name: &str, data: na::Vector3<f32>) {
        self.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program_id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);


            gl.Uniform3f(proj_loc, data.x, data.y, data.z);
        }
    }


    fn set_vec4(&self, gl: &gl::Gl, name: &str, data: na::Vector4<f32>) {
        self.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program_id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);


            gl.Uniform4f(proj_loc, data.x, data.y, data.z, data.w);
        }
    }


    fn set_mat4(&self, gl: &gl::Gl, name: &str, data: na::Matrix4<f32>) {
        self.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(self.program_id(), proj_str.as_ptr() as *mut gl::types::GLchar);

            gl.UniformMatrix4fv(proj_loc, 1, gl::FALSE, data.as_slice().as_ptr() as *const f32);
        }
    }


    fn set_i32(&self, gl: &gl::Gl, name: &str, data: i32) {
        self.set_used();
        unsafe {
            let name_str = std::ffi::CString::new(name).unwrap();

            let loc = gl.GetUniformLocation( self.program_id(), name_str.as_ptr() as *mut gl::types::GLchar);

            gl.Uniform1i(loc, data);
        }
    }

    fn set_f32(&self, gl: &gl::Gl, name: &str, data: f32) {
        self.set_used();
        unsafe {
            let name_str = std::ffi::CString::new(name).unwrap();

            let loc = gl.GetUniformLocation( self.program_id(), name_str.as_ptr() as *mut gl::types::GLchar);

            gl.Uniform1f(loc, data);
        }
    }

    fn set_slice_mat4(&self, gl: &gl::Gl, name: &str, data: &[na::Matrix4<f32>]) {
        let name_str = std::ffi::CString::new(name).unwrap();
        let len: i32 = data.len() as i32;

        unsafe {
            let loc = gl.GetUniformLocation(
                self.program_id(),
                name_str.as_ptr() as *mut gl::types::GLchar,
            );

            gl.UniformMatrix4fv(loc, len, gl::FALSE, data.as_ptr() as *const f32);
        }
    }

}


impl Shader for BaseShader {

    fn set_used(&self) {
        self.set_used();
    }

    fn program_id(&self) -> gl::types::GLuint {
        self.program.id()
    }

}

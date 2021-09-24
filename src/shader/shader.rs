
use failure;
use crate::gl;

use crate::na as na;
use crate::shader::*;

/// A shader that has a vertex and fragment shader.
/// This is also entry point for setting uniforms.
pub struct Shader {
    program: Program,
}

impl Shader {

    /// A new shader from vertex and fragment sources
    pub fn new(gl: &gl::Gl, vert_shader: &str, frag_shader: &str) -> Result<Shader, failure::Error> {

        let program = Program::from_text(gl, vert_shader, frag_shader)?;

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

    pub fn set_vec3(&self, gl: &gl::Gl, name: &str, data: &na::Vector3<f32>) {
        self.program.set_used();


        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);

            let vec3 = data;
            gl.Uniform3f(proj_loc, vec3.x, vec3.y, vec3.z);
        }
    }

    pub fn set_mat4(&self, gl: &gl::Gl, name: &str, data:na::Matrix4<f32>) {

        self.program.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new(name).unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);

            gl.UniformMatrix4fv(
                proj_loc,
                1,
                gl::FALSE,
                data.as_slice().as_ptr() as *const f32);
        }
    }


    /// Creates a basic default shader
    pub fn default_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

        // default program for square
        let vert_source = r"#version 330 core
                                   layout (location = 0) in vec3 aPos;
                                   void main()
                                   {
                                       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                                   }";

        let frag_source = r"#version 330 core
                    out vec4 FragColor;
                    void main()
                    {
                        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
                    }";

        Shader::new(gl, vert_source, frag_source)
    }
}

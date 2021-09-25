use std;
use std::ffi::{CString, CStr};

use failure::*;
use crate::gl;


pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
}

impl Program {

    pub fn from_text(gl: &gl::Gl, vert_shader: &str, frag_shader: &str) -> Result<Program, Error> {


        let vert_c_source;
        let frag_c_source;

        unsafe {
            vert_c_source = CString::from_vec_unchecked(vert_shader.bytes().collect::<Vec<_>>());
            frag_c_source = CString::from_vec_unchecked(frag_shader.bytes().collect::<Vec<_>>());
        }

        let vert_source = ShaderSource::from_source(gl, &vert_c_source, gl::VERTEX_SHADER).map_err(|message| Error::CompileError {name: "vert_shader".to_string(), message})?;
        let frag_source = ShaderSource::from_source(gl, &frag_c_source, gl::FRAGMENT_SHADER).map_err(|message| Error::CompileError {name: "frag_shader".to_string(), message})?;


        Program::from_shaders(gl, &[vert_source, frag_source]).map_err(|message| Error::LinkError {
            name: "default from text".to_string(),
            message
        })



    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[ShaderSource]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()); }
        }

        unsafe { gl.LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {

            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error: CString = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());

        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id); }
        }

        Ok(Program {
            gl: gl.clone(),
            id : program_id
        })
    }


    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id)
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}


impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}



pub struct ShaderSource {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl ShaderSource {

    fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<ShaderSource, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(ShaderSource { gl: gl.clone(), id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}


impl Drop for ShaderSource {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}


fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLenum
) -> Result<gl::types::GLuint,String> {

    let id = unsafe { gl.CreateShader(kind) };

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id)
    };

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl.GetShaderiv(id,gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}



fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe {
        return CString::from_vec_unchecked(buffer)
    };
}

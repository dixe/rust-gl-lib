use crate::gl;

pub struct Buffer<B> where B: BufferType {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>,
}


pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;

impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}


pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}



impl<B> Buffer<B> where B: BufferType {
    pub fn new(gl: &gl::Gl) -> Buffer<B> {
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Buffer
        {
            gl: gl.clone(),
            vbo,
            _marker: ::std::marker::PhantomData,
        }
    }


    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, 0);
        }
    }


    pub fn dynamic_draw_data(&self, size: u32) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                size as gl::types::GLsizeiptr,
                0 as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

}

impl<B> Drop for Buffer<B> where B: BufferType {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;


pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}



impl VertexArray {
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray
        {
            gl: gl.clone(),
            vao
        }
    }


    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }


    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vao);
        }

    }
}

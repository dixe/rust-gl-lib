use crate::gl;

use crate::texture;
pub struct Buffer<B> where B: BufferType {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>,
}


pub struct FrameBuffer {
    gl: gl::Gl,
    fbo: gl::types::GLuint,
    pub color_tex: texture::TextureId,
    pub depth_stencil_tex: texture::TextureId,
    pub depth_test: bool,
    // clear color r,g,b,a
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl FrameBuffer {
    /// create a framebuffer with color in color_tex and depth+stencil in depth_stencil_tex
    /// could also use render buffer for depth and stencil if we don't want to sample directly from depth_stencil_tex any way
    /// for now use texture so we can debug render it OPTIMIZATION
    pub fn new(gl: &gl::Gl, viewport: &gl::viewport::Viewport) -> Self {
        let mut fbo: gl::types::GLuint = 0;

        // gen a texture to render to and a depth/stencil buffer
        let color_tex = texture::gen_texture_framebuffer(&gl, viewport);
        let depth_stencil_tex = texture::gen_texture_depth_and_stencil(&gl, viewport);


        unsafe {
            // gen framebuffer
            gl.GenFramebuffers(1, &mut fbo);

            // bind it
            gl.BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_tex, 0);
            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, depth_stencil_tex, 0);


            let status = gl.CheckNamedFramebufferStatus(fbo, gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                panic!("Failed to create frame buffer {:?}", (status, gl::FRAMEBUFFER_COMPLETE));
            }
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        FrameBuffer
        {
            gl: gl.clone(),
            fbo,
            color_tex,
            depth_stencil_tex,
            depth_test: true,
            // clear color r,g,b,a
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 1.0,
        }
    }


    pub fn update_viewport(&mut self, gl: &gl::Gl, viewport: &gl::viewport::Viewport) {
        // update textures

        self.color_tex = texture::gen_texture_framebuffer(&gl, viewport);
        self.depth_stencil_tex = texture::gen_texture_depth_and_stencil(&gl, viewport);
        unsafe {

             // bind it
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.color_tex, 0);
            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, self.depth_stencil_tex, 0);

            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

    }

    pub fn complete(&self) -> bool {
        unsafe {
            let status = self.gl.CheckFramebufferStatus(gl::FRAMEBUFFER);
            println!("{:?}", (status, gl::FRAMEBUFFER_COMPLETE));
            status == gl::FRAMEBUFFER_COMPLETE
        }
    }

    /// BIND FRAME BUFFER FOR BOTH READ AND WRITE
    pub fn bind_and_clear(&self, clear_bits: u32) {
        unsafe {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            self.gl.ClearColor(self.r, self.g, self.b, self.a);
            self.gl.Clear(clear_bits);
            if self.depth_test {
                self.gl.Enable(gl::DEPTH_TEST);
            } else {
                self.gl.Disable(gl::DEPTH_TEST);
            }
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}


impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteFramebuffers(1, &mut self.fbo);
        }
    }
}
pub struct BufferTypeFrame;
impl BufferType for BufferTypeFrame {
    const BUFFER_TYPE: gl::types::GLuint = gl::FRAMEBUFFER;
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


    pub fn dynamic_draw_size(&self, size: u32) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE,
                size as gl::types::GLsizeiptr,
                0 as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }


    pub fn dynamic_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }


    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn sub_data<T>(&self, data: &[T], byte_offset: usize) {
        unsafe {
            self.gl.BufferSubData(
                B::BUFFER_TYPE,
                byte_offset as gl::types::GLsizeiptr,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid
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

}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vao);
        }
    }
}

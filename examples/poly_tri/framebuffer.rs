//Implentation folowing http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-14-render-to-texture/
use gl_lib::gl;

pub struct FrameBuffer {
    framebuffer_id: gl::types::GLuint,
    texture_id: gl::types::GLuint,
    width: i32,
    height: i32,
}

impl FrameBuffer {
    pub fn new(gl: &gl::Gl, width: i32, height: i32) -> Self {
        let mut frame_id: gl::types::GLuint = 0;
        let mut tex_id: gl::types::GLuint = 0;
        unsafe {
            gl.GenFramebuffers(1, &mut frame_id);
            gl.BindFramebuffer(gl::FRAMEBUFFER, frame_id);

            gl.GenTextures(1, &mut tex_id);
            gl.BindTexture(gl::TEXTURE_2D, tex_id);

            // Give an empty image to OpenGL ( the last "0" )
            // TODO: Get width and height from input to new
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                0 as *const gl::types::GLvoid,
            );

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl.FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, tex_id, 0);

            let draw_buffer = [gl::COLOR_ATTACHMENT0];

            gl.DrawBuffers(1, &draw_buffer as *const gl::types::GLenum);

            if gl.CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Frame buffer creation failed");
            }

            // bind screen (0) as framebuffer again
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        FrameBuffer {
            framebuffer_id: frame_id,
            texture_id: tex_id,
            width,
            height,
        }
    }


    pub fn render(&self, gl: &gl::Gl) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
            gl.Viewport(0, 0, self.width, self.height);
        }
    }
}

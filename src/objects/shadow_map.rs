use crate::gl;
use crate::texture;
use crate::na;
use crate::shader::{self, Shader};
use crate::typedef::{V4, Mat4};

pub struct ShadowMap {
    depth_map_fbo: u32,
    pub depth_map: texture::TextureId,
    pub shader: shader::BaseShader,
    w: i32,
    h: i32,
    pub texture_offset: u32,
    pub z_far: f32,
    pub z_near: f32,
    pub size: f32
}

impl ShadowMap {

    pub fn new(gl: &gl::Gl) -> Self {

        // we could use buffer::FrameBuffer, but it is set up for color, depth ect, so easier to just to it manually here
        // so we can set drawbuffer none and reader buffer none,
        let mut depth_map_fbo = 0;
        let w = 4096;
        let h = 4096;
        let depth_map = texture::gen_texture_depth(gl, w, h);

        unsafe {
            gl.GenFramebuffers(1, &mut depth_map_fbo);
            gl.BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo);
            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_map, 0);
            gl.DrawBuffer(gl::NONE);
            gl.ReadBuffer(gl::NONE);
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }


        let shader = shader::load_object_shader("shadow_map", gl).unwrap();
        ShadowMap {
            depth_map_fbo,
            depth_map,
            shader,
            w,
            h,
            texture_offset: 0,
            z_near: 0.5,
            z_far: 50.5,
            size: 10.0
        }
    }


    pub fn pre_render(&self, gl: &gl::Gl, light_pos: na::Vector3::<f32>, light_space_mats: &mut Vec::<na::Matrix4::<f32>>) {

        unsafe {
            gl.Viewport(0, 0, self.w, self.h);
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.depth_map_fbo);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.shader.set_used();

        self.light_space_mats(light_pos, light_space_mats);

        // TODO: we should set the whole vec into mats
        self.shader.set_mat4(gl, "light_space_mat", light_space_mats[0]);
    }



    fn light_space_mats(&self, light_pos: na::Vector3::<f32>, light_space_mats: &mut Vec::<na::Matrix4::<f32>>) {
        light_space_mats.push(self.projection() * self.view(light_pos));
    }

    pub fn projection(&self) -> na::Matrix4::<f32> {
        // see https://learnopengl.com/Guest-Articles/2021/CSM for info about how to generate this and view
        // so that everything in the view frustrum is in the shadow map, but also multiple leves, so small close things
        // still has a nice shadow
        // how much space out light sees, bigger is more space, but also lower resolution in shadow map
        // making this big, like 50 or 100, makes the shadows pixelated
        let size = self.size;
        na::Matrix4::new_orthographic(-size, size, -size, size, self.z_near, self.z_far)

    }


    fn getFrustumCornersWorldSpace(proj: &Mat4, view: Mat4) -> Vec::<V4>
    {

        let inv: Mat4 = (proj * view).try_inverse().unwrap();

        let mut frustumCorners = vec![];
        for xi in 0..2 {
            for yi in 0..2 {
                for zi in 0..2 {
                    let x = xi as f32;
                    let y = yi as f32;
                    let z = zi as f32;

                    let pt = inv * V4::new(
                        2.0 * x - 1.0,
                        2.0 * y - 1.0,
                        2.0 * z - 1.0,
                        1.0);

                    frustumCorners.push(pt / pt.w);

                }
            }
        }

        frustumCorners
    }

    pub fn view(&self, light_pos: na::Vector3::<f32>) -> na::Matrix4::<f32> {

        // TODO: look at https://learnopengl.com/Guest-Articles/2021/CSM so view "follows"
        // user camera, so when not viewing 0,0,0. We still get shadows
        let target = na::Point3::new(0.0, 0.0, 0.0);

        let point_pos = na::Point3::new(light_pos.x, light_pos.y, light_pos.z);

        let up = na::Vector3::new(0.0, 0.0, 1.0);

        na::Matrix::look_at_rh(&point_pos, &target, &up)

    }

    pub fn post_render(&self, gl: &gl::Gl, width: i32, height: i32) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl.Viewport(0, 0, width, height);

            gl.Enable(gl::DEPTH_TEST);
            gl.ActiveTexture(gl::TEXTURE0 + self.texture_offset);
            gl.BindTexture(gl::TEXTURE_2D, self.depth_map);
        }

    }
}

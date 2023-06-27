use crate::gl;
use crate::texture;
use crate::na;
use crate::shader::{self, Shader};

pub struct ShadowMap {
    depth_map_fbo: u32,
    pub depth_map: texture::TextureId,
    pub shader: shader::BaseShader
}

impl ShadowMap {

    pub fn new(gl: &gl::Gl) -> Self {

        let mut depth_map_fbo = 0;

        unsafe {

            gl.GenFramebuffers(1, &mut depth_map_fbo);
        }


        let depth_map = texture::gen_texture_depth(gl, 1024, 1024);

        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo);
            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_map, 0);
            gl.DrawBuffer(gl::NONE);
            gl.ReadBuffer(gl::NONE);
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }


        let shader = create_shader(gl);
        ShadowMap {
            depth_map_fbo,
            depth_map,
            shader,
        }
    }


    pub fn pre_render(&self, gl: &gl::Gl, light_pos: na::Vector3::<f32>) {

        unsafe {
            gl.Viewport(0, 0, 1024, 1024);
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.depth_map_fbo);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.shader.set_used();
        let light_space_mat = self.light_space_mat(light_pos);

        self.shader.set_mat4(gl, "light_space_mat", light_space_mat);
    }



    pub fn light_space_mat(&self, light_pos: na::Vector3::<f32>) -> na::Matrix4::<f32> {
        self.projection() * self.view(light_pos)
    }

    pub fn projection(&self) -> na::Matrix4::<f32> {
        let z_near = 0.5;
        let z_far = 100.5;
        let size = 5.0;
        na::Matrix4::new_orthographic(-size, size, -size, size, z_near, z_far)

    }

    pub fn view(&self, light_pos: na::Vector3::<f32>) -> na::Matrix4::<f32> {

        let target = na::Point3::new(0.0, 0.0, 0.0);

        let point_pos = na::Point3::new(light_pos.x, light_pos.y, light_pos.z);

        let up = na::Vector3::new(0.0, 1.0, 0.0);

        na::Matrix::look_at_rh(&point_pos, &target, &up)

    }

    pub fn post_render(&self, gl: &gl::Gl, width: i32, height: i32) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl.Viewport(0, 0, width, height);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.Enable(gl::DEPTH_TEST);
            gl.ActiveTexture(gl::TEXTURE1);
            gl.BindTexture(gl::TEXTURE_2D, self.depth_map);
        }

    }
}






pub fn create_shader(gl: &gl::Gl) -> shader::BaseShader {
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 BoneWeights;
layout (location = 3) in vec2 BoneIndices;

uniform mat4 light_space_mat;
uniform mat4 model;
uniform mat4 uBones[32];


mat4 boneTransform() {

  if(int(BoneIndices.x) < 0)
  {
    return mat4(1.0);
  }
  mat4 ret;

  // Weight1 * Bone1 + Weight2 * Bone2
  ret = BoneWeights.x * uBones[int(BoneIndices.x)]
       + BoneWeights.y * uBones[int(BoneIndices.y)];

  return ret;

}

void main()
{
  mat4 bt = boneTransform();
    gl_Position = light_space_mat * model * bt * vec4(aPos, 1.0);
}";

    let frag_source = r"#version 330 core
void main()
{
}";


    shader::BaseShader::new(gl, vert_source, frag_source).unwrap()
}

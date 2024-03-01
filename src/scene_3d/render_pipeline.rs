use crate::{gl};
use crate::imode_gui::ui::*;
use crate::animations::skeleton::{Bones};
use crate::shader::{mesh_shader, BaseShader, load_object_shader};
use crate::typedef::*;

use crate::objects::{shadow_map::ShadowMap, cubemap::{Cubemap}};
use crate::camera::{self};

use crate::shader::Shader;
use std::collections::{HashMap};
use crate::color::Color;
use crate::scene_3d::scene_3d::{SceneMesh, EntityId};

use crate::scene_3d::Fbos;
use crate::scene_3d::render_scene;
use std::rc::Rc;
use crate::shader::reload_object_shader;
use crate::shader::texture_shader;
use crate::scene_3d::PostProcessUniformSet;
use crate::buffer;
use crate::scene_3d::RenderMesh;

pub type RenderPipelineId = usize;


fn default_uniform_set<T>(_ :&gl::Gl, _: &mut BaseShader, _ : &T) {

}


pub struct RenderPipeline<UserPostProcessData> {

    pub name: Rc::<str>,
    pub id: RenderPipelineId,

    pub gl: gl::Gl,

    pub clear_buffer_bits: u32,

    // Should these be share?d
    pub shadow_map: Option<ShadowMap>,
    pub cubemap : Option::<Cubemap>,
    pub fbos: Option::<Fbos<UserPostProcessData>>,


    // SHADERS
    pub cubemap_shader: BaseShader,
    pub mesh_shader: mesh_shader::MeshShader,
    pub stencil_shader: Option<mesh_shader::MeshShader>,
}

impl<UserPostProcessData> RenderPipeline<UserPostProcessData> {


    pub fn new(gl: gl::Gl, name: Rc::<str>, id: RenderPipelineId) -> Result<Self,  failure::Error> {

        let mut sm = ShadowMap::new(&gl);
        sm.texture_offset = 1;
        let mesh_shader = mesh_shader::MeshShader::new(&gl)?;
        let cubemap_shader = load_object_shader("cubemap", &gl).unwrap();

        Ok(Self {
            gl,
            name,
            id,
            mesh_shader,
            cubemap_shader,
            cubemap: None,
            fbos: None,
            stencil_shader: None,
            clear_buffer_bits: gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT,
            shadow_map: Some(sm),
        })
    }

    pub fn use_fbos(&mut self,
                    data: UserPostProcessData,
                    fun: Option<PostProcessUniformSet<UserPostProcessData>>,
                    viewport: &gl::viewport::Viewport) {

        // frame buffer to render to
        let mesh_fbo = buffer::FrameBuffer::new(&self.gl, &viewport);

        let mut ui_fbo = buffer::FrameBuffer::new(&self.gl, &viewport);

        // all has to be 0, since opengl works with premultiplied alphas, so if a is 0, all others have to be 0
        ui_fbo.r = 0.0;
        ui_fbo.g = 0.0;
        ui_fbo.b = 0.0;
        ui_fbo.a = 0.0;


        let mut post_process_shader = texture_shader::TextureShader::new(&self.gl).unwrap();

        reload_object_shader("postprocess", &self.gl, &mut post_process_shader.shader);

        self.fbos = Some(Fbos {
            mesh_fbo,
            ui_fbo,
            post_process_shader,
            post_process_uniform_set: fun.unwrap_or(default_uniform_set),
            post_process_data: data
        });
    }

    pub fn use_shadow_map(&mut self) {
        if self.shadow_map.is_none() {
            let mut sm = ShadowMap::new(&self.gl);
            sm.texture_offset = 1;
            self.shadow_map = Some(sm);
        }
    }

    pub fn use_stencil(&mut self) {
        let mut mesh_shader = mesh_shader::MeshShader::new(&self.gl).unwrap();
        mesh_shader.shader = load_object_shader("stencil", &self.gl).unwrap();

        self.stencil_shader = Some(mesh_shader);
        self.clear_buffer_bits |= gl::STENCIL_BUFFER_BIT;

        unsafe {
            self.gl.Enable(gl::STENCIL_TEST);
            self.gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            self.gl.StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
        }
    }

    fn setup_gl_state(&self) {

        // setup state for this pipeline. Settings like depth and stencil can change from pipeline to pipeline
        // so we cannot assume anything between frames
        unsafe {
            // enable/disable stencil buffer. can be enabled/disabled by another renderpipeline
            if self.stencil_shader.is_some() {
                self.gl.Enable(gl::STENCIL_TEST);
            }
            else {
                self.gl.Disable(gl::STENCIL_TEST);
            }


            // check if we should disable depth test
            if self.clear_buffer_bits & gl::DEPTH_BUFFER_BIT == 0 {
                self.gl.Disable(gl::DEPTH_TEST);
            }
            else {
                self.gl.Enable(gl::DEPTH_TEST);
            }
        }

    }

    pub fn render(&mut self, _mesh_data: &Vec::<SceneMesh>,
                  camera: &camera::Camera,
                  light_pos: V3,
                  light_color: Color,
                  ui: &mut Ui,
                  viewport: &gl::viewport::Viewport,
                  _bones: &HashMap::<EntityId, Bones>,
                  default_bones: &Bones,
                  render_meshes: &[RenderMesh],) {



        self.setup_gl_state();

        // TODO: Allocate once and reuse a vec, maybe just on scene
        let mut light_space_mats = vec![];

        // RENDER TO SHADOW MAP
        if let Some(sm) = &self.shadow_map {
            // calc and set light_space_mats
            sm.pre_render(&self.gl, light_pos,  &mut light_space_mats);

            unsafe {
                self.gl.Enable(gl::CULL_FACE);
                self.gl.CullFace(gl::FRONT);
            }


            for rm in render_meshes {
                sm.shader.set_mat4(&self.gl, "model", rm.model_mat);
                sm.shader.set_slice_mat4(&self.gl, "uBones", rm.bones);

                rm.mesh.render(&self.gl);
            }

            unsafe {
                self.gl.CullFace(gl::BACK);
            }

            sm.post_render(&self.gl, viewport.w, viewport.h);

        }


        if let Some(ref mut fbos) = self.fbos {
            fbos.ui_fbo.unbind();

            fbos.mesh_fbo.bind_and_clear(self.clear_buffer_bits);



            render_scene(&self.gl, &camera, &self.mesh_shader, &default_bones,
                         &self.cubemap, &self.cubemap_shader, &self.stencil_shader, &render_meshes,
                         &light_space_mats, light_pos, light_color);


            fbos.mesh_fbo.unbind();


            // Post process 2, render fbo color texture
            // TODO: maybe add this to unbind?? since unbind is a frame buffer bind or screen buffer.
            unsafe {
                self.gl.Disable(gl::DEPTH_TEST);
                self.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
                self.gl.Clear(gl::COLOR_BUFFER_BIT);
            }

            let w = viewport.w as f32;
            let h = viewport.h as f32;

            let size =  V2::new(w, h);

            fbos.post_process_shader.shader.set_used();
            (fbos.post_process_uniform_set)(&self.gl, &mut fbos.post_process_shader.shader, &fbos.post_process_data);

            ui.drawer2D.render_img_custom_shader(fbos.mesh_fbo.color_tex, 0, 0, size, &fbos.post_process_shader);

            // TODO: Handle this when using instanced ui rendering
            // Draw ui on top at last
            ui.drawer2D.render_img(fbos.ui_fbo.color_tex, 0, 0, size);
        } else {
            render_scene(&self.gl, &camera, &self.mesh_shader, &default_bones,
                         &self.cubemap, &self.cubemap_shader, &self.stencil_shader, &render_meshes,
                         &light_space_mats, light_pos, light_color);
        }
    }
}

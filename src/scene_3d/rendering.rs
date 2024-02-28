use crate::{gl};
use crate::imode_gui::ui::*;
use crate::animations::skeleton::{Bones, Skeleton};
use crate::animations::gltf_animation::{Start, AnimationPlayer, StartTransition};
use crate::objects::gltf_mesh::{self, Animation, KeyFrame};
use crate::shader::{mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use crate::particle_system::{emitter};
use crate::typedef::*;
use crate::texture;
use crate::objects::{shadow_map::ShadowMap, mesh::Mesh, cubemap::{self, Cubemap}};
use crate::camera::{self, free_camera, follow_camera, Camera};
use crate::na::{Translation3, Rotation3, Rotation2};
use crate::{buffer, movement::Inputs};
use crate::shader::Shader;
use crate::audio::audio_player::AudioPlayer;
use std::{thread, sync::{Arc, Mutex}};
use std::rc::Rc;
use std::collections::{VecDeque, HashMap};
use crate::helpers;
use sdl2::event::{Event, WindowEvent};
use crate::collision3d::CollisionBox;
use crate::color::Color;
use crate::particle_system::particle_circle::ParticleCircle;
use core::convert::TryInto;
use crate::scene_3d::scene_3d::{SceneMesh, EntityId};
use crate::scene_3d::SceneEntity;
use crate::scene_3d::Fbos;


pub struct RenderMesh<'a> {
    pub model_mat: Mat4,
    pub mesh: &'a Mesh,
    pub bones: &'a Bones,
    pub texture: Option<texture::TextureId>
}



// Can not be &scene, since then using fbo is not good, and we want a function,
// since we want to call it from mutiple places
pub fn render_scene(gl: &gl::Gl, camera: &Camera,
                    mesh_shader: &mesh_shader::MeshShader,
                    _mesh_data: &Vec::<SceneMesh>,
                    _bones: &HashMap::<EntityId, Bones>,
                    default_bones: &Bones,
                    cubemap_opt: &Option<Cubemap>,
                    cubemap_shader: &BaseShader,
                    stencil_shader: &Option<mesh_shader::MeshShader>,
                    _shadow_map: &Option<ShadowMap>,
                    render_meshes: &[RenderMesh],
                    light_space_mats: &Vec::<Mat4>,
                    light_pos: V3,
                    light_color: Color) {


    let mut uniforms = mesh_shader::Uniforms {
        light_pos,
        light_color,
        projection: camera.projection(),
        model: Mat4::identity(),
        view: camera.view(),
        view_pos: camera.pos(),
        bones: default_bones,
    };

    // SETUP STENCIL
    if stencil_shader.is_some() {
        unsafe {
            gl.StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl.StencilMask(0xFF);
            gl.Enable(gl::DEPTH_TEST);
        }
    }

    // DRAW MESHES
    mesh_shader.shader.set_used();
    mesh_shader.shader.set_i32(gl, "Texture", 0);
    mesh_shader.shader.set_i32(gl, "shadowMap", 1);


    // TODO: We should set mats as a vec
    let mut light_space_mat = Mat4::identity();

    if light_space_mats.len() > 0 {
        light_space_mat = light_space_mats[0];
    }

    for rm in render_meshes {
        uniforms.model = rm.model_mat;
        uniforms.bones = rm.bones;

        mesh_shader.set_uniforms(uniforms);
        mesh_shader.shader.set_mat4(gl,"lightSpaceMat", light_space_mat);

        if let Some(tex) = rm.texture {
            texture::active_texture(gl, 0);
            texture::set_texture(gl, tex);
        }
        rm.mesh.render(gl);

        // STENCIL RENDER PASS
        if let Some(ref stencil) = stencil_shader {
            unsafe {
                gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
                gl.StencilMask(0x00);
            }

            stencil.shader.set_used();
            stencil.set_uniforms(uniforms);

            rm.mesh.render(gl);

            unsafe {
                gl.StencilFunc(gl::ALWAYS, 1, 0xFF);
                gl.StencilMask(0xFF);
                // this make stencil shader individual for each mesh.
                // without this fx outline will be like last image in https://learnopengl.com/Advanced-OpenGL/Stencil-testing
                // seems we want it in a per mesh basis.
                // can also be changes to this is a field on scene and can be set to 0, ie. not clearing
                gl.Clear(gl::STENCIL_BUFFER_BIT);
            }
        }
    }


    // SKYBOX RENDER
    if let Some(ref cubemap) = cubemap_opt {
        // DRAW SKYBOX
        cubemap_shader.set_used();

        // could use nalgebra glm to remove translation part on cpu, and not have gpu multiply ect.
        cubemap_shader.set_mat4(gl, "projection", camera.projection());
        cubemap_shader.set_mat4(gl, "view", camera.view());
        cubemap.render(gl);
    }
}



pub struct RenderPipeLine<UserPostProcessData> {

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

impl<UserPostProcessData> RenderPipeLine<UserPostProcessData> {

    pub fn render(&mut self, mesh_data: &Vec::<SceneMesh>,
                  camera: camera::Camera,
                  light_pos: V3,
                  light_color: Color,
                  ui: &mut Ui,
                  viewport: &gl::viewport::Viewport,
                  bones: &HashMap::<EntityId, Bones>,
                  default_bones: &Bones,
                  entities: &HashMap::<usize, SceneEntity>) {

        // TODO: Maybe have this on scene to reuse vector alloc
        // Setup render meshes data
        let mut render_meshes = vec![];
        for (key, entity) in entities {

            let trans = Translation3::from(entity.pos + entity.root_motion);

            let rotation = Rotation3::from_euler_angles(entity.side_pitch.angle(), entity.forward_pitch.angle(), entity.z_angle.angle());

            render_meshes.push(RenderMesh {
                model_mat: trans.to_homogeneous() * rotation.to_homogeneous(),
                bones: bones.get(key).unwrap_or(&default_bones),
                mesh: &mesh_data[entity.mesh_id].mesh,
                texture: mesh_data[entity.mesh_id].texture_id,
            });
        }


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
            for rm in &render_meshes {
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



            render_scene(&self.gl, &camera, &self.mesh_shader, &mesh_data, &bones, &default_bones,
                         &self.cubemap, &self.cubemap_shader, &self.stencil_shader, &self.shadow_map, &render_meshes,
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
            render_scene(&self.gl, &camera, &self.mesh_shader, &mesh_data, &bones, &default_bones,
                         &self.cubemap, &self.cubemap_shader, &self.stencil_shader, &self.shadow_map, &render_meshes,
                         &light_space_mats, light_pos, light_color);
        }
    }
}

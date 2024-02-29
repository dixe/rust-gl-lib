use crate::{gl};
use crate::imode_gui::ui::*;
use crate::animations::skeleton::{Bones};
use crate::shader::{mesh_shader, BaseShader, load_object_shader};
use crate::typedef::*;
use crate::texture;
use crate::objects::{shadow_map::ShadowMap, mesh::Mesh, cubemap::{Cubemap}};
use crate::camera::{self, Camera};
use crate::na::{Translation3, Rotation3};
use crate::shader::Shader;
use std::collections::{HashMap};
use crate::color::Color;
use crate::scene_3d::scene_3d::{SceneMesh, EntityId};
use crate::scene_3d::SceneEntity;
use crate::scene_3d::Fbos;
use std::rc::Rc;
use crate::shader::reload_object_shader;
use crate::shader::texture_shader;
use crate::scene_3d::PostProcessUniformSet;
use crate::buffer;
use crate::scene_3d::{RenderPipeline, RenderPipelineId, ParticleScene};
use crate::particle_system::{emitter};


// where to keep ids? on scene?
pub struct RenderPipelines<Data> {
    gl: gl::Gl,
    pipelines: Vec::<RenderPipeline<Data>>
}



impl<Data> RenderPipelines<Data> {

    pub fn new(gl: gl::Gl) -> Result::<Self, failure::Error> {
        Ok(Self {
            gl: gl.clone(),
            pipelines: vec![RenderPipeline::new(gl, "default".into(), 0)?]
        })
    }


    pub fn add(&mut self, name: Rc::<str>) -> &mut RenderPipeline<Data> {

        // find max_id
        let mut max_id = 0;
        for pipeline in &self.pipelines {
            max_id = max_id.max(pipeline.id);
        }


        // create new pipeline
        self.pipelines.push(RenderPipeline::new(self.gl.clone(), name.clone(), max_id + 1).unwrap());

        return self.pipeline(name).unwrap();
    }

    pub fn id(&self, name: Rc::<str>) -> Option::<RenderPipelineId> {
        for pipeline in &self.pipelines {
            if name == pipeline.name {
                return Some(pipeline.id);
            }
        }

        return None;
    }

    pub fn default(&mut self) -> &mut RenderPipeline<Data> {
        return self.pipeline("default".into()).expect("Default pipeline should be there, and if removed, don't query it!!");
    }

    pub fn pipeline(&mut self, name: Rc::<str>) ->  Option::<&mut RenderPipeline<Data>> {
        for pipeline in &mut self.pipelines {
            if name == pipeline.name {
                return Some(pipeline);
            }
        }

        return None;
    }

    pub fn mesh_shader(&mut self, name: Rc::<str>) -> Option::<&mut BaseShader> {
        for pipeline in &mut self.pipelines {
            if name == pipeline.name {
                return Some(&mut pipeline.mesh_shader.shader);
            }
        }

        return None;
    }


    pub fn use_shadow_map(&mut self, name: Rc::<str>) {
        for pipeline in &mut self.pipelines {
            if name == pipeline.name {

                pipeline.use_shadow_map();

                break;
            }
        }
    }

    pub fn use_stencil(&mut self, name: Rc::<str>) {
        for pipeline in &mut self.pipelines {
            if name == pipeline.name {

                pipeline.use_stencil();

                break;
            }
        }
    }


    pub fn render(&mut self, mesh_data: &Vec::<SceneMesh>,
                  camera: &camera::Camera,
                  light_pos: V3,
                  light_color: Color,
                  ui: &mut Ui,
                  viewport: &gl::viewport::Viewport,
                  bones: &HashMap::<EntityId, Bones>,
                  default_bones: &Bones,
                  entities: &HashMap::<usize, SceneEntity>,
                  emitter: &emitter::Emitter<ParticleScene>) {



        // setup render meshes for each pipeline
        let mut render_meshes = vec![];
        for _ in 0..self.pipelines.len() {
            render_meshes.push(vec![]);
        }

        for (key, entity) in entities.iter() {

            let trans = Translation3::from(entity.pos + entity.root_motion);

            let rotation = Rotation3::from_euler_angles(entity.side_pitch.angle(), entity.forward_pitch.angle(), entity.z_angle.angle());

            // TODO: We could index out of bound, since pipeline has ids, and we are using them as index

            render_meshes[entity.render_pipeline_id].push(RenderMesh {
                model_mat: trans.to_homogeneous() * rotation.to_homogeneous(),
                bones: bones.get(key).unwrap_or(&default_bones),
                mesh: &mesh_data[entity.mesh_id].mesh,
                texture: mesh_data[entity.mesh_id].texture_id,
            });
        }

        // add particle entities

        for p in emitter.iter() {
            let trans = Translation3::from(p.pos);

            render_meshes[p.render_pipeline_id].push(RenderMesh {
                model_mat: trans.to_homogeneous(),
                bones: &default_bones,
                mesh: &mesh_data[p.mesh_id].mesh,
                texture: mesh_data[p.mesh_id].texture_id,
            });
        }


        // setup render meshes for each pipeline
        for render_pipeline in &mut self.pipelines {
            let id = render_pipeline.id;

            render_pipeline.render(&mesh_data,
                                   &camera,
                                   light_pos,
                                   light_color,
                                   ui,
                                   viewport,
                                   bones,
                                   default_bones,
                                   &render_meshes[id]
            );
        }
    }
}

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
                    default_bones: &Bones,
                    cubemap_opt: &Option<Cubemap>,
                    cubemap_shader: &BaseShader,
                    stencil_shader: &Option<mesh_shader::MeshShader>,
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

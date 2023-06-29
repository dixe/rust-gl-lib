use crate::{gl};
use crate::imode_gui::drawer2d::*;
use crate::imode_gui::ui::*;
use crate::animations::skeleton::{Bones, Skeleton};
use crate::animations::gltf_animation::{Start, AnimationPlayer};
use crate::objects::gltf_mesh::{self, Animation};
use crate::shader::{mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use crate::typedef::*;
use crate::objects::{shadow_map::ShadowMap, mesh::Mesh, cubemap::{self, Cubemap}};
use crate::camera::{self, free_camera, follow_camera, Camera};
use crate::na::{Translation3};
use crate::{buffer, movement::Inputs};
use crate::shader::Shader;
use std::{thread, sync::{Arc, Mutex}};
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;


pub struct DataMap<T> {
    data: HashMap::<usize, T>,
    next_id: usize
}

impl<T> Default for DataMap<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            next_id: 1
        }
    }
}

impl<T> DataMap<T> {
    pub fn insert(&mut self, data: T) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.data.insert(id, data);

        id
    }

    pub fn get(&self, id: &usize) -> Option<&T> {
        self.data.get(&id)
    }
    pub fn get_mut(&mut self, id: &usize) -> Option<&mut T> {
        self.data.get_mut(&id)
    }
}

pub type EntityId = usize;
pub type MeshIndex = usize;
pub type SkeletonIndex = usize;


struct SceneMesh {
    mesh: Mesh,
    skeleton: Option<SkeletonIndex>
}


pub type PostProcessUniformSet<T> = fn(&gl::Gl, &mut BaseShader, &T);

fn default_uniform_set<T>(_ :&gl::Gl, _: &mut BaseShader, _ : &T) {

}

pub struct Fbos<UserPostprocesData> {
    pub mesh_fbo: buffer::FrameBuffer,
    pub ui_fbo: buffer::FrameBuffer,
    pub post_process_shader: texture_shader::TextureShader,
    pub post_process_uniform_set: PostProcessUniformSet<UserPostprocesData>,
    pub post_process_data: UserPostprocesData
}

pub enum SceneControllerSelected {
    Free,
    Follow
}


pub struct Scene<UserPostProcessData> {
    pub ui: Ui,
    pub gl: gl::Gl,

    pub camera: camera::Camera,

    pub inputs: Inputs,
    pub follow_controller: follow_camera::Controller,

    // if we use a follow cam and this is set the inputs will be used to control this entity
    // works as a default 3d char controller.
    // for own implementation keep this as none and just do in in user code
    pub controlled_entity: Option<EntityId>,

    pub selected: SceneControllerSelected,
    pub light_pos: V3,

    pub player: AnimationPlayer<EntityId>,

    pub cubemap : Option::<Cubemap>,
    pub cubemap_shader: BaseShader,

    pub clear_buffer_bits: u32,

    cubemap_imgs: Option::<Arc::<Mutex::<Option<Vec::<image::RgbImage>>>>>,

    pub mesh_shader: mesh_shader::MeshShader,

    // multiple entities can use the same mesh
    pub meshes: HashMap::<Rc::<str>, MeshIndex>, // name to mesh, pt mesh names are uniquie
    mesh_data: Vec::<SceneMesh>, // name to mesh, pt mesh names are uniquie

    // multiple meshes can use the same skeleton, we only need the inverse bind matrix
    pub skeletons: Vec::<Skeleton>,

    // animations is lined to skeleton, so have skeleton
    pub animations: HashMap::<SkeletonIndex, HashMap::<Rc::<str>, Rc<Animation>>>,

    // Each entity can have one set of bones
    pub bones: HashMap::<EntityId, Bones>,

    //pub animation_ids: HashMap::<EntityId, EntityId>, // Kinda want to get rid of this, and maybe just use entityId as key to animaiton player. Maybe the player should just take an id in Start. This is already out of sync and make root motion buggy;
    pub entities: DataMap::<SceneEntity>,

    default_bones: Bones,

    pub fbos: Option::<Fbos<UserPostProcessData>>,

    pub stencil_shader: Option<mesh_shader::MeshShader>,

    pub shadow_map: Option<ShadowMap>,

    //render_meshes: Vec::<RenderMesh>



}


#[derive(Default)]
pub struct SceneEntity {
    pub mesh_id: MeshIndex,
    pub pos: V3,
    // having pos and root motion make everything easier, since we can just set this in animation update.
    // if we tried to directly add it to pos, then we had to take dt into account, and also somehow make sure that we
    // got every part of every frame, so when dt changes frame, we had to add the last part of old frame root motion
    // ect. ect. this make it easier
    pub root_motion: V3,
    pub skeleton_id: Option::<SkeletonIndex>, // Is indirectly a duplicated data, since meshindex points to Scene mesh, which has skel_id. But lets keep it as a convinience
}

impl< UserPostProcessData> Scene<UserPostProcessData> {
    pub fn new(gl: gl::Gl, viewport: gl::viewport::Viewport) -> Result<Scene<UserPostProcessData>, failure::Error> {

        let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
        let ui = Ui::new(drawer_2d);
        let mesh_shader = mesh_shader::MeshShader::new(&gl)?;
        let cubemap_shader = load_object_shader("cubemap", &gl).unwrap();
        let player = AnimationPlayer::<EntityId>::new();

        let mut default_bones = vec![];
        for _i in 0..32 {
            default_bones.push(Mat4::identity())
        }

        unsafe {
            gl.Enable(gl::DEPTH_TEST);
            gl.ClearColor(0.9, 0.9, 0.9, 1.0);
        }

        Ok(Self {
            gl,
            shadow_map: None,
            //render_meshes: vec![],
            ui,
            camera: camera::Camera::new(viewport.w as f32, viewport.h as f32),
            light_pos: V3::new(0.0, 10.0, 30.0),
            inputs: Default::default(),
            follow_controller: Default::default(),
            selected: SceneControllerSelected::Free,
            mesh_shader,
            cubemap_shader,
            player,
            cubemap: None,
            meshes: Default::default(),
            mesh_data: Default::default(),
            entities: Default::default(),
            skeletons: Default::default(),
            animations: Default::default(),
            bones: Default::default(),
            cubemap_imgs: None,
            default_bones,
            fbos: None,
            stencil_shader: None,
            clear_buffer_bits: gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT,
            controlled_entity: None,

        })
    }

    pub fn entity(&self, id: &EntityId) -> Option<&SceneEntity> {
        self.entities.get(id)
    }

    pub fn entity_mut(&mut self, id: &EntityId) -> Option<&mut SceneEntity> {
        self.entities.get_mut(id)
    }

    pub fn create_entity(&mut self, mesh_name: &str) -> EntityId {

        let mesh_id = *self.meshes.get(mesh_name).unwrap();
        let skeleton_id = self.mesh_data[mesh_id].skeleton;
        let entity = SceneEntity {
            mesh_id,
            pos: V3::new(0.0, 0.0, 0.0),
            root_motion: V3::new(0.0, 0.0, 0.0),
            skeleton_id: self.mesh_data[mesh_id].skeleton
        };

        let id = self.entities.insert(entity);

        if let Some(s_id) = skeleton_id {
            // set bones
            self.bones.insert(id, self.skeletons[s_id].create_bones());
        }

        id
    }

    pub fn load_all_meshes(&mut self, path: &str, root_motion: bool) {

        // defaults to not split animations into rotation/scale and motion into root motion
        let gltf_data = gltf_mesh::meshes_from_gltf(path, root_motion).unwrap();

        let mut skin_id_to_skel_idx : HashMap::<usize, usize> = HashMap::default();
        for (skin_id, skeleton) in &gltf_data.skins.skeletons {
            self.skeletons.push(skeleton.clone());
            skin_id_to_skel_idx.insert(*skin_id, self.skeletons.len() - 1);
        }

        for (name, gltf_mesh) in &gltf_data.meshes.meshes {
            let mesh = gltf_mesh.get_mesh(&self.gl);
            // find index into skeletons, if mesh has skeleton
            let skeleton = gltf_data.skins.mesh_to_skin.get(name).map(|skin_id| *skin_id_to_skel_idx.get(skin_id).unwrap());

            self.mesh_data.push(SceneMesh {
                mesh,
                skeleton
            });

            self.meshes.insert(Rc::from(name.to_string()), self.mesh_data.len() - 1);
        }

        for skin_id in gltf_data.animations.keys() {
            let skel_id = skin_id_to_skel_idx.get(skin_id).unwrap();

            if !self.animations.contains_key(skel_id) {
                self.animations.insert(*skel_id, Default::default());
            }

            let map : &mut HashMap::<Rc::<str>, Rc<Animation>> = self.animations.get_mut(skel_id).unwrap();
            for (name, anim) in gltf_data.animations.get(skin_id).unwrap() {
                map.insert(name.clone(), anim.clone());
            }
        }
    }

    pub fn use_shadow_map(&mut self) {
        self.shadow_map = Some(ShadowMap::new(&self.gl));
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



    pub fn use_fbos(&mut self, data: UserPostProcessData, fun: Option<PostProcessUniformSet<UserPostProcessData>>) {

        // frame buffer to render to
        let mesh_fbo = buffer::FrameBuffer::new(&self.gl, &self.ui.drawer2D.viewport);

        let mut ui_fbo = buffer::FrameBuffer::new(&self.gl, &self.ui.drawer2D.viewport);

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

    pub fn set_skybox(&mut self, path: String) {

        //START load
        let cm = Arc::new(Mutex::new(None));
        self.cubemap_imgs = Some(cm.clone());

        thread::spawn(move || {
            let imgs = cubemap::load_cubemap_images(&path);
            {
                let mut mutex_cm = cm.lock().unwrap();
                *mutex_cm = Some(imgs);
            }
        });
    }

    pub fn camera_follow(&mut self, pos: V3) {
        self.follow_controller.update_camera_target(pos);
    }


    pub fn change_camera(&mut self) {
        self.selected = match self.selected {
            SceneControllerSelected::Free => SceneControllerSelected::Follow,
            SceneControllerSelected::Follow => SceneControllerSelected::Free
        }
    }

    pub fn frame_start(&mut self, event_pump: &mut sdl2::EventPump)  {

        if let Some(ref mut fbos) = self.fbos {
            fbos.ui_fbo.bind_and_clear(self.clear_buffer_bits);
        } else {
            unsafe {
                self.gl.Clear(self.clear_buffer_bits);
            }
        }

        let dt = self.dt();

        self.ui.consume_events(event_pump);

        self.inputs.frame_start();

        for event in &self.ui.frame_events {
            self.inputs.update_events(event);
        }

        match self.selected {
            SceneControllerSelected::Free => free_camera::update_camera(&mut self.camera, dt, &self.inputs),
            SceneControllerSelected::Follow => {
                if let Some(entity_id) = self.controlled_entity {
                    let e = self.entities.get_mut(&entity_id).unwrap();

                    // ignore z since we assume its a char controller that cannot fly
                    e.pos.x += self.inputs.movement.x * self.inputs.speed * dt;
                    e.pos.y -= self.inputs.movement.y * self.inputs.speed * dt;

                    let base_sens = 3.0;

                    // update desired camera pitch
                    self.follow_controller.desired_pitch += self.inputs.mouse_movement.yrel * self.inputs.sens * self.inputs.inverse_y *  dt * base_sens;

                    // update camera pos xy (yaw)
                    let vec_xy = (self.camera.pos - e.pos).xy();
                    let dist = vec_xy.magnitude();

                    let mut dir = vec_xy.normalize();

                    let mut tan = dir.yx();
                    tan.y *= -1.0;
                    tan = tan * self.inputs.mouse_movement.xrel * self.inputs.sens * dt * base_sens;
                    dir = (dir + tan).normalize();

                    self.camera.pos.x = e.pos.x + dir.x * dist;
                    self.camera.pos.y = e.pos.y + dir.y * dist;


                    // vec xychange
                }

                self.follow_controller.update_camera(&mut self.camera, dt);
            }
        }


        // WAIT FOR LOAD AND SET OF CUBEMAP
        // also a scene thing, maybe in update if cubemap i set
        if let Some(ref mut cmi) = self.cubemap_imgs {
            if self.cubemap.is_none() {
                let mut lock = cmi.try_lock();
                if let Ok(ref mut mutex_imgs) = lock {
                    if let Some(ref imgs) = **mutex_imgs {
                        self.cubemap = Some(Cubemap::from_images(&self.gl, &imgs));
                    }
                    **mutex_imgs = None
                }
            } else {
                self.cubemap_imgs = None;
            }
        }
    }

    pub fn update_animations(&mut self) {
        let dt = self.dt();
        self.player.update(dt);

        // update entities skeleton
        for (entity_id, entity) in &mut self.entities.data {
            if !self.player.expired(entity_id) {
                if let Some(skel_id) = entity.skeleton_id {

                    let skeleton = self.skeletons.get_mut(skel_id).unwrap();
                    self.player.update_skeleton(*entity_id, skeleton);
                    let bones = self.bones.get_mut(entity_id).unwrap();
                    skeleton.set_all_bones_from_skeleton(bones);
                }

                // update root motion. Maybe have a flag on scene or something to disable this. Or have a sperate method
                entity.root_motion = self.player.root_motion(entity_id);
            } else {
                // just make sure that we update entity pos with root motion now that animatio is done,
                // a new on might start from 0 so we need to make the root motion an actual part of pos
                entity.pos.x += entity.root_motion.x;
                entity.pos.y += entity.root_motion.y;
                entity.root_motion = V3::new(0.0, 0.0, 0.0);
            }
        }
    }



    pub fn render(&mut self) {

        // setup render meshes data
        let mut render_meshes = vec![];
        for (key, entity) in &self.entities.data {

            let trans = Translation3::from(entity.pos + entity.root_motion);

            render_meshes.push(RenderMesh {
                model_mat: trans.to_homogeneous(),
                bones: self.bones.get(key).unwrap_or(&self.default_bones),
                mesh: &self.mesh_data[entity.mesh_id].mesh,
            });
        }




        let mut light_space_mat = Mat4::identity();
        // RENDER TO SHADOW MAP
        if let Some(sm) = &self.shadow_map {
            sm.pre_render(&self.gl, self.light_pos);
            light_space_mat = sm.light_space_mat(self.light_pos);

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

            // TODO: Work with non fixed viewport
            sm.post_render(&self.gl, self.ui.drawer2D.viewport.w, self.ui.drawer2D.viewport.h);
        }



        if let Some(ref mut fbos) = self.fbos {
            fbos.ui_fbo.unbind();

            fbos.mesh_fbo.bind_and_clear(self.clear_buffer_bits);

            render_scene(&self.gl, &self.camera, &self.mesh_shader, &self.mesh_data, &self.bones, &self.default_bones,
                         &self.cubemap, &self.cubemap_shader, &self.stencil_shader, &self.shadow_map, &render_meshes,
                         light_space_mat, self.light_pos);

            fbos.mesh_fbo.unbind();


            // Post process 2, render fbo color texture
            // TODO: maybe add this to unbind?? since unbind is a frame buffer bind or screen buffer.
            unsafe {
                self.gl.Disable(gl::DEPTH_TEST);
                self.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
                self.gl.Clear(gl::COLOR_BUFFER_BIT);
            }

            let w = self.ui.drawer2D.viewport.w as f32;
            let h = self.ui.drawer2D.viewport.h as f32;

            let size =  V2::new(w, h);

            fbos.post_process_shader.shader.set_used();
            (fbos.post_process_uniform_set)(&self.gl, &mut fbos.post_process_shader.shader, &fbos.post_process_data);
            self.ui.drawer2D.render_img_custom_shader(fbos.mesh_fbo.color_tex, 0, 0, size, &fbos.post_process_shader);

            // Draw ui on top at last
            self.ui.drawer2D.render_img(fbos.ui_fbo.color_tex, 0, 0, size);

        } else {
            render_scene(&self.gl, &self.camera, &self.mesh_shader, &self.mesh_data, &self.bones, &self.default_bones,
                         &self.cubemap, &self.cubemap_shader, &self.stencil_shader, &self.shadow_map, &render_meshes,
                         light_space_mat, self.light_pos);
        }
    }


    pub fn dt(&self) -> f32 {
        self.ui.dt()
    }

}



pub fn stop_animation(entity_id: &EntityId, player: &mut AnimationPlayer::<EntityId>, entities: &mut DataMap::<SceneEntity>) {

    if !player.expired(entity_id) {
        player.remove(*entity_id);
        // update entity pos to be pos + root_motion, since root motion will be reset to new anim
        let e = entities.get_mut(entity_id).unwrap();
        // assume that root motion keeps up grounded
        e.pos.x += e.root_motion.x;
        e.pos.y += e.root_motion.y;
    }
}

pub fn play_animation(anim: Rc::<Animation>, repeat: bool, entity_id: &EntityId, player: &mut AnimationPlayer::<EntityId>, entities: &mut DataMap::<SceneEntity>) {

    stop_animation(entity_id, player, entities);

    player.start(Start {anim, repeat, id: *entity_id});
}


pub struct RenderMesh<'a> {
    pub model_mat: Mat4,
    pub mesh: &'a Mesh,
    pub bones: &'a Bones
}

// Can not be &scene, since then using fbo is not good, and we want a function,
// since we want to call it from mutiple places
fn render_scene(gl: &gl::Gl, camera: &Camera,
                mesh_shader: &mesh_shader::MeshShader,
                mesh_data: &Vec::<SceneMesh>,
                bones: &HashMap::<EntityId, Bones>,
                default_bones: &Bones,
                cubemap_opt: &Option<Cubemap>,
                cubemap_shader: &BaseShader,
                stencil_shader: &Option<mesh_shader::MeshShader>,
                shadow_map: &Option<ShadowMap>,
                render_meshes: &[RenderMesh],
                light_space_mat: Mat4,
                light_pos: V3) {


    let mut uniforms = mesh_shader::Uniforms {
        light_pos,
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

    for rm in render_meshes {
        uniforms.model = rm.model_mat;
        uniforms.bones = rm.bones;

        mesh_shader.set_uniforms(uniforms);
        mesh_shader.shader.set_mat4(gl,"lightSpaceMat", light_space_mat);

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
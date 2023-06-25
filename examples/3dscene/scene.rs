use gl_lib::{gl};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer, AnimationId};
use gl_lib::objects::gltf_mesh::{self, Animation};
use gl_lib::shader::{mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cubemap::{self, Cubemap}};
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Translation3};
use gl_lib::{buffer};
use gl_lib::shader::Shader;
use std::{thread, sync::{Arc, Mutex}};
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;


struct DataMap<T> {
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


pub struct Scene<UserPostProcessData> {
    pub ui: Ui,
    pub gl: gl::Gl,

    pub camera: camera::Camera,
    pub free_controller: free_camera::Controller,

    pub player: AnimationPlayer,

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

    pub animation_ids: HashMap::<EntityId, AnimationId>,
    entities: DataMap::<SceneEntity>,

    default_bones: Bones,

    pub fbos: Option::<Fbos<UserPostProcessData>>,

    pub stencil_shader: Option<mesh_shader::MeshShader>

}


#[derive(Default)]
pub struct SceneEntity {
    pub mesh_id: MeshIndex,
    pub pos: V3,
    pub skeleton_id: Option::<SkeletonIndex>, // Is indirectly a duplicated data, since meshindex points to Scene mesh, which has skel_id. But lets keep it as a convinience
}

impl< UserPostProcessData> Scene<UserPostProcessData> {
    pub fn new(gl: gl::Gl, viewport: gl::viewport::Viewport) -> Result<Scene<UserPostProcessData>, failure::Error> {

        let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
        let ui = Ui::new(drawer_2d);
        let mesh_shader = mesh_shader::MeshShader::new(&gl)?;
        let cubemap_shader = load_object_shader("cubemap", &gl).unwrap();
        let player = AnimationPlayer::new();

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
            ui,
            camera: camera::Camera::new(viewport.w as f32, viewport.h as f32),
            free_controller: free_camera::Controller::default(),
            mesh_shader,
            cubemap_shader,
            player,
            cubemap: None,
            meshes: Default::default(),
            mesh_data: Default::default(),
            entities: Default::default(),
            skeletons: Default::default(),
            animations: Default::default(),
            animation_ids: Default::default(),
            bones: Default::default(),
            cubemap_imgs: None,
            default_bones,
            fbos: None,
            stencil_shader: None,
            clear_buffer_bits: gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT

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
            pos: V3::identity(),
            skeleton_id: self.mesh_data[mesh_id].skeleton
        };

        let id = self.entities.insert(entity);

        if let Some(s_id) = skeleton_id {
            // set bones
            self.bones.insert(id, self.skeletons[s_id].create_bones());
        }

        id
    }

    pub fn load_all_meshes(&mut self, path: &str) {

        let gltf_data = gltf_mesh::meshes_from_gltf(path).unwrap();

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

        for event in &self.ui.frame_events {
            self.free_controller.update_events(event);
        }

        self.free_controller.update_camera(&mut self.camera, dt);


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
        for (entity_id, entity) in &self.entities.data {
            if let Some(anim_id) = self.animation_ids.get(entity_id) {
                if let Some(skel_id) = entity.skeleton_id {
                    let skeleton = self.skeletons.get_mut(skel_id).unwrap();
                    self.player.update_skeleton(*anim_id, skeleton);
                    let bones = self.bones.get_mut(entity_id).unwrap();
                    skeleton.set_all_bones_from_skeleton(bones);

                }
            }
        }
    }

    pub fn render(&mut self) {

        if let Some(ref mut fbos) = self.fbos {
            fbos.ui_fbo.unbind();

            fbos.mesh_fbo.bind_and_clear(self.clear_buffer_bits);


            render_scene(&self.gl, &self.camera, &self.mesh_shader, &self.mesh_data, &self.bones, &self.default_bones,
                         &self.entities, &self.cubemap, &self.cubemap_shader, &self.stencil_shader);

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
                         &self.entities, &self.cubemap, &self.cubemap_shader, &self.stencil_shader);
        }
    }


    pub fn dt(&self) -> f32 {
        self.ui.dt()
    }

}


pub fn play_animation(anim: Rc::<Animation>, repeat: bool, entity_id: &EntityId, player: &mut AnimationPlayer, anim_ids: &mut HashMap::<EntityId, AnimationId>) {

    if let Some(anim_id) = anim_ids.get(entity_id) {
        player.remove(*anim_id);
    }

    // get old animation id
    // TODO: let
    let id = player.start(Start {anim, repeat});

    anim_ids.insert(*entity_id, id);


}


// Can not be &scene, since then using fbo is not good, and we want a function,
// since we want to call it from mutiple places
fn render_scene(gl: &gl::Gl, camera: &Camera,
                mesh_shader: &mesh_shader::MeshShader,
                mesh_data: &Vec::<SceneMesh>,
                bones: &HashMap::<EntityId, Bones>,
                default_bones: &Bones,
                entities: &DataMap::<SceneEntity>,
                cubemap_opt: &Option<Cubemap>,
                cubemap_shader: &BaseShader,
                stencil_shader: &Option<mesh_shader::MeshShader>) {

    let pos = V3::new(0.0, 0.0, 0.0);
    let trans = Translation3::from(pos);
    let model_mat = trans.to_homogeneous();

    let mut uniforms = mesh_shader::Uniforms {
        light_pos: V3::new(0.0, 100.0, 100.0),
        projection: camera.projection(),
        model: model_mat,
        view: camera.view(),
        view_pos: camera.pos(),
        bones: default_bones
    };


    if stencil_shader.is_some() {
        unsafe {
            gl.StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl.StencilMask(0xFF);
            gl.Enable(gl::DEPTH_TEST);
        }
    }

    // DRAW MESHES
    mesh_shader.shader.set_used();
    for (key, entity) in &entities.data {

        let trans = Translation3::from(entity.pos);
        uniforms.model = trans.to_homogeneous();

        uniforms.bones = bones.get(key).unwrap_or(default_bones);

        mesh_shader.set_uniforms(uniforms);
        mesh_data[entity.mesh_id].mesh.render(gl);

        if let Some(ref stencil) = stencil_shader {
            unsafe {
                gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
                gl.StencilMask(0x00);
                gl.Disable(gl::DEPTH_TEST);
            }

            stencil.shader.set_used();
            stencil.set_uniforms(uniforms);
            mesh_data[entity.mesh_id].mesh.render(gl);

            unsafe {
                gl.StencilFunc(gl::ALWAYS, 1, 0xFF);
                gl.StencilMask(0xFF);
                gl.Enable(gl::DEPTH_TEST);

                // this make stencil shader individual for each mesh.
                // without this fx outline will be like last image in https://learnopengl.com/Advanced-OpenGL/Stencil-testing
                // seems we want it in a per mesh basis.
                // can also be changes to this is a field on scene and can be set to 0, ie. not clearing
                gl.Clear(gl::STENCIL_BUFFER_BIT);
            }
        }
    }


    if let Some(ref cubemap) = cubemap_opt {
        // DRAW SKYBOX
        cubemap_shader.set_used();

        // could use nalgebra glm to remove translation part on cpu, and not have gpu multiply ect.
        cubemap_shader.set_mat4(gl, "projection", camera.projection());
        cubemap_shader.set_mat4(gl, "view", camera.view());
        cubemap.render(gl);
    }
}

use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::animations::skeleton::{Bones, Skeleton};
use gl_lib::animations::gltf_animation::{Start, AnimationPlayer, AnimationId};
use gl_lib::objects::gltf_mesh::{self, KeyFrame, GltfData, Animation};
use gl_lib::shader::{self, mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use gl_lib::typedef::*;
use gl_lib::objects::{mesh::Mesh, cubemap::{self, Cubemap}};
use gl_lib::camera::{self, free_camera, Camera};
use gl_lib::na::{Scale3, Translation3};
use gl_lib::{buffer, texture};
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
}

pub type EntityId = usize;
pub type MeshIndex = usize;
pub type SkeletonIndex = usize;


struct SceneMesh {
    mesh: Mesh,
    skeleton: Option<SkeletonIndex>
}


pub struct Scene<UserData> {
    pub user_data: UserData,
    pub ui: Ui,
    pub gl: gl::Gl,

    pub camera: camera::Camera,
    pub free_controller: free_camera::Controller,

    pub player: AnimationPlayer,

    pub cubemap : Option::<Cubemap>,
    pub cubemap_shader: BaseShader,

    pub mesh_shader: mesh_shader::MeshShader,
    pub post_process_shader: Option<texture_shader::TextureShader>,

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

}


#[derive(Default)]
pub struct SceneEntity {
    pub mesh_id: MeshIndex,
    pub skeleton_id: Option::<SkeletonIndex>, // Is indirectly a duplicated data, since meshindex points to Scene mesh, which has skel_id. But lets keep it as a convinience
}

impl<UserData> Scene<UserData> {
    pub fn new(user_data: UserData, gl: gl::Gl, viewport: gl::viewport::Viewport) -> Result<Scene< UserData>, failure::Error> {

        let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
        let ui = Ui::new(drawer_2d);
        let mesh_shader = mesh_shader::MeshShader::new(&gl)?;
        let cubemap_shader = load_object_shader("cubemap", &gl).unwrap();
        let player = AnimationPlayer::new();

        let mut default_bones = vec![];
        for i in 0..32 {
            default_bones.push(Mat4::identity())
        }

        unsafe {
            gl.Enable(gl::DEPTH_TEST);
            gl.ClearColor(0.9, 0.9, 0.9, 1.0);
        }

        Ok(Self {
            gl,
            user_data,
            ui,
            camera: camera::Camera::new(viewport.w as f32, viewport.h as f32),
            free_controller: free_camera::Controller::default(),
            mesh_shader,
            post_process_shader: None,
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
            default_bones
        })
    }

    pub fn get_entity(&self, id: &EntityId) -> Option<&SceneEntity> {
        self.entities.get(id)
    }

    pub fn create_entity(&mut self, mesh_name: &str) -> EntityId {

        let mesh_id = *self.meshes.get(mesh_name).unwrap();
        let skeleton_id = self.mesh_data[mesh_id].skeleton;
        let entity = SceneEntity {
            mesh_id,
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

    pub fn set_skybox<P: AsRef<Path> + std::fmt::Debug>(&mut self, path: &P) {

        self.cubemap = Some(cubemap::Cubemap::from_path(&self.gl, path));

        //TODO: do lazy/multiuthreaded loading of sky box

        /*

//START
    let mut cubemap_imgs: Arc::<Mutex::<Option<Vec::<image::RgbImage>>>> = Arc::new(Mutex::new(None));

    // start thread to load cubemap
    let cm = cubemap_imgs.clone();
    thread::spawn(move || {

        let imgs = cubemap::load_cubemap_images(&"assets/cubemap/skybox/");
        {
            let mut mutex_cm = cm.lock().unwrap();
            *mutex_cm = Some(imgs);
        }

    });
         */

        /*

        // WAIT FOR LOAD AND SET
        // also a scene thing, maybe in update if cubemap i set
        if cubemap.is_none() {
            let mut lock = cubemap_imgs.try_lock();
            if let Ok(ref mut mutex_imgs) = lock {
                if let Some(ref imgs) = **mutex_imgs {
                    cubemap = Some(Cubemap::from_images(gl, &imgs));
                    // TODO:  maybe some cleanup of images and img vec, so we don't keep it in ram
                }
                **mutex_imgs = None

            } else {
                println!("try_lock failed");
            }
        }
*/

    }

    pub fn frame_start(&mut self, event_pump: &mut sdl2::EventPump)  {

        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let dt = self.dt();

        self.ui.consume_events(event_pump);

        for event in &self.ui.frame_events {
            self.free_controller.update_events(event);
        }


        self.free_controller.update_camera(&mut self.camera, dt);
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


    /// Default rendering using fbo, cubemap ect if enabled and avaialbe in scene
    pub fn render(&mut self) {

        // DRAW MESHES
        self.mesh_shader.shader.set_used();
        for (key, entity) in &self.entities.data {
            let pos = V3::new(-10.0, -10.0, 0.0);
            let trans = Translation3::from(pos);
            let model_mat = trans.to_homogeneous();


            let uniforms = mesh_shader::Uniforms {
                light_pos: V3::new(0.0, 100.0, 100.0),
                projection: self.camera.projection(),
                model: model_mat,
                view: self.camera.view(),
                view_pos: self.camera.pos(),
                bones: self.bones.get(key).unwrap_or(&self.default_bones)
            };

            self.mesh_shader.set_uniforms(uniforms);

            self.mesh_data[entity.mesh_id].mesh.render(&self.gl);
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
    let id = player.start(Start {anim, repeat: true});

    anim_ids.insert(*entity_id, id);


}

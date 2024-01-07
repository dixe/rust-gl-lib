use crate::{gl};
use crate::imode_gui::drawer2d::*;
use crate::imode_gui::ui::*;
use crate::animations::skeleton::{Bones, Skeleton};
use crate::animations::gltf_animation::{Start, AnimationPlayer, StartTransition};
use crate::objects::gltf_mesh::{self, Animation, KeyFrame};
use crate::shader::{mesh_shader, BaseShader, texture_shader, reload_object_shader, load_object_shader};
use crate::particle_system::emitter;
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

use sdl2::event::{Event, WindowEvent};
use crate::collision3d::CollisionBox;


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
    skeleton: Option<SkeletonIndex>,
    texture_id: Option<texture::TextureId>
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


pub type EntityControllerFn<T> = fn(&mut SceneEntity, &mut Camera, &mut follow_camera::Controller, &Inputs, f32, &T);

pub struct ControlledEntity<T> {
    pub id: EntityId,
    pub user_data: T,
    pub control_fn: EntityControllerFn<T>,
}

pub struct SceneInputs {
    pub free: Inputs,
    pub follow: Inputs,
    pub selected: SceneControllerSelected,
}

impl SceneInputs {
    fn update_events(&mut self, event: &Event) {
        match self.selected {
            SceneControllerSelected::Free => self.free.update_events(event),
            SceneControllerSelected::Follow => self.follow.update_events(event)
        }
    }

    pub fn current_mut(&mut self) -> &mut Inputs {
        match self.selected {
            SceneControllerSelected::Free => &mut self.free,
            SceneControllerSelected::Follow => &mut self.follow,
        }
    }


    pub fn current(&self) -> &Inputs {
        match self.selected {
            SceneControllerSelected::Free => &self.free,
            SceneControllerSelected::Follow => &self.follow,
        }
    }
}

pub struct Scene<UserPostProcessData, UserControllerData =()> {
    pub ui: Ui,
    pub gl: gl::Gl,
    pub ui_mode: bool,

    pub sdl: sdl2::Sdl,

    pub camera: camera::Camera,

    pub inputs: SceneInputs,
    pub follow_controller: follow_camera::Controller,

    // if we use a follow camera and this is set the inputs will be used to control this entity
    // works as a default 3d char controller.
    // for own implementation keep this as none and just do in in user code
    pub controlled_entity: Option<ControlledEntity<UserControllerData>>,

    pub light_pos: V3,

    pub player: AnimationPlayer<EntityId>,

    audio_player: AudioPlayer,

    pub cubemap : Option::<Cubemap>,
    pub cubemap_shader: BaseShader,

    pub clear_buffer_bits: u32,

    pub action_queue: ActionQueue,

    cubemap_imgs: Option::<Arc::<Mutex::<Option<Vec::<image::RgbImage>>>>>,

    pub mesh_shader: mesh_shader::MeshShader,

    // multiple entities can use the same mesh
    pub meshes: HashMap::<Rc::<str>, MeshIndex>, // name to mesh, pt mesh names are uniquie
    mesh_data: Vec::<SceneMesh>, // name to mesh, pt mesh names are uniquie

    // multiple meshes can use the same skeleton, we only need the inverse bind matrix
    pub skeletons: Vec::<Skeleton>,

    pub skeleton_hit_boxes: HashMap::<EntityId, Vec::<CollisionBox>>,

    // animations is linked to skeleton, so have skeletonIndex as key
    pub animations: HashMap::<SkeletonIndex, HashMap::<Rc::<str>, Rc<Animation>>>,

    // Each entity can have one set of bones
    pub bones: HashMap::<EntityId, Bones>,

    //pub animation_ids: HashMap::<EntityId, EntityId>, // Kinda want to get rid of this, and maybe just use entityId as key to animaiton player. Maybe the player should just take an id in Start. This is already out of sync and make root motion buggy;
    pub entities: DataMap::<SceneEntity>,

    pub emitter: emitter::Emitter,

    default_bones: Bones,

    pub fbos: Option::<Fbos<UserPostProcessData>>,

    pub stencil_shader: Option<mesh_shader::MeshShader>,

    pub shadow_map: Option<ShadowMap>,

    pub viewport: gl::viewport::Viewport
}


#[derive(Default)]
pub struct SceneEntity {
    pub mesh_id: MeshIndex,
    // TODO: Maybe have some of this in arrays like data driven, so fx world does not have velocity
    // acceleration ect
    pub pos: V3,
    pub acceleration: V3,
    pub velocity: V3,
    pub target_z_angle: Rotation2<f32>,
    pub z_angle: Rotation2<f32>, // facing angle when char is in t pose

    pub forward_pitch: Rotation2<f32>,
    pub side_pitch: Rotation2<f32>,

    // having pos and root motion make everything easier, since we can just set this in animation update.
    // if we tried to directly add it to pos, then we had to take dt into account, and also somehow make sure that we
    // got every part of every frame, so when dt changes frame, we had to add the last part of old frame root motion
    // ect. ect. this make it easier
    // only really works for animations where we cannot change direction during animaiton,
    pub root_motion: V3,
    pub skeleton_id: Option::<SkeletonIndex>, // Is indirectly a duplicated data, since meshindex points to Scene mesh, which has skel_id. But lets keep it as a convinience
}

impl<UserPostProcessData, UserControllerData> Scene<UserPostProcessData, UserControllerData> {
    pub fn new(gl: gl::Gl, viewport: gl::viewport::Viewport, sdl: sdl2::Sdl) -> Result<Scene<UserPostProcessData, UserControllerData>, failure::Error> {

        let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
        let ui = Ui::new(drawer_2d);
        let mesh_shader = mesh_shader::MeshShader::new(&gl)?;
        let cubemap_shader = load_object_shader("cubemap", &gl).unwrap();
        let player = AnimationPlayer::<EntityId>::new();

        let audio_player = AudioPlayer::new(sdl.audio().unwrap());

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
            sdl,
            ui_mode: true,
            shadow_map: None,
            //render_meshes: vec![],
            ui,
            viewport,
            emitter: emitter::Emitter::new(1000, emitter::emit_1, emitter::update_1),
            camera: camera::Camera::new(viewport.w as f32, viewport.h as f32),
            light_pos: V3::new(0.0, 10.0, 30.0),
            inputs : SceneInputs {
                follow: Default::default(),
                free: Default::default(),
                selected: SceneControllerSelected::Free,
            },
            follow_controller: Default::default(),
            mesh_shader,
            cubemap_shader,
            player,
            audio_player,
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
            controlled_entity: None::<ControlledEntity<UserControllerData>>,
            action_queue: VecDeque::default(),
            skeleton_hit_boxes: Default::default(),
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
            z_angle: Rotation2::identity(),
            forward_pitch: Rotation2::identity(),
            side_pitch: Rotation2::identity(),
            pos: V3::new(0.0, 0.0, 0.0),
            acceleration: V3::new(0.0, 0.0, 0.0),
            velocity: V3::new(0.0, 0.0, 0.0),
            target_z_angle: Rotation2::identity(),
            root_motion: V3::new(0.0, 0.0, 0.0),
            skeleton_id: self.mesh_data[mesh_id].skeleton
        };

        let id = self.entities.insert(entity);

        println!("Added entity {:?} - {} tex={:?}", id, mesh_name, self.mesh_data[mesh_id].texture_id);

        if let Some(s_id) = skeleton_id {
            // set bones
            self.bones.insert(id, self.skeletons[s_id].create_bones());
        }


        id
    }

    pub fn load_sound(&mut self, name: Rc::<str>, path: &str) {
        self.audio_player.add_sound(name.clone(), path);
    }



    pub fn load_all_meshes(&mut self, path: &str, root_motion: bool) {

        // defaults to not split animations into rotation/scale and motion into root motion
        let gltf_data = gltf_mesh::meshes_from_gltf(path, root_motion).unwrap();

        let mut skin_id_to_skel_idx : HashMap::<usize, usize> = HashMap::default();
        for (skin_id, skeleton) in &gltf_data.skins.skeletons {
            self.skeletons.push(skeleton.clone());
            skin_id_to_skel_idx.insert(*skin_id, self.skeletons.len() - 1);
        }


        let mut tex_to_id : HashMap::<usize, texture::TextureId> = HashMap::default();

        for (name, gltf_mesh) in &gltf_data.meshes.meshes {
            let mesh = gltf_mesh.get_mesh(&self.gl);
            // find index into skeletons, if mesh has skeleton
            let skeleton = gltf_data.skins.mesh_to_skin.get(name).map(|skin_id| *skin_id_to_skel_idx.get(skin_id).unwrap());

            let mut texture_id = None;
            if let Some(tex) = gltf_mesh.texture {
                println!("tex for {name}  {:#?}", tex);
                if !tex_to_id.contains_key(&tex) {
                    println!("{:?}", tex);
                    //TODO: Fix this so we load correct texture and not just 0
                    println!("IMAGES: {:?}",gltf_data.images.len());
                    let id = texture::gen_texture_rgba_nearest(&self.gl, &gltf_data.images[0]);
                    tex_to_id.insert(tex, id);
                    // load texture
                }
                texture_id = tex_to_id.get(&tex).map(|id| *id);
            }

            self.mesh_data.push(SceneMesh {
                mesh,
                skeleton,
                texture_id
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
        let mut sm = ShadowMap::new(&self.gl);
        sm.texture_offset = 1;
        self.shadow_map = Some(sm);
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


    pub fn controlled_data_mut(&mut self) -> Option<&mut UserControllerData> {
        self.controlled_entity.as_mut().map(|data| &mut data.user_data)
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
        self.inputs.selected = match self.inputs.selected {
            SceneControllerSelected::Free => SceneControllerSelected::Follow,
            SceneControllerSelected::Follow => SceneControllerSelected::Free
        }
    }

    pub fn allow_char_inputs(&self) -> bool {
        match self.inputs.selected {
            SceneControllerSelected::Free => false,
            SceneControllerSelected::Follow => true
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

        self.inputs.current_mut().frame_start();

        for event in &self.ui.frame_events {
            if !self.ui_mode {
                self.inputs.update_events(event);
            }

            match event {
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    self.ui_mode = !self.ui_mode;

                    self.ui.enabled = self.ui_mode;

                    self.sdl.mouse().show_cursor(self.ui_mode);
                    self.sdl.mouse().set_relative_mouse_mode(!self.ui_mode);
                },
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Tab), .. } => {
                    self.inputs.selected = match self.inputs.selected {
                        SceneControllerSelected::Free => SceneControllerSelected::Follow,
                        SceneControllerSelected::Follow => SceneControllerSelected::Free
                    }
                },
                Event::Window {win_event: WindowEvent::Resized(x,y), ..} => {
                    self.camera.width = *x as f32;
                    self.camera.height = *y as f32;
                    self.viewport.w = *x;
                    self.viewport.h = *y;

                    if let Some(ref mut fbos) = self.fbos {
                        fbos.mesh_fbo.update_viewport(&self.gl, &self.viewport);
                        fbos.ui_fbo.update_viewport(&self.gl, &self.viewport);
                    }
                    self.viewport.set_used(&self.gl);
                },
                _ => {}
            }
        }

        // update particles
        self.emitter.update(dt);

        match self.inputs.selected {
            SceneControllerSelected::Free => free_camera::update_camera(&mut self.camera, dt, &self.inputs.free),
            SceneControllerSelected::Follow => {
                // TODO: Should be a function points or something, we most likely want to disable/ignore movement inputs
                // when fx roll animation is playing
                // but not camera input

                if let Some(entity) = &self.controlled_entity {
                    let e = self.entities.get_mut(&entity.id).unwrap();
                    (entity.control_fn)(e, &mut self.camera, &mut self.follow_controller, &self.inputs.follow, dt, &entity.user_data);
                }


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

    pub fn update_actions(&mut self) {

        while let Some(action) = self.action_queue.pop_front() {
            match action {
                Action::StartAnimation(e_id, name, trans_time) => {

                    let skel = self.entity(&e_id).unwrap().skeleton_id.unwrap();
                    let anim = self.animations.get(&skel).unwrap().get(&name).unwrap();
                    play_animation(anim.clone(), false, &e_id, &mut self.player, &mut self.entities, Some(trans_time));
                },
                Action::StartAnimationLooped(e_id, name, trans_time) => {

                    let skel = self.entity(&e_id).unwrap().skeleton_id.unwrap();
                    let anim = self.animations.get(&skel).unwrap().get(&name).unwrap();
                    play_animation(anim.clone(), true, &e_id, &mut self.player, &mut self.entities, Some(trans_time));
                },
                Action::PlaySound(name) => {
                    self.audio_player.play_sound(&name)
                }
            }
        }
    }

    pub fn update_animations(&mut self) {
        let dt = self.dt();
        self.player.update(dt);

        // update entities skeleton
        for (entity_id, entity) in &mut self.entities.data {
            // update input for controlled entity with animation epxired info
            if let Some(ce) = &self.controlled_entity {
                if ce.id == *entity_id {
                    let inputs = self.inputs.current_mut();
                    inputs.animation_expired = self.player.expired(entity_id);
                }
            }

            if !self.player.expired(entity_id) {
                if let Some(skel_id) = entity.skeleton_id {
                    let skeleton = self.skeletons.get_mut(skel_id).unwrap();
                    let bones = self.bones.get_mut(entity_id).unwrap();
                    self.player.update_skeleton_and_bones(*entity_id, skeleton, bones);

                    // update hitboxes to current skeleton position
                    if let Some(hit_boxes) = self.skeleton_hit_boxes.get_mut(entity_id) {
                        let rotation = Rotation3::from_euler_angles(entity.forward_pitch.angle(),
                                                                    entity.side_pitch.angle(),
                                                                    entity.z_angle.angle());

                        skeleton.update_bone_collision_boxes(hit_boxes, entity.pos , rotation);
                    }
                }

                // update root motion. Maybe have a flag on scene or something to disable this.
                // take current angle into account. Changing angle mid animation also just changes the root motion
                let rot = Rotation3::from_euler_angles(0.0, 0.0, entity.z_angle.angle());
                entity.root_motion = rot * self.player.root_motion(entity_id);
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

            let rotation = Rotation3::from_euler_angles(entity.side_pitch.angle(), entity.forward_pitch.angle(), entity.z_angle.angle());

            render_meshes.push(RenderMesh {
                model_mat: trans.to_homogeneous() * rotation.to_homogeneous(),
                bones: self.bones.get(key).unwrap_or(&self.default_bones),
                mesh: &self.mesh_data[entity.mesh_id].mesh,
                texture: self.mesh_data[entity.mesh_id].texture_id,
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
            sm.post_render(&self.gl, self.viewport.w, self.viewport.h);
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

            let w = self.viewport.w as f32;
            let h = self.viewport.h as f32;

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



fn stop_animation(entity_id: &EntityId,
                  player: &mut AnimationPlayer::<EntityId>,
                  entities: &mut DataMap::<SceneEntity>,
                  create_key_frame: bool) -> Option<KeyFrame> {

    if !player.removed(entity_id) {
        // update entity pos to be pos + root_motion, since root motion will be reset to new anim
        let e = entities.get_mut(entity_id).unwrap();
        // assume that root motion keeps us grounded
        e.pos.x += e.root_motion.x;
        e.pos.y += e.root_motion.y;
        e.root_motion = V3::new(0.0, 0.0, 0.0);

        let res = if create_key_frame {
            player.key_frame(entity_id)
        } else {
            None
        };

        player.remove(*entity_id);

        return res;
    }

    None
}

fn play_animation(anim: Rc::<Animation>, repeat: bool,  entity_id: &EntityId, player: &mut AnimationPlayer::<EntityId>, entities: &mut DataMap::<SceneEntity>, trans_time: Option::<f32>) {

    let key_frame = stop_animation(entity_id, player, entities, trans_time.is_some());

    let transition = key_frame.map(|start_frame| StartTransition {
        start_frame,
        time: trans_time.unwrap()
    });

    player.start(Start {anim, speed: 1.0, repeat, id: *entity_id, transition});
}


pub struct RenderMesh<'a> {
    pub model_mat: Mat4,
    pub mesh: &'a Mesh,
    pub bones: &'a Bones,
    pub texture: Option<texture::TextureId>
}

// Can not be &scene, since then using fbo is not good, and we want a function,
// since we want to call it from mutiple places
fn render_scene(gl: &gl::Gl, camera: &Camera,
                mesh_shader: &mesh_shader::MeshShader,
                _mesh_data: &Vec::<SceneMesh>,
                _bones: &HashMap::<EntityId, Bones>,
                default_bones: &Bones,
                cubemap_opt: &Option<Cubemap>,
                cubemap_shader: &BaseShader,
                stencil_shader: &Option<mesh_shader::MeshShader>,
                _shadow_map: &Option<ShadowMap>,
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
    mesh_shader.shader.set_i32(gl, "Texture", 0);
    mesh_shader.shader.set_i32(gl, "shadowMap", 1);

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

/// Simple controller for moving around
/// and updating follow camera
pub fn base_controller<T>(entity: &mut SceneEntity, camera: &mut Camera, follow_controller: &mut follow_camera::Controller, inputs: &Inputs, dt: f32, _user_data: &T) {

    // update player pos
    let mut d = entity.pos - camera.pos;
    d.z = 0.0;
    d = d.normalize();
    let t = V3::new(d.y, -d.x, 0.0);

    let mut m = d * inputs.movement.x + t * inputs.movement.y;

    entity.forward_pitch = Rotation2::new(0.0);
    entity.side_pitch = Rotation2::new(0.0);
    if m.magnitude() > 0.0 {

        m = m.normalize(); // check sekrio what happens when holding right or left
        // ignore z since we assume its a char controller that cannot fly

        let new_angle = m.y.atan2(m.x);
        let mut diff = new_angle - entity.z_angle.angle();

        // normalize to range -pi to pi
        if diff < -std::f32::consts::PI {
            diff += std::f32::consts::TAU;
        }

        if diff > std::f32::consts::PI {
            diff -= std::f32::consts::TAU;
        }

        let sign = diff.signum();
        let r_speed = 10.0;
        // change with max rotation speed
        let mut change = sign * r_speed * dt;

        // if we max speed would over shot target angle, change just the needed amount
        if change.abs() > diff.abs() {
            change = diff;
        }

        // do the update of rotation
        let z_rot = Rotation2::new(change);
        entity.z_angle *= z_rot;

        entity.forward_pitch = Rotation2::new(0.2 * inputs.movement.x as f32 );
        entity.side_pitch = Rotation2::new(0.2 * inputs.movement.y as f32 );

        entity.pos += m * inputs.speed * dt;
    }


    //Update camera

    let base_sens = 3.0;

    follow_controller.update_dist(inputs.mouse_wheel);

    follow_controller.desired_pitch += inputs.mouse_movement.yrel * inputs.sens * inputs.inverse_y *  dt * base_sens;

    follow_controller.yaw_change = inputs.mouse_movement.xrel * inputs.sens * dt * base_sens;

    follow_controller.update_camera_target(entity.pos + entity.root_motion);

    follow_controller.update_camera(camera, dt);
}



pub type ActionQueue = VecDeque::<Action>;

// Generic actions, so StartAnimation, Plays sound
// and not Attack, Roll ect.
pub enum Action {
    // TODO: Maybe don't use string, but use something morel lgiht weight like Rc::<str> or Anim or ids
    // but should still be easy for the user
    StartAnimation(EntityId, Rc::<str>, f32),
    StartAnimationLooped(EntityId, Rc::<str>, f32),
    PlaySound(Rc::<str>),
    //SpawnParticle(String"name", loc, other info if needed)
}

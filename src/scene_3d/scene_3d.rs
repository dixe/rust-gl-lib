use crate::{gl};
use crate::imode_gui::ui::*;
use crate::animations::skeleton::{Bones, Skeleton};
use crate::animations::gltf_animation::{Start, AnimationPlayer, StartTransition};
use crate::objects::gltf_mesh::{self, Animation, KeyFrame};
use crate::shader::{BaseShader, texture_shader};
use crate::particle_system::{emitter};
use crate::typedef::*;
use crate::texture;
use crate::objects::{mesh::Mesh, cubemap::{self, Cubemap}};
use crate::camera::{self, free_camera, follow_camera, Camera};
use crate::na::{Rotation3, Rotation2};
use crate::{buffer, movement::Inputs};
use crate::audio::audio_player::AudioPlayer;
use std::{thread, sync::{Arc, Mutex}};
use std::rc::Rc;
use std::collections::{VecDeque, HashMap};
use crate::helpers;
use sdl2::event::{Event, WindowEvent};
use crate::collision3d::CollisionBox;
use crate::color::Color;
use crate::scene_3d::types::DataMap;
use crate::scene_3d::actions::*;
use crate::scene_3d::RenderPipelines;
use crate::scene_3d::RenderPipelineId;
use crate::scene_3d::ParticleScene;


pub type EntityId = usize;
pub type MeshIndex = usize;
pub type SkeletonIndex = usize;


pub struct SceneMesh {
    pub mesh: Mesh,
    pub skeleton: Option<SkeletonIndex>,
    pub texture_id: Option<texture::TextureId>
}


pub type PostProcessUniformSet<T> = fn(&gl::Gl, &mut BaseShader, &T);

pub struct Fbos<UserPostprocesData> {
    pub mesh_fbo: buffer::FrameBuffer,
    pub ui_fbo: buffer::FrameBuffer,
    pub post_process_shader: texture_shader::TextureShader,
    pub post_process_uniform_set: PostProcessUniformSet<UserPostprocesData>,
    pub post_process_data: UserPostprocesData
}

#[derive(Debug)]
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

pub struct Scene<UserPostProcessData, UserControllerData = ()> {
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
    pub light_color: Color,

    pub player: AnimationPlayer<EntityId>,

    audio_player: AudioPlayer,

    pub cubemap : Option::<Cubemap>,

    pub clear_buffer_bits: u32,

    pub action_queue: ActionQueue,

    cubemap_imgs: Option::<Arc::<Mutex::<Option<Vec::<image::RgbImage>>>>>,

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

    pub emitter: emitter::Emitter<ParticleScene>,

    default_bones: Bones,

    pub fbos: Option::<Fbos<UserPostProcessData>>,

    pub viewport: gl::viewport::Viewport,

    pub render_pipelines: RenderPipelines<UserPostProcessData>,

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

    pub render_pipeline_id: RenderPipelineId
}






impl<UserPostProcessData, UserControllerData> Scene<UserPostProcessData, UserControllerData> {
    pub fn new(sdl_setup: &mut helpers::BasicSetup) -> Result<Scene<UserPostProcessData, UserControllerData>, failure::Error> {
        let gl = sdl_setup.gl.clone();
        let viewport = sdl_setup.viewport;
        let ui = sdl_setup.ui();
        let sdl = sdl_setup.sdl.clone();

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


        let mut camera = camera::Camera::new(viewport.w as f32, viewport.h as f32);
        let look_at = V3::new(0.0, 0.0, 0.0);

        camera.move_to(V3::new(10.4, 0.0, 5.0));
        camera.look_at(look_at);


        Ok(Self {
            render_pipelines: RenderPipelines::new(gl.clone())?,
            gl,
            sdl,
            ui_mode: true,
            ui,
            viewport,
            emitter: emitter::Emitter::new(1000, |_, _, _| {}, |_, _,| {}),
            camera,
            light_pos: V3::new(0.0, 10.0, 30.0),
            light_color: Color::Rgb(255, 255, 255),
            inputs : SceneInputs {
                follow: Default::default(),
                free: Default::default(),
                selected: SceneControllerSelected::Free,
            },
            follow_controller: Default::default(),
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
            skeleton_id: self.mesh_data[mesh_id].skeleton,
            render_pipeline_id: 0
        };

        let id = self.entities.insert(entity);

        //println!("Added entity {:?} - {} tex={:?}", id, mesh_name, self.mesh_data[mesh_id].texture_id);

        if let Some(s_id) = skeleton_id {
            // set bones
            self.bones.insert(id, self.skeletons[s_id].create_bones());
        }


        id
    }

    pub fn remove_entity(&mut self, id: &EntityId) {

        if let Some(_e) = self.entities.remove(id) {
            // entity e removed, will be destroyed at end of this scope
            self.bones.remove(id);
        }
    }

    pub fn load_sound(&mut self, name: Rc::<str>, path: &str) {
        self.audio_player.add_sound(name.clone(), path);
    }

    pub fn set_entity_render_pipeline(&mut self, id: EntityId, name: Rc::<str>) {
        if let Some(entity) = self.entities.get_mut(&id) {
            if let Some(pipe_id) = self.render_pipelines.id(name) {
                entity.render_pipeline_id = pipe_id;
            }
        }
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

    pub fn controlled_data_mut(&mut self) -> Option<&mut UserControllerData> {
        self.controlled_entity.as_mut().map(|data| &mut data.user_data)
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

        self.ui.start_frame(event_pump);

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
                    if let Some(e) = self.entities.get_mut(&entity.id) {
                        (entity.control_fn)(e, &mut self.camera, &mut self.follow_controller, &self.inputs.follow, dt, &entity.user_data);
                    }
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

        //TODO: have a playing/pause bool
        self.update_actions();
        self.update_animations();
    }


    pub fn frame_end(&mut self) {
        self.ui.end_frame();
    }

    pub fn update_actions(&mut self) {

        while let Some(action) = self.action_queue.pop_front() {
            match action {
                Action::StartAnimation(e_id, name, trans_time) => {

                    let skel = self.entity(&e_id).unwrap().skeleton_id.unwrap();
                    let anim = self.animations.get(&skel).unwrap().get(&name).expect(&format!("Animation {:?} was expected for {:?}", name, e_id));
                    play_animation(anim.clone(), false, &e_id, &mut self.player, &mut self.entities, Some(trans_time));
                },
                Action::StartAnimationLooped(e_id, name, trans_time) => {
                    if let Some(skel) = self.entity(&e_id).and_then(|x| x.skeleton_id) {
                        let anim = self.animations.get(&skel).unwrap().get(&name).unwrap();
                        play_animation(anim.clone(), true, &e_id, &mut self.player, &mut self.entities, Some(trans_time));
                    }
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


        self.render_pipelines.render(
            &self.mesh_data,
            &self.camera,
            self.light_pos,
            self.light_color,
            &mut self.ui,
            &self.viewport,
            &self.bones,
            &self.default_bones,
            &self.entities.data,
            &self.emitter
        );


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

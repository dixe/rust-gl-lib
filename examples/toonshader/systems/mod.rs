use gl_lib::{gl, helpers};
use gl_lib::scene_3d as scene;
use gl_lib::color::Color;
use gl_lib::typedef::V3;
use gl_lib::shader;
use gl_lib::scene_3d::EntityId;
use gl_lib::camera::{follow_camera, Camera};
use gl_lib::movement::Inputs;
use gl_lib::na::{Rotation2};
use gl_lib::scene_3d::actions;
use sdl2::event::Event;
use gl_lib::scene_3d::ParticleScene;
use crate::Scene;
use crate::Unit;


pub mod missile;
pub mod death;


pub type SystemFn = fn(&mut GameData, &mut Scene);

#[derive(Debug, Default)]
pub struct GameData {
    pub enemies: Vec::<Unit>,
    pub missiles: Vec::<missile::Missile>,
}


pub fn setup_systems() -> Vec::<SystemFn> {
    vec![death::death_system, missile::missile_system]
}

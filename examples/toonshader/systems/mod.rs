use crate::Scene;
use crate::Unit;
use std::rc::Rc;
use gl_lib::scene_3d::MeshIndex;
use std::collections::HashMap;

pub mod missile;
pub mod unit;
pub mod auto_attack;
pub mod death;
pub mod goap_ai;
pub mod cooldown;

pub type SystemFn = fn(&mut GameData, &mut Scene);

#[derive(Debug, Default)]
pub struct GameData {
    pub units: Vec::<Unit>,
    pub missiles: Vec::<missile::Missile>,
    pub goap_datas: Vec::<goap_ai::GoapData>,
    pub mesh_index_to_attack_name: HashMap::<MeshIndex, Rc::<str>>
}


pub fn setup_systems() -> Vec::<SystemFn> {
    vec![cooldown::cooldown_system, auto_attack::auto_attack_system, missile::missile_system, death::death_system]
}

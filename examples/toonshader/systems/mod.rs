use crate::Scene;
use std::rc::Rc;
use gl_lib::scene_3d::MeshIndex;
use std::collections::HashMap;
use gl_lib::scene_3d::EntityId;
use crate::goap_ai::EntityAiFn;

pub mod missile;
pub mod unit;
pub mod auto_attack;
pub mod death;
pub mod goap_ai;
pub mod cooldown;

pub type SystemFn = fn(&mut GameData, &mut Scene);

#[derive(Debug, Default)]
pub struct GameData {
    pub units: Vec::<unit::Unit>,
    pub units_data: HashMap::<EntityId, unit::UnitData>,
    pub missiles: Vec::<missile::Missile>,
    pub goap_datas: Vec::<goap_ai::GoapData>,
    pub mesh_index_to_attack_name: HashMap::<MeshIndex, Rc::<str>>,
    pub goap_action_to_fn: HashMap::<Rc::<str>, EntityAiFn>
}


pub fn setup_systems() -> Vec::<SystemFn> {
    vec![cooldown::cooldown_system,
         goap_ai::update_senses,
         goap_ai::check_current_plan,
         goap_ai::goap_plan_system,
         goap_ai::execute_goal_system,
         //auto_attack::auto_attack_system,
         missile::missile_system,
         death::death_system]
}

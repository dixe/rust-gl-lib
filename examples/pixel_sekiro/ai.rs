use crate::entity::Entity;
use rand::prelude::*;

pub fn skeleton_logic(entity: &mut Entity, ) {
    let attack_r = rand::random::<f32>();

    if attack_r > 0.8 {
        entity.inputs.set_attack();
    }
}

use crate::entity::{Entity, EntityState};
use rand::prelude::*;

pub fn skeleton_logic(entity: &mut Entity, ) {
    let attack_r = rand::random::<f32>();

    match entity.state {
        EntityState::Idle(_) => {
            if attack_r > 0.9 {
                //entity.active_combo = (entity.active_combo + 1) % 2;
            }
            else if attack_r > 0.8 {
                entity.inputs.set_attack();
            }
        },
        EntityState::Attack(_) => {

            //println!("{:?}", (entity.combos[entity.active_combo].attacks, entity.attack_counter));
            // always try to finish a combo
            if entity.attack_counter <= entity.combos[entity.active_combo].attacks {
                entity.inputs.set_attack();
            }
        },
        _ => {}
    }

}

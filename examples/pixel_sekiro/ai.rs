use crate::entity::{Entity, EntityState};


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
        EntityState::AttackDamage(_) => {

            //println!("{:?}", (entity.combos[entity.active_combo].attacks, entity.attack_counter));
            // always try to finish a combo
            if entity.attack_counter <= entity.combos[entity.active_combo].attacks {
                entity.inputs.set_attack();
            }
        },
        EntityState::Recover(_) => {
            // if recovering and in combo, we want to finish combo
            if entity.attack_counter > 0 && entity.attack_counter <= entity.combos[entity.active_combo].attacks {
                entity.inputs.set_attack();
            }
        },
        _ => {}
    }

}

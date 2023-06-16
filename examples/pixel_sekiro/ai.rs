use crate::entity::{Entity, EntityState};


pub fn skeleton_logic(entity: &mut Entity, ) {
    let attack_r = rand::random::<f32>();

    match entity.state {
        EntityState::Idle(_) => {

            // wait atleast 2 sec between attacks
            if attack_r > 0.9 && entity.last_attack_time.elapsed().as_millis() > 2000 {
                entity.inputs.set_attack();
            }
        },
        EntityState::AttackDeflected(_, _) => {
            if entity.has_next_combo_attack() {
                entity.inputs.set_attack();
            }
        },
        EntityState::AttackDamage(_) => {
            if entity.has_next_combo_attack() {
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

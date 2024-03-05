use gl_lib::scene_3d::actions;
use crate::Scene;
use crate::GameData;
use crate::systems::unit::UnitSystem;
use gl_lib::scene_3d::EntityId;


pub trait DeathSystem {
    fn on_death(&mut self, id: EntityId, scene: &mut Scene) -> bool;
    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool;
}

pub fn death_system(game: &mut (impl DeathSystem + UnitSystem), scene: &mut Scene) {

    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let u = game.unit(i);
        let id = u.id;
        let unit = game.unit_data_mut(id);

        let mut update = true;

        // if alive and get lower than 0 hp play dead anim
        if !unit.dead && unit.hp <= 0.0 {
            update = game.on_death(id, scene);
        } else if unit.dead {
            update = game.update_dead(i, scene);
        }

        if update {
            i += 1;
        }
    }
}


impl DeathSystem for GameData {

    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool {

        let unit = self.unit(idx);

        let id = unit.id;

        // wait for animation to be done.
        if scene.player.expired(&id) {
            // remove unit
            scene.remove_entity(&id);
            self.units.swap_remove(idx);

            // remove unit data
            self.units_data.remove(&id);

            return false;
        }
        true
    }


    fn on_death(&mut self, id: EntityId, scene: &mut Scene) -> bool {
        let unit = self.units_data.get_mut(&id).expect("Death system should not have called with idx outside scope");
        // TODO: invalidate the units goal and plan, might help us


        // set dead
        unit.dead = true;
        // start death anim
        scene.action_queue.push_back(actions::Action::StartAnimation(id, "death".into(), 0.0));
        true
    }
}

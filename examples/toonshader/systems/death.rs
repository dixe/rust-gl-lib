use gl_lib::scene_3d::actions;
use crate::Scene;
use crate::GameData;
use crate::systems::unit::UnitSystem;

pub trait DeathSystem {
    fn on_death(&mut self, idx: usize, scene: &mut Scene) -> bool;
    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool;
}


pub fn death_system(game: &mut (impl DeathSystem + UnitSystem), scene: &mut Scene) {

    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let unit = game.unit(i);

        let mut update = true;

        // if alive and get lower than 0 hp play dead anim
        if !unit.dead && unit.hp <= 0.0 {
            update = game.on_death(i, scene);
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
        let unit = self.units.get_mut(idx).expect("Death system should not have called with idx outside scope");

        if scene.player.expired(&unit.id) {
            // remove unit
            scene.remove_entity(&unit.id);
            self.units.swap_remove(idx);
            return false;
        }

        true
    }


    fn on_death(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let unit = self.units.get_mut(idx).expect("Death system should not have called with idx outside scope");

        // set dead
        unit.dead = true;
        // start death anim
        scene.action_queue.push_back(actions::Action::StartAnimation(unit.id, "death".into(), 0.0));
        true
    }
}

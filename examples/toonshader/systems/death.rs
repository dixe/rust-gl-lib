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
use crate::GameData;

pub trait DeathSystem {
    fn units(&self) -> usize;
    fn unit(&mut self, idx: usize) -> &mut Unit; // should be some kind of Hp trait impl
    fn on_death(&mut self, idx: usize, scene: &mut Scene) -> bool;
    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool;
}


pub fn death_system(game: &mut impl DeathSystem, scene: &mut Scene) {

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
    fn units(&self) -> usize {
        self.enemies.len()
    }

    fn unit(&mut self, idx: usize) -> &mut Unit {
        self.enemies.get_mut(idx).expect("Death system should not have called with idx outside scope")
    }

    fn update_dead(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let unit = self.enemies.get_mut(idx).expect("Death system should not have called with idx outside scope");

        if scene.player.expired(&unit.id) {
            // remove unit
            scene.remove_entity(&unit.id);
            self.enemies.swap_remove(idx);
            return false;
        }

        true
    }


    fn on_death(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let unit = self.enemies.get_mut(idx).expect("Death system should not have called with idx outside scope");

        // set dead
        unit.dead = true;
        // start death anim
        scene.action_queue.push_back(actions::Action::StartAnimation(unit.id, "death".into(), 0.0));
        true
    }
}

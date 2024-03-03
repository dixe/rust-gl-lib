use gl_lib::scene_3d as scene;
use gl_lib::scene_3d::EntityId;
use gl_lib::scene_3d::ParticleScene;
use crate::Scene;
use crate::GameData;
use crate::systems::unit::UnitSystem;
use crate::systems::auto_attack;
use gl_lib::typedef::V3;
use gl_lib::scene_3d::actions;
use gl_lib::goap;
use std::rc::Rc;
use crate::systems::goap_ai::GoapSystem;
use crate::systems::missile::{self, MissileSystem};

pub fn attack(id: EntityId, game: &mut GameData, scene: &mut Scene) {
    // find nearet enemy
    let this = match game.units.iter().filter(|x| x.id == id).nth(0) {
        Some(t) => t,
        None => {
            return;
        }
    };

    let dt = scene.dt();
    if let Some(closest) = auto_attack::find_closest_enemy(this, game, &scene.entities) {
        if closest.dist < this.range && this.cooldown <= 0.0 {

            scene.action_queue.push_back(actions::Action::StartAnimation(this.id, "attack".into(), 0.0));
            scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));

            let arrow_id = scene.create_entity("arrow");
            scene::update_pos(scene, arrow_id, closest.this_pos + closest.dir * 0.1);

            game.spawn_missile(missile::Missile {id: arrow_id, target_id: closest.target.id });

            // set cool down
            let unit = game.units.iter_mut().filter(|x| x.id == id).nth(0).unwrap();
            unit.cooldown = 5.0;

        }
    }

    //set_action_complete(id, game);
}


fn set_action_complete(id: EntityId, game: &mut GameData) {
    if let Some(goap) = game.goap_data_by_entity_id(id) {

        let action = match &goap.next_action {
            Some(a) => a,
            None => {
                return;
            }
        };


        for (post, _) in &action.post {
            goap.state.insert(post.clone(), true);
        }

        goap.goal = None;
        goap.next_action = None;
    }
}

pub fn go_to_player(id: EntityId, game: &mut GameData, scene: &mut Scene) {
    // find nearet enemy
    let this = match game.units.iter().filter(|x| x.id == id).nth(0) {
        Some(t) => t,
        None => {
            return;
        }
    };

    let dt = scene.dt();
    if let Some(closest) = auto_attack::find_closest_enemy(this, game, &scene.entities) {
        // move to target


        let inc = dt * 5.0;
        let _ = scene.entities.get_mut(&id).map(|e| {
            e.pos += closest.dir * inc;
        });

        let dist = closest.dist - inc;


        // update if in range, remove action, since it is complete
        if dist <= this.range {
            set_action_complete(id, game);
        }
    }
}

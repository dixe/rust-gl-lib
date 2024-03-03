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
use crate::systems::goap_ai::{self, GoapData, GoapSystem};
use crate::systems::missile::{self, MissileSystem};



pub fn attack(id: EntityId, game: &mut GameData, scene: &mut Scene) -> Option<()> {
    let goap = game.goap_data_by_entity_id(id)?;

    if let Some(target) = &goap.senses.target {

    } else {
        // target not active, invalidate action
        //invalidate_action(goap);
        return None;
    }


    // find nearet enemy
    let this = game.units_data.get(&id)?;
    // check that current target is alive and in range, otherwise we need to invalidate the plan


    panic!("Should not find enemy");
    let dt = scene.dt();
    if let Some(closest) = auto_attack::find_closest_enemy(id, game, &scene.entities) {
        if closest.dist < this.range && this.cooldown <= 0.0 {

            scene.action_queue.push_back(actions::Action::StartAnimation(this.id, "attack".into(), 0.0));
            scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));

            let arrow_id = scene.create_entity("arrow");
            scene::update_pos(scene, arrow_id, closest.this_pos + closest.dir * 0.1);

            game.spawn_missile(missile::Missile {id: arrow_id, target_id: closest.target });

            // set cool down
            let unit = game.units_data.get_mut(&id).unwrap();
            unit.cooldown = 5.0;

        }
    }

    //set_action_complete(id, game);

    Some(())
}


fn invalidate_action(goap: &mut GoapData) {

    goap.goal = None;
    goap.plan.clear();
}

fn set_action_complete(id: EntityId, game: &mut GameData) -> Option<()> {
    let goap = game.goap_data_by_entity_id(id)?;

    let action = &goap.plan.last().unwrap(); // Do we know that we have atleast 1 action in our plan?
    println!("Completed {:?}", action.name);
    for (post, val) in &action.post {
        println!("setting {:?} = {:?}", post, val);
        goap.state.insert(post.clone(), *val);
    }


    // action complete might not complete the goal
    goap.goal = None;
    goap.plan.clear();

    Some(())
}


pub fn acquire_target(id: EntityId, game: &mut GameData, scene: &mut Scene) -> Option<()> {

    let dt = scene.dt();
    if let Some(closest) = auto_attack::find_closest_enemy(id, game, &scene.entities) {
        // update senses and complete action

        if let Some(goap) = game.goap_data_by_entity_id(id) {
            goap.senses.target = Some(goap_ai::Target{ id: closest.target, pos: closest.target_pos} );
        }

        set_action_complete(id, game);
    }

    None
}

pub fn go_to_target(id: EntityId, game: &mut GameData, scene: &mut Scene) -> Option<()> {

    let dt = scene.dt();
    if let Some(goap) = game.goap_data_by_entity_id(id) {
        // move to target

        if let Some(target) = &goap.senses.target {

            println!("{:?}", (goap.senses.pos_self, target.pos));
            let mut dir = goap.senses.pos_self - target.pos;

            if dir.magnitude() > 0.0 {
                dir = dir.normalize();
            }

            // TODO: This updates entity pos. maybe should update vel and have a physics system move entities??
            // Maybe this is ok
            let inc = dt * 5.0;
            let _ = scene.entities.get_mut(&id).map(|e| {
                println!("{:?}", e.pos);
                e.pos += dir * inc;
            });

        } else {
            invalidate_action(goap)
        }
    }

    Some(())
}



pub fn empty(_: EntityId, _: &mut GameData, _: &mut Scene) -> Option<()> {
    None
}

use gl_lib::scene_3d as scene;
use gl_lib::scene_3d::EntityId;
use gl_lib::scene_3d::ParticleScene;
use crate::Scene;
use crate::GameData;
use crate::systems::unit::{self, UnitSystem};
use gl_lib::typedef::V3;
use gl_lib::scene_3d::actions;
use crate::missile;
use crate::missile::MissileSystem;
use gl_lib::scene_3d::SceneEntity;
use gl_lib::scene_3d::DataMap;


pub fn auto_attack_system(game: &mut (impl UnitSystem + MissileSystem), scene: &mut Scene) {

    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let u = game.unit(i);
        let unit = game.unit_data(u.id);

        if unit.cooldown > 0.0 || unit.dead {
            i += 1;
            continue;
        }

        let id = u.id;

        if let Some(closest) = find_closest_enemy(id, game, &scene.entities) {
            if closest.dist < unit.range {
                //println!("{:?} auto attack {:?} at range: {:?}", unit.id, closest.target.id, closest.dist);

                scene.action_queue.push_back(actions::Action::StartAnimation(unit.id, "attack".into(), 0.0));
                scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));

                let id = scene.create_entity("arrow");
                scene::update_pos(scene, id, closest.this_pos + closest.dir * 0.1);

                game.spawn_missile(missile::Missile {id, target_id: closest.target });


                // set cool down
                let unit = game.unit_data_mut(id);
                unit.cooldown = 5.0;

            }
        }

        i += 1;
    }
}


pub struct ClosestEnemy {
    pub target: EntityId,
    pub dist: f32,
    pub this_pos: V3,
    pub target_pos: V3,
    pub dir: V3,
}

// this is a general function, that can be usefull in many systems
/// Find closest enemy that is not dead. But it might not be in range
pub fn find_closest_enemy(id: EntityId, game: &impl UnitSystem, entities: &DataMap::<SceneEntity>) -> Option::<ClosestEnemy> {
    let mut closest = None;
    let mut min = f32::MAX;
    let mut i = 0;

    let this_pos = entities.get(&id).expect("unit from trait UnitSystem should exist in scene").pos;

    let this = game.unit_data(id).clone();

    while i < game.units() { // use while loop so we can modify during loop
        let u = game.unit(i);
        let unit = game.unit_data(u.id);


        if unit.hp > 0.0 && unit.team != this.team {
            let unit_pos = entities.get(&unit.id).expect("unit from trait UnitSystem should exist in scene").pos;
            let mut dir = unit_pos - this_pos;
            let dist = dir.magnitude();
            if dist > 0.0 {
                dir = dir.normalize();
            }

            if dist < min {
                min = dist;
                closest = Some(ClosestEnemy{
                    target: id,
                    dist,
                    this_pos,
                    target_pos: unit_pos,
                    dir
                });
            }
        }
        i += 1;

    }
    return closest;
}

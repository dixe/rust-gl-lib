use gl_lib::scene_3d as scene;
use gl_lib::scene_3d::EntityId;
use gl_lib::scene_3d::ParticleScene;
use crate::Scene;
use crate::GameData;
use crate::Unit;
use crate::systems::unit::UnitSystem;
use gl_lib::typedef::V3;
use gl_lib::scene_3d::actions;
use crate::missile;
use crate::missile::MissileSystem;
use gl_lib::scene_3d::SceneEntity;
use gl_lib::scene_3d::DataMap;


pub fn auto_attack_system(game: &mut (impl UnitSystem + MissileSystem), scene: &mut Scene) {

    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let this = game.unit(i);

        if this.cooldown > 0.0 {
            i += 1;
            continue;
        }

        if let Some(closest) = find_closest_enemy(&this, game, &scene.entities) {
            if closest.dist < this.range {

                //println!("{:?} auto attack {:?} at range: {:?}", this.id, closest.target.id, closest.dist);


                scene.action_queue.push_back(actions::Action::StartAnimation(this.id, "attack".into(), 0.0));
                scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));

                let id = scene.create_entity("arrow");
                scene::update_pos(scene, id, closest.this_pos + closest.dir * 0.1);

                game.spawn_missile(missile::Missile {id, target_id: closest.target.id });


                // set cool down
                let unit = game.unit_mut(i);
                unit.cooldown = 5.0;

            }
        }

        i += 1;
    }
}


pub struct ClosestEnemy {
    pub target: Unit,
    pub dist: f32,
    pub this_pos: V3,
    pub target_pos: V3,
    pub dir: V3,
}

// this is a general function, that can be usefull in many systems
/// Find closest enemy that is not dead. But it might not be in range
pub fn find_closest_enemy(this: &Unit, game: &impl UnitSystem, entities: &DataMap::<SceneEntity>) -> Option::<ClosestEnemy> {
    let mut closest = None;
    let mut min = f32::MAX;
    let mut i = 0;
    let this_pos = entities.get(&this.id).expect("unit from trait UnitSystem should exist in scene").pos;

    while i < game.units() { // use while loop so we can modify during loop
        let unit = game.unit(i);

        if unit.hp > 0.0 && unit.team != this.team {
            let unit_pos = entities.get(&unit.id).expect("unit from trait UnitSystem should exist in scene").pos;
            let mut dir = this_pos - unit_pos;
            let dist = dir.magnitude();
            if dist > 0.0 {
                dir = dir.normalize();
            }

            if dist < min {
                min = dist;
                closest = Some(ClosestEnemy{
                    target: *unit,
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

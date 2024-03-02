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


pub trait AutoAttackSystem {
    fn start_attack_animation(&self, entity_id: &EntityId, scene: &mut Scene) ;
}

pub fn auto_attack_system(game: &mut (impl UnitSystem + MissileSystem + AutoAttackSystem), scene: &mut Scene) {

    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let this = game.unit(i);

        if let Some(closest) = find_closest_enemy(&this, game, &scene.entities) {
            if closest.dist < this.range {

                //println!("{:?} auto attack {:?} at range: {:?}", this.id, closest.target.id, closest.dist);

                // TODO: set attack cooldown to start

                scene.action_queue.push_back(actions::Action::StartAnimation(this.id, "attack".into(), 0.0));
                scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));

                let id = scene.create_entity("arrow");
                scene::update_pos(scene, id, closest.this_pos + closest.dir * 0.1);

                game.spawn_missile(missile::Missile {id, target_id: closest.target.id });

            }
        }

        i += 1;
    }
}

impl AutoAttackSystem for GameData {
    fn start_attack_animation(&self, entity_id: &EntityId, scene: &mut Scene) {

        if let Some(entity) = scene.entity(entity_id) {
            if let Some(name) = self.mesh_index_to_attack_name.get(&entity.mesh_id) {
                scene.action_queue.push_back(actions::Action::StartAnimation(*entity_id, name.clone(), 0.0));
                scene.action_queue.push_back(actions::Action::PlaySound("attack".into()));
            }

        }
    }
}

pub struct ClosestEnemy {
    target: Unit,
    dist: f32,
    this_pos: V3,
    target_pos: V3,
    dir: V3,
}

// this is a general function, that can be usefull in many systems
pub fn find_closest_enemy(this: &Unit, game: &impl UnitSystem, entities: &DataMap::<SceneEntity>) -> Option::<ClosestEnemy> {
    let mut closest = None;
    let mut min = f32::MAX;
    let mut i = 0;
    let this_pos = entities.get(&this.id).expect("unit from trait UnitSystem should exist in scene").pos;

    while i < game.units() { // use while loop so we can modify during loop
        let unit = game.unit(i);

        if unit.team != this.team {
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


/*
impl AutoAttackSystem for GameData {
    fn attacks(&self) -> usize {
        self.attacks.len()
    }

    fn attack(&mut self, idx: usize) -> &mut AutoAttack {
        self.attacks.get_mut(idx).expect("Death system should not have called with idx outside scope")
    }

    fn on_attack_hit(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let m = self.attacks[idx];

        self.attacks.swap_remove(idx);

        // remove from scene
        scene.remove_entity(&m.id);


        // apply damage, if enemy dies, it will get handled by the death system.
        if let Some(enemy) = self.enemies.iter_mut().find(|e| e.id == m.target_id) {
            enemy.hp -= 1.0;
        }

        // damage particle on enemy on hit
        if let Some(target) = scene.entities.get_mut(&m.target_id) {
            // damage mesh
            scene.emitter.emit_new(ParticleScene {
                life: 0.3,
                total_life: 0.3,
                pos: target.pos,
                mesh_id: *scene.meshes.get("Damage".into()).unwrap(),
                render_pipeline_id: 1
            });
        }

        // maybe a attack bounces to next target,
        // and we decrement bounce life, and then
        // we return false unti bounces or not target reached.
        // For now just true since we remove from attacks
        true
    }
}
*/

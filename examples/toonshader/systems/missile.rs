use gl_lib::scene_3d as scene;
use gl_lib::scene_3d::EntityId;
use gl_lib::scene_3d::ParticleScene;
use crate::Scene;
use crate::GameData;

#[derive(Debug, Default, Clone, Copy)]
pub struct Missile {
    pub id: EntityId,
    pub target_id: EntityId,
}



pub trait MissileSystem {
    /// Return number of missiles used for loop
    fn missiles(&self) -> usize;

    /// Return mut missile for given index in loop
    fn missile(&mut self, idx: usize) -> &mut Missile; // should be some kind of missile trait

    fn spawn_missile(&mut self, missile: Missile);

    /// Call impl for on hit for given missile idx, return bool indicating whether the missile was remove or not.
    /// Used to continue loop correctly
    fn on_missile_hit(&mut self, idx: usize, scene: &mut Scene) -> bool;
}


impl MissileSystem for GameData {
    fn missiles(&self) -> usize {
        self.missiles.len()
    }

    fn missile(&mut self, idx: usize) -> &mut  Missile {
        self.missiles.get_mut(idx).expect("Death system should not have called with idx outside scope")
    }

    fn spawn_missile(&mut self, missile: Missile) {
        self.missiles.push(missile);
    }

    fn on_missile_hit(&mut self, idx: usize, scene: &mut Scene) -> bool {
        let m = self.missiles[idx];

        self.missiles.swap_remove(idx);

        // remove from scene
        scene.remove_entity(&m.id);

        // apply damage, if enemy dies, it will get handled by the death system.

        if let Some(enemy) = self.units_data.get_mut(&m.target_id) {
            enemy.hp -= 1.0;
        }

        // damage particle on enemy on hit
        if let Some(target) = scene.entities.get_mut(&m.target_id) {
            // damage mesh
            scene.emitter.emit_new(ParticleScene {
                life: 0.3,
                total_life: 0.3,
                pos: target.pos,
                mesh_id: *scene.meshes.get("Damage").unwrap(),
                render_pipeline_id: 1
            });
        }

        // maybe a missile bounces to next target,
        // and we decrement bounce life, and then
        // we return false unti bounces or not target reached.
        // For now just true since we remove from missiles
        true
    }
}

pub fn missile_system(game: &mut impl MissileSystem, scene: &mut Scene) {
    let speed = 20.0;
    let dt = scene.dt();
    let mut i = 0;
    while i < game.missiles() { // use while loop so we can modify during loop

        let m = game.missile(i);

        let missile = scene.entities.get(&m.id).unwrap();
        // TODO: this can fail, if the target is dead and gone
        if let Some(target) = scene.entities.get(&m.target_id) {


            let dir = target.pos - missile.pos;

            let new_p = missile.pos + dir.normalize() * speed * dt;

            scene::update_dir(scene, m.id, dir);
            scene::update_pos(scene, m.id, new_p);

            // fake some collision, maybe have missile system call back to impl for hit
            let mut update = true;
            if dir.xy().magnitude() < 0.2 {
                update = game.on_missile_hit(i, scene);
            }
            if update {
                i += 1;
            }
        } else {
            // this is kinda wonky, maybe just have a remove on missile system.
            // We call on hit since that also removes missile. And when we are here, the target is gone,
            // so we want to remove it, well maybe we want to change target, but on_missile_hit should handle that
            game.on_missile_hit(i, scene);
            i += 1;
            // remove missile;
        }
    }
}

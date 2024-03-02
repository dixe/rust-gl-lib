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


// maybe overkill to have a function/system only for cooldown. But might be better since we can do it in 1 place,
// it does move the resposibility from other systems, and clearly defines where it should happen. To avoid doing it twice or more
pub fn cooldown_system(game: &mut impl UnitSystem, scene: &mut Scene) {

    let dt = scene.dt();
    let mut i = 0;
    while i < game.units() { // use while loop so we can modify during loop
        let unit = game.unit_mut(i);

        if unit.cooldown > 0.0 {
            unit.cooldown -= dt;
        }

        i += 1;
    }
}

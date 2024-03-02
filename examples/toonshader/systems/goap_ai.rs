use gl_lib::scene_3d as scene;
use gl_lib::scene_3d::EntityId;
use gl_lib::scene_3d::ParticleScene;
use crate::Scene;
use crate::GameData;
use crate::Unit;
use crate::systems::unit::UnitSystem;
use gl_lib::typedef::V3;
use gl_lib::scene_3d::actions;
use gl_lib::goap;
use std::rc::Rc;

#[derive(Debug)]
pub struct GoapData {
    id: EntityId,
    state: goap::State,
    // Do we need this? I think we do so ai know when to find a plan/goal, and when to just execute the current goal
    goal: Option::<goap::Goal>,
    goals: Rc::<goap::Goals>,
    actions: Rc::<goap::Actions>
}

pub trait GoapSystem {
    /// Return number of units we should run goap ai for
    fn goap_datas(&self) -> usize;

    /// Return mut goap_data for given index in loop
    fn goap_data(&mut self, idx: usize) -> &mut GoapData;
}

pub fn goap_data_system(game: &mut impl GoapSystem, scene: &mut Scene) {

    let mut i = 0;
    while i < game.goap_datas() { // use while loop so we can modify during loop
        let goap_data = game.goap_data(i);

        if goap_data.goal.is_some() {
            // check if goal is complete
            i += 1;
            continue;
        }


        if let Some((goal, plan)) = goap::plan(&goap_data.goals, &goap_data.actions, &goap_data.state) {
            println!("Found plan for {:?} {:?} -- {:?}", goap_data.id, goal.name, plan);
        }


        i += 1;
    }
}



impl GoapSystem for GameData {

    fn goap_datas(&self) -> usize {
        self.goap_datas.len()
    }

    fn goap_data(&mut self, idx: usize) -> &mut GoapData {
        self.goap_datas.get_mut(idx).expect("Goap system should not have called with idx outside scope")
    }
}

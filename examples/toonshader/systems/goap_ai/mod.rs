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


pub mod action_functions;
pub type EntityAiFn = fn(EntityId, &mut GameData, &mut Scene);


#[derive(Debug)]
pub Senses {
    pub target: Option<EntityId>,

}

#[derive(Debug)]
pub struct GoapData {
    pub id: EntityId,
    pub state: goap::State, // only the str to bool values used by the goap planner
    pub senses: Senses,
    // Do we need this? I think we do so ai know when to find a plan/goal, and when to just execute the current goal
    pub goal: Option::<goap::Goal>,
    pub next_action: Option::<Rc::<goap::Action>>, // next step in plan
    pub goals: Rc::<goap::Goals>,
    pub actions: Rc::<goap::Actions>,
}

pub trait GoapSystem {
    /// Return number of units we should run goap ai for
    fn goap_datas(&self) -> usize;

    /// Return mut goap_data for given index in loop
    fn goap_data(&mut self, idx: usize) -> &mut GoapData;

    fn goap_data_by_entity_id(&mut self, entity_id: EntityId) -> Option::<&mut GoapData>;

    fn get_action_fun(&self, actions: Rc::<goap::Action>) -> EntityAiFn;
}


pub fn execute_goal_system(game: &mut GameData, scene: &mut Scene) {

    let mut i = 0;

    while i < game.units() { // use while loop so we can modify during loop
        let this = game.unit(i);

        if this.dead {
            i += 1;
            continue;
        }

        let this_id = this.id.clone();
        if let Some(goap_data) = game.goap_data_by_entity_id(this.id) {
            if let Some(action) = &goap_data.next_action.clone() {
                let fun = game.get_action_fun(action.clone());
                (fun)(this_id, game, scene);
            }
        }
        i += 1;
    }
}

pub fn goap_data_system(game: &mut impl GoapSystem, scene: &mut Scene) {

    let mut i = 0;
    while i < game.goap_datas() { // use while loop so we can modify during loop
        let goap_data = game.goap_data(i);

        if goap_data.next_action.is_some() {

            // check if goal is complete
            i += 1;
            continue;
        }

        if let Some((goal, plan)) = goap::plan(&goap_data.goals, &goap_data.actions, &goap_data.state) {
            println!("Found plan for {:?} {:?} -- {:?}", goap_data.id, goal.name, plan);
            goap_data.next_action = Some(plan[0].clone());
            goap_data.goal = Some(goal);
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

    fn goap_data_by_entity_id(&mut self, entity_id: EntityId) -> Option::<&mut GoapData> {
        let mut i = 0;
        while i < self.goap_datas.len() {
            if self.goap_datas[i].id == entity_id {
                return Some(&mut self.goap_datas[i]);
            }
            i+= 1;
        }

        None
    }

    fn get_action_fun(&self, action: Rc::<goap::Action>) -> EntityAiFn {
        match self.goap_action_to_fn.get(&action.name) {
            Some(fun) => fun.clone(),
            None => empty
        }
    }
}

fn empty(_: EntityId, _: &mut GameData, _: &mut Scene) {
    panic!();
}

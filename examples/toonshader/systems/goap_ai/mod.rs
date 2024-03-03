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
pub type EntityAiFn = fn(EntityId, &mut GameData, &mut Scene) -> Option<()> ;


#[derive(Debug, Default)]
pub struct Target {
    id: EntityId,
    pos: V3
}

#[derive(Debug, Default)]
pub struct Senses {
    pub target: Option<Target>,
    pub pos_self: V3
}

#[derive(Debug)]
pub struct GoapData {
    pub id: EntityId,
    pub state: goap::State, // only the str to bool values used by the goap planner
    pub senses: Senses,
    // Do we need this? I think we do so ai know when to find a plan/goal, and when to just execute the current goal
    pub goal: Option::<goap::Goal>,
    pub plan: Vec::<Rc::<goap::Action>>,
    pub goals: Rc::<goap::Goals>,
    pub actions: Rc::<goap::Actions>,
}

impl GoapData {
    pub fn new(id: EntityId, goals: Rc::<goap::Goals>, actions: Rc::<goap::Actions>) -> Self {
        Self {
            id,
            state: goap::State::default(),
            senses: Senses::default(),
            goals,
            actions,
            goal: None,
            plan: vec![]
        }

    }
}

pub trait GoapSystem {
    /// Return number of units we should run goap ai for
    fn goap_datas(&self) -> usize;

    /// Return mut goap_data for given index in loop
    fn goap_data(&mut self, idx: usize) -> &mut GoapData;

    fn goap_data_by_entity_id(&mut self, entity_id: EntityId) -> Option::<&mut GoapData>;

    fn get_action_fun(&self, action_name: Rc::<str>) -> EntityAiFn;
}


pub fn update_senses(game: &mut GameData, scene: &mut Scene) {
    let mut i = 0;
    while i < game.goap_datas.len() { // use while loop so we can modify during loop


        let goap_data = &mut game.goap_datas[i];

        // TARGET SENSES
        if let Some(entity_self) = scene.entity(&goap_data.id) {
            panic!("It this correct");
            goap_data.senses.pos_self = entity_self.pos;
        } else {
            panic!();
            // maybe set is dead? or remove the goap_data
        }

        if let Some(target) = &mut goap_data.senses.target  {
            //check if we are still in range
            if let Some(target_entity) = scene.entity(&target.id) {
                target.pos = target_entity.pos;
                let dist = target.pos - goap_data.senses.pos_self;

                // TODO: calc from
                let in_range = dist < 10.0;

                goap_data.state.insert("InRangeOfTarget".into(), in_range);
            } else {
                goap_data.senses.target = None;
                goap_data.state.insert("HasTarget".into(), false);
                goap_data.state.insert("InRangeOfTarget".into(), false);
            }
        }

        i += 1;
    }
}


pub fn check_current_plan(game: &mut GameData, scene: &mut Scene) {
    let mut i = 0;
    while i < game.goap_datas.len() { // use while loop so we can modify during loop

        let goap_data = &mut game.goap_datas[i];

        // check that the current plan is stil valid

        if let Some(next_action) = goap_data.plan.last() {

            if !goap::is_valid(&next_action.pre, &goap_data.state)  {
                println!("invalid {:?}", next_action.name);
            }
        }

        i += 1;
    }
}

/// Take the current plan and execute it
pub fn execute_goal_system(game: &mut GameData, scene: &mut Scene) {

    let mut i = 0;

    while i < game.units() { // use while loop so we can modify during loop
        let u = game.unit(i);
        let unit = game.unit_data(u.id);

        let id = u.id;
        //TODO: Should be moved to be part of goal/plan as an is_alive state
        if unit.dead {
            i += 1;
            continue;
        }


        if let Some(goap_data) = game.goap_data_by_entity_id(unit.id) {
            if goap_data.plan.len() == 0 {
                // no plan found? can this be, either set goal to none, so we will find a new goal,
                // or panic since we don't allow goal without a plan!!

                println!("No plan found for {:?}, resetting goal to plan new one", goap_data.goal);
                goap_data.goal = None;
            } else {
                let next_action = goap_data.plan.last().unwrap().name.clone();
                let fun = game.get_action_fun(next_action);
                (fun)(id, game, scene);
            }
        }
        i += 1;
    }
}

pub fn goap_plan_system(game: &mut impl GoapSystem, scene: &mut Scene) {

    let mut i = 0;
    while i < game.goap_datas() { // use while loop so we can modify during loop
        let goap_data = game.goap_data(i);

        if goap_data.goal.is_some() {
            // check if goal is complete
            i += 1;
            continue;
        }

        if let Some((goal, plan)) = goap::plan(&goap_data.goals, &goap_data.actions, &goap_data.state) {
            // TODO: Maybe have a vec, and pass as mut to goap::plan, so we don't have to clone it all the time
            println!("Found plan for {:#?}, {:#?}", goal, plan);
            goap_data.plan = plan.clone();
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

    fn get_action_fun(&self, action_name: Rc::<str>) -> EntityAiFn {
        match self.goap_action_to_fn.get(&action_name) {
            Some(fun) => fun.clone(),
            None => action_functions::empty
        }
    }
}

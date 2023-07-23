use std::rc::Rc;
use std::collections::{BinaryHeap, HashMap};
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};

pub type Conditions = HashMap::<Rc::<str>, bool>;




#[derive(Clone, Debug, Deserialize)]
pub struct Goals {
    // Yes goal not goals, something to do with toml deserialize
    pub goal: Vec<Goal>
}


#[derive(Clone, Debug, Deserialize)]
pub struct Goal {
    pub name: Rc::<str>,
    pub desired_state: Conditions,
    pub is_valid: Conditions
}


fn is_goal_valid(conditions: &Conditions, state: &State) -> bool {


    // all values in conditions has to be the same in state.
    // false conditions are also satisfied when variable is not in state
    for (name, val) in conditions {
        let state_val = state.get(name).unwrap_or(&false);
        if state_val != val {
            return false;
        }
    }

    true

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actions {
    // Yes not actions, something to do with toml deserialize
    pub action: Vec<Action>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub name : Rc::<str>,
    pub pre: Conditions,
    pub cost: i32,
    pub post: Conditions,
}


impl Eq for Action {}


impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


// make this a type param on goal, action, and cond
// so each program can make their own state/blackboard type
// should be copy, so hashmap is not too good I think
pub type State = HashMap::<Rc::<str>, bool>;


impl Action {

    fn satisfy_name(&self, cond: &Rc::<str>, val: bool) -> bool {

        // might only work when conditions in post are always true
        for (name, post_val) in &self.post {
            if name == cond {
                return *post_val == val;
            }
        }
        false
    }

}



// assume goals are ordered by priority
pub fn plan(goals: &Goals, actions: &Actions, state: &State) -> Option<(Goal, Vec<Rc::<str>>)> {
    for goal in &goals.goal {
        if is_goal_valid(&goal.is_valid, state) {

            // try to make a plan

            // if we got a plan return it

            if let Some(plan) = plan_goal(&goal.desired_state, actions, state) {
                return Some((goal.clone(), plan.actions));
            }
        }
    }

    None
}

fn plan_goal(conditions: &Conditions, actions: &Actions, state: &State) -> Option<Node> {

    let mut plans = BinaryHeap::new();

    let mut first_node = Node::default();

    first_node.conditions = state.clone();

    push_node(&mut plans, &conditions, None, first_node);

    while let Some(node) = plans.pop() {
        // find all actions that satisfied one of the required conditions and push action if possible

        if node.finished() {
            return Some(node);
        }


        for (cond, val) in &node.conditions {

            // skip satisfied conditions
            if *val {
                continue;
            }

            for action in &actions.action {
                if action.satisfy_name(cond, !val) {
                    push_node(&mut plans, &conditions, Some(action), node.clone());

                }
            }
        }

    }

    None
}

fn push_node(heap: &mut BinaryHeap::<Node>, req_conds: &Conditions, act: Option<&Action>, mut node: Node) {

    if let Some(action) = act {
        node.cost += action.cost;
        node.actions.push(action.name.clone());

        for (name, _) in &action.pre {

            // if we don't have condition insert as false to indicate
            // that we need it
            // if we have it, it is either false, and we don't want to do anything
            // or it is true and we stil don't want to do anything
            if !node.conditions.contains_key(name) {
                node.conditions.insert(name.clone(), false);
            }
        }

        // set all action post to true, since we know that after this action
        // they will be true
        for (name, val) in &action.post {
            node.conditions.insert(name.clone(), *val);
        }
    }


    // insert require conditions last, since some might be satisfied by the action
    for (name, _) in req_conds {
        // if we don't have condition insert as false to indicate
        // that we need it
        // if we have it, it is either false, and we don't want to do anything
        // or it is true and we stil don't want to do anything
        if !node.conditions.contains_key(name) {
            node.conditions.insert(name.clone(), false);
        }
    }

    heap.push(node);

}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    actions : Vec::<Rc::<str>>,
    conditions: HashMap<Rc::<str>, bool>
}

impl Node {

    /// Check if all conditions are satisfied in node
    /// if they are then it is a valid "path" for the goal
    fn finished(&self) -> bool {
        for (_, val) in &self.conditions {
            if ! val {
                return false;
            }
        }
        true
    }
}


// impl to make min-heap instead of max heap on cost
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}


// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

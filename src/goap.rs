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


pub fn is_valid(conditions: &Conditions, state: &State) -> bool {

    // all values in conditions has to be the same in state.
    // false conditions are also satisfied when variable is not in state
    //println!("\n\n{:#?}", state);
    for (name, val) in conditions {
        //println!("req {:?}",(&name, val));
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
    pub action: Vec<Rc::<Action>>
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
        // might only work when conditions in post are always true, which they might not be
        for (name, post_val) in &self.post {
            if name == cond {
                //println!("{:?} Post condition = {:?} is required to be {:?}", cond, post_val, val);
                return *post_val == val;
            }
        }
        false
    }

}



// assume goals are ordered by priority
pub fn plan(goals: &Goals, actions: &Actions, state: &State) -> Option<(Goal, Vec<Rc::<Action>>)> {
    for goal in &goals.goal {
        if is_valid(&goal.is_valid, state) {
            //println!("Goal {:?} is valid", goal.name);
            // try to make a plan

            // if we got a plan return it

            if let Some(plan) = plan_goal(&goal.desired_state, actions, state) {

                //println!("Found plan  for Goal {:?}", goal.name);
                return Some((goal.clone(), plan.actions));
            }
        }
    }

    None
}

fn plan_goal(conditions: &Conditions, actions: &Actions, state: &State) -> Option<Node> {

    //println!("\n\nPLAN GOAL WITH {:#?}",conditions);

    let mut plans = BinaryHeap::new();

    let mut first_node = Node::default();


    //setup first node with all conditions and state
    first_node.state = state.clone();

    for (name, v) in conditions {
        first_node.required.insert(name.clone(), *v);
    }

    plans.push(first_node);


    while let Some(node) = plans.pop() {
        // find all actions that satisfied one of the required conditions and push action if possible

        if node.finished() {
            return Some(node);
        }


        for (cond, required_val) in &node.required {
            // skip satisfied conditions
            let state_val = node.state.get(cond).unwrap_or(&false);
            if required_val  == state_val {
                continue;
            }

            // TODO: find action that satisfy the most required condition, with the lowest cost

            // TODO: Also only choose actions which has all its pre satisfied, by the current state
            for action in &actions.action {
                if action.satisfy_name(cond, *required_val) {
                    //println!("NODE {:?}", node);

                    //println!("\n\n\nPUSHING ACTION {:#?}", action);
                    push_node(&mut plans, action.clone(), node.clone());

                    // we found and action, so break
                    break;

                }
            }
        }

    }

    None
}

fn push_node(heap: &mut BinaryHeap::<Node>, action: Rc::<Action>, mut node: Node) {

    node.cost += action.cost;
    node.actions.push(action.clone());

    // hmm, this seems like we are going in both directions.
    // adding pre to get preconditions
    // and adding post to simulate action done.

    //I don't think adding post is correct, but we still need to update the state some how
    // maybe what we should do is remove the actions.post, from the node required??


    // remove actions.post from node requried
    for (name, val) in &action.post {

        if node.required.get(name).is_some() {

            node.required.remove(name);
        }
    }


    // add actions pre conditon to update the required state
    for (name, val) in &action.pre {
        //println!("Adding {:?} = {} to required", name, val);
        node.required.insert(name.clone(), *val);

    }



    heap.push(node);
}


#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    actions : Vec::<Rc::<Action>>,

    // gets updated when nodes are pushed, by their post
    state: HashMap<Rc::<str>, bool>,

    // required conditions to be in a specific true or false state. For the goal/node to be presented as a plan
    required: HashMap<Rc::<str>, bool>
}

impl Node {

    /// Check if all conditions are satisfied in node
    /// if they are then it is a valid "path" for the goal
    fn finished(&self) -> bool {
        for (name, val) in &self.required {
            let state_val = self.state.get(name).unwrap_or(&false);
            //println!("req {:?}",(&name, val, state_val));
            if val != state_val {
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

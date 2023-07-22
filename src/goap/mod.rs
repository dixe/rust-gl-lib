use std::rc::Rc;
use std::collections::{BinaryHeap, HashMap};
use core::cmp::Ordering;
use serde::Deserialize;

#[derive(Clone)]
pub struct Goal {
    pub name: Rc::<str>,
    pub desired_state: Vec<Cond>,
    pub is_valid: fn(&State) -> bool
}


#[derive(Clone, Debug, Deserialize)]
pub struct Action {
    pub name : Rc::<str>,
    pub pre: Vec<Cond>,
    pub cost: i32,
    pub post: Vec<Cond>,
}


impl Eq for Action {}


impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}



#[derive(Debug, Clone, Deserialize)]
pub struct Cond {
    pub name: Rc::<str>,
    pub state: bool,
}

// make this a type param on goal, action, and cond
// so each program can make their own state/blackboard type
// should be copy, so hashmap is not too good I think
pub type State = HashMap::<Rc::<str>, bool>;


pub struct Plan {
    pub actions: Vec<Action>,
    pub cost: i32
}


impl Action {

    fn satisfy(&self, cond: &Cond) -> bool {
        self.satisfy_name(&cond.name)
    }

    fn satisfy_name(&self, cond: &Rc::<str>) -> bool {
        for post in &self.post {
            if &post.name == cond {
                return true;
            }
        }
        false
    }

}



// assume goals are ordered by priority
pub fn plan(goals: &[Goal], actions: &[Action], state: &mut State) -> Option<(Goal, Vec<Rc::<str>>)> {
    for goal in goals {
        if (goal.is_valid)(state) {

            // try to make a plan

            // if we got a plan return it

            if let Some(plan) = plan_goal(goal.desired_state.clone(), actions, state) {
                return Some((goal.clone(), plan.actions));
            }
        }
    }

    None
}

fn plan_goal(mut conditions: Vec::<Cond>, actions: &[Action], state: &State) -> Option<Node> {

    let mut plans = BinaryHeap::new();

    let mut first_node = Node::default();


    first_node.conditions = state.clone();

    push_node(&mut plans, &conditions, None, first_node);

    let mut i = 0;
    while let Some(node) = plans.pop() {
        i += 1;
        // find all actions that satisfied one of the required conditions and push action if possible

        if node.finished() {
            return Some(node);
        }


        for (cond, val) in &node.conditions {
            // skip satisfied conditions
            if *val {
                continue;
            }

            for action in actions {
                if action.satisfy_name(cond) {
                    push_node(&mut plans, &conditions, Some(action), node.clone());
                }
            }
        }

    }

    None
}

fn push_node(heap: &mut BinaryHeap::<Node>, req_conds: &Vec::<Cond>, act: Option<&Action>, mut node: Node) {

    if let Some(action) = act {
        node.cost += action.cost;
        node.actions.push(action.name.clone());

        for pre in &action.pre {

            // if we don't have condition insert as false to indicate
            // that we need it
            // if we have it, it is either false, and we don't want to do anything
            // or it is true and we stil don't want to do anything
            if !node.conditions.contains_key(&pre.name) {
                node.conditions.insert(pre.name.clone(), false);
            }
        }



        // set all action post to true, since we know that after this action
        // they will be true
        for post in &action.post {
            node.conditions.insert(post.name.clone(), true);
        }
    }


    // insert require conditions last, since some might be satisfied by the action
    for req in req_conds {
        // if we don't have condition insert as false to indicate
        // that we need it
        // if we have it, it is either false, and we don't want to do anything
        // or it is true and we stil don't want to do anything
        if !node.conditions.contains_key(&req.name) {
            node.conditions.insert(req.name.clone(), false);
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















/*
fn valid(goal.pre, actions: &[Action], state: {money: 20}) -> Option<Plan> {
    let sat = goal.sat;

    let is_valid = true;
    for cond in goal.sat {
        // see that "BuyAxe" satisfies goal
        // filters out has_money, since it is satifsied

        // maybe return a new state, with money = 10, so that if a later
        // action requires 15 money, we cannot also do that
        let (new_sat, new_state) = filter(buy.pre, state);

        is_valid &= valid(new_sat, [buy, to_shop], new_state);
    }

    if is_valid {
        return Some(actions);// so to_shop, buy)
    }
    return None
}


// take a list of conditions and filters out already satified
// conditions, with the alteration of the state if needed
fn filter(conditions: &[Cond], state) -> (&[Cond], state) {

}
*/

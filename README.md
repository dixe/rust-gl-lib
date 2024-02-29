# Open gl with some batteries

This crate uses [gl_generator](https://crates.io/crates/gl_generator) and adds some useful tools


# Text rendering

The TextRenderer can be used to render text using signed distance filed given a font. This textrendering can be scale and rotated while still looking crips. If text is not a main focus using this is an easy way to get some text on the screen.


# Gui
## Imode gui

Intermediate mode gui

## Tests

Run with `cargo t -- --test-threads=1` to only have 1 sdl instance at a time.

# Bugs

* [x] Text renderer ignores scale when calling render box.
* [x] Text rendering use regular bitmap font when text is small, only use sdf when text i large.
* [x] Intermediate mode gui should internally keep track of drawer2D.z and for most widget draw on top. This way rendering the background of windows after widgets is not a problem


# Features
* [ ] Make cargo fix and cargo text run on all test and examples
* [ ] Nav mesh
* [x] Toon shader
* [ ] Cascading shadow maps
* [ ] Mutiple different render pipelines, basically different shaders and enable/disable shadows, post process ect for each pipeline
* [ ] Death system handle death animation before removing. Simple solution is vec of death objects ids, where we can wait until the last animation has expired. We can already get that using the inputs, or the scene maybe


# Component System
Use specs crate
Often systems want to use some components for entities. Fx a damage system will want the health of entities. A component system in scene could be
with string index, so we can query "health" for a entity_id. The idea behind putting it into scene is to have a place to store them, and a built in system for querying
compontents. So adding a now one, fx health, attackInfo, armor, ect. is quite simple. The main problem is the types. One way is to extend scene using macro,
so adding a new health components will generate a container and logic for retriving the component. Can we use enum with dispatch? So components are user defined,
and we then define a

# Instancing
Drawing some thing instanced and some not, reuslts in fx slider knob circle will have the clearcolor as part of the square that is
alpha 0. Cannot get around it, unless we can draw the slider background before the knob. That means we need to to instancing in
layers, and circles should be instanced too.


https://www.reddit.com/r/godot/comments/xgrk0g/goap_goaloriented_action_planning_is_absolutely/
and https://www.youtube.com/watch?v=gm7K68663rA
and https://alumni.media.mit.edu/~jorkin/GOAP_draft_AIWisdom2_2003.pdf
and https://www.youtube.com/watch?v=nEnNtiumgII
struct WorldState {
  Enemy_visible: bool,
  is_alert: bool,
  weapon_equiped: bool,
  enemy_attacking: bool,
  ..
}


Action<WorldState> {
  name: str,
step: PlanStep, should be function that returns a planstep maybe
required_state: Vec::<ReqState>,
stisfies_state: Vec::<ReqState>,
}


// maybe just return next step
fn Plan()-> Vec::<PlanStep> {

}

Goal {

}

struct PlanStep {
Goto(),
Aimaiton(Animation)
}


```

Action {
    Name : Rc::<str>,
    Pre: Rc::<[Cond]>,
    cost: i32,
    post: Rc::<[Cond]>,
}

Cond {
    name: Rc::<str>,
    state: State,
}

State {
    True,
    False,
    Leq(i32),
    Qeq(i32),
    Eq(i32)
}



Goal {
    name: Rc::<str>,
    sat: Rc::<[Cond]>,
}

let goal = Goal { name: "GetAxe",
       sat: Cond { name: "HasAxe", state: True}
}

let buy = Action {
    name: "BuyAxe",
    pre: [
        Cond { name: "AtShop", state: True},
        Cond { name: "HasMoney", state: geq(10) }
    ],
    post: [
        Cond { name: "HasAxe", state: True},
    ]
}

let to_shop = Action {
    name: "GotToShop",
    post: [
        Cond { name: "AtShop", state: True},
    ]
}


fn valid(goal.pre, [buy, to_shop], state: {money: 20}) -> Option<Actions> {
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
```

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

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
* [ ] Intermediate mode gui should internally keep track of drawer2D.z and for most widget draw on top. This way rendering the background of windows after widgets is not a problem


# ActionQueue on scene

Maybe in a different module have actionQueue

That takes the queue and a mut scene, and then we can go over actions, and
update entity active animations, fx start attack, play sould, ect.

Should we also change to idle here when fx attack is done?

Alternative should the scene set as on input that the current animation has finished? That way we get it as an input and can react in the same function
that also sets state.


ActionQueue {
queue<Action>,
}

// Generic actions, so StartAnimation, Plays sound
// and not Attack, Roll ect.
Action {
StartAnimation(EntityId, "name"), // or Rc(Anim) is we don't want names
PlaySound("name"),
SpawnParticle("name", loc, other info if needed)
}

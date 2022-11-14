# Open gl with some batteries

This crate uses [gl_generator](https://crates.io/crates/gl_generator) and adds some useful tools


# Text rendering

The TextRenderer can be used to render text using signed distance filed given a font. This textrendering can be scale and rotated while still looking crips. If text is not a main focus using this is an easy way to get some text on the screen.


# Sdl gui
A elm inspired gui lib/framework. Builds on top of the regular gl lib.



## Tests

Run with `cargo t -- --test-threads=1` to only have 1 sdl instance at a time.


# Widgets

## Layoout protocol:
A widget will first ask all its children to calculate their size, width and height. Should also be able to specify they just want
all theey can get. This can be useful when flex, and we have multiple children, fx in a row.

A widget is responsible for setting its children geometry position. This position is relative to the incoming BoxContraint.

So a row widget might see that some children wants to float left, center and right. When the sizes are returned, we can calculate
their positions in the given box contraint.





# Bugs

* [x] Text renderer ignores scale when calling render box.
* [ ] Text rendering use regular bitmap font when text is small, only use sdf when text i large.

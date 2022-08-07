# Open gl with some batteries

This crate uses [gl_generator](https://crates.io/crates/gl_generator) and adds some useful tools


# Text rendering

The TextRenderer can be used to render text using signed distance filed given a font. This textrendering can be scale and rotated while still looking crips. If text is not a main focus using this is an easy way to get some text on the screen.


# Sdl gui
A elm inspired gui lib/framework. Builds on top of the regular gl lib.



## Tests

Run with `cargo t -- --test-threads=1` to only have 1 sdl instance at a time.

# Bugs

[ ] Text renderer ignores scale when calling render box.

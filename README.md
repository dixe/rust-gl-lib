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


## Fluent or markup language
In the current state it is cumbersome to both create each widget and manually build the tree, by passing parents. We want something more fluent. The main goal are 
fluent builder, with support for attributes. A challenge is that we also need to get the Ids of widgets, so we can listen and send messages to widgets. The output
of a builder or markup language parser could be a Dictionary/hashmap, that maps from string name/id to a usize Id.

### Markup Language

Xml in the form
```<?xml version="1.0" encoding="UTF-8"?>
<row>
  <checkbox id="cb"/>
  <col>
    <slider id="s" min="0" max="100" value="40"/>
    <checkbox id="cb2"/>
  </col>
</row>
```

We want to take that xml and parse it to a `ui_state`.

Required a hashmap of functions from XmlNode to a `Box::<dyn Widget>`, supply the default builtin functions. Custom functions for custom widgets.



### Builder
```
let b = Builder();
  
b.add(Row()
      .add(Checkbox("cb")
           .flex(3))
      .add(Slider("s", 0, 100, 50)
           .flex(2))
  );
```

# Bugs

* [x] Text renderer ignores scale when calling render box.
* [ ] Text rendering use regular bitmap font when text is small, only use sdf when text i large.

# Idea: Retained intermediate mode
Basic Imode works by clearing layout every frame and laying widget out as they
come. It is a simple way to do it. The major drawback of this approach is that
doing any complex layout is not possible, fx doing a column for labels and
column for data inputs. If we go frame by frame then when rendering the first
label, we cannot know how wide the label column should be, since later labels
might be wider than the first. But we need an answer since we are already
drawing the first data input.

The idea is to keep track of the layout from last frame along with a hash for
the widget. And only when the hash changes, which is when we add/remove widget,
or modify a widget size, then we redo the layout. Otherwise we just use the
stored layout.


The main motivation is that ui state is quite static, atleast on a frame to
frame time scale. So on the first frame we will do the layout from current
tree. If we encounter a widget id we don't know we will just do as we do now
When frame end/start see if current hash is different from tree hash. If it is
redo layout and update tree hash to current hash. As long as the hashes are
equal the layout should be the same, and we can just do some lookup to get
widget position and size.

The tree should for each widget keep info about it size and anchor position.

Only widget that are part of a structed layout contibute to the tree hash.
Currently that is all widgets that call `layout_rect`. Fx
adding/removing/changing drag points should not change the tree or its hash.

Hash should be widget id and its size, so if we update a widgets size, fx by
changin the text, it should result in a new hash, since it will require a full
relayout of the window it belongs to.

First step include:
* [x] Keep layout current layout tree, pr window. changing widget in a window does not change layout of other windows
* [ ] Update tree when layout changes

Test doc for text renderer

# Widgets

## Input to widgets slution 1
Maybe combine sdl input and general widget inputs. 
```
struct WidgetInputMessage {
  widget_id: usize,
  input: WidgetInput
 }
 
enum WidgetInput {
Sdl(sdl2::Event),
ApplicationInput(Box<dyn Any>>)
}
```

Now a widget just gets inputs. This will create a senario where if an event is sent to a widget, and it ignores it, we will not know. But that might be fine. 
The `dispatch_events` method in `event_handling` will work almost as it does not. But it will no call the `widget_handle_event` directly, but add all events for a widget
to the `WidgetInputMessage` queue. After we have dispatched event, a new method is added for reacting to the event. These events are both events from sdl, and 
events from the application. This way we can also disable sdl events by just not calling `dispatch_events`.

This will require 1 method in Widget for handling events.

## Input to widgets slution 2
Have a queue like WidgetOutputs, but for inputs.
```
struct WidgetInput {
  widget_id: usize,
  input: Box<dyn<Any>>
}
```

We then have a new method for dispatching widgetinputs. This will loop over inputs and dispatch them to the widgets. 



This will require 2 methods for handling events. One for Sdl and one for widgetInputs. A default passthrough method can be made for WidgetInput.

 
 ## Widget handle inputs
 Should a widget be able to update/send a message directly to another widget? Say you want a slider to connect to a text field widget. Either the application does the
 handling of reading outputs from slider, and feeding it into the text. Or the slider have and option to take outputs, that it can directly send message to using the 
 widget input queue.
 
 To start with, having the application do the work seems like the best option. Since we want to read the output most likely. If we read the slider output, then forwarding it to the correct textfield should be easy. And if we don't, then the slider value is always only used inside widgets, which for non trivial example programs. Are not readly useful. We assume that we always want to use the outputs of widgets. Since this output should influence application logic. And if we need to handle this output, then forwarding the output to another widget can happen there.

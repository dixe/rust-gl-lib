use crate::widget_gui::*;
use sdl2::event;

pub fn dispatch_events(state: &mut UiState, event: &event::Event) {

    use event::Event::*;

    // Text events. Dispatch to the current focus
    if event.is_text() {
        return;
    }


    //println!("{:?}", event);


    // Mouse events are dispatches to the matching widget, given position
    if event.is_mouse() {
        match event {
            MouseButtonUp { x, y, ..} => {
                let pos = Position {x: *x, y: *y};
                if let Some(id) = state.get_widget(pos) {
                    widget_handle_event(id, state, &event);
                }

            },
            MouseButtonDown { x, y, ..} => {
                let pos = Position {x: *x, y: *y};
                if let Some(id) = state.get_widget(pos) {
                    widget_handle_event(id, state, &event);
                }
            },
            MouseMotion { x, y, ..} => {
                let pos = Position {x: *x, y: *y};
                if let Some(id) = state.get_widget(pos) {
                    widget_handle_event(id, state, &event);
                }

             },
            _ => {}
        };
        return;
    }
}

fn widget_handle_event(id: Id, state: &mut UiState, event: &event::Event, ) {
    let widget = &mut state.widgets[id];
    widget.handle_sdl_event(event, &state.geom[id], id, &mut state.dispatcher_queue);
}


pub fn run_listeners(state: &mut UiState) {

    for id in 0..state.queues.len() {

        let mut listen_ctx = ListenerCtx {
            id,
            widgets: state.widgets.as_mut_slice(),

        };

        while let Some(e) = state.queues[id].pop_front() {
            let listener = &mut state.listeners[id];
            listener(e, &mut listen_ctx);
        }
    }
}

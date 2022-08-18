use crate::widget_gui::*;
use sdl2::event;

pub fn dispatch_events(state: &mut UiState, event: &event::Event) {

    use event::Event::*;

    // Text events. Dispatch to the current focus
    if event.is_text() {
        return;
    }

    // Mouse events are dispatches to the matching widget, given position
    if event.is_mouse() {
        match event {
            MouseButtonUp { mouse_btn, x, y, ..} => {
                let pos = Position {x: *x, y: *y};
                if let Some(id) = state.get_widget(pos) {
                    let dispatcher = &mut state.dispatchers[id];
                    dispatcher(&event, id, &mut state.dispatcher_queue);
                }

            },
            _ => {}
        };
        return;
    }
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

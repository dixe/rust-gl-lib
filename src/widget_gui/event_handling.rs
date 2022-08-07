use crate::widget_gui::*;
use sdl2::event;

pub fn handle_events(state: &mut UiState, event: &event::Event) {

    // TODO: figure out which widget gets the event, then from there propagate up usig parent
    // for now use the first widget

    let id = 2;
    let handler = &mut state.handlers[id];

    handler(&event, id, &mut state.handler_queue);

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

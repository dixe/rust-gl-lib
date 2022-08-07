use crate::widget_gui::*;
use sdl2::event;

pub fn handle_events(state: &mut UiState, event: &event::Event) {

    // TODO: figure out which widget gets the event, then from there propagate up usig parent
    // for now use the first widget


    let id = 0;
    let handler = &mut state.handlers[id];

    handler(&event, &mut state.queues[id]);



    let mut listen_ctx = ListenerCtx {
        id,
        widgets: state.widgets.as_mut_slice()
    };

    let listener = &mut state.listeners[id];



    while let Some(event) = state.queues[id].data.pop_front() {
        listener(event, &mut listen_ctx);
    }


}

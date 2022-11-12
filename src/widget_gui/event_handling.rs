use crate::widget_gui::*;
use sdl2::event;

pub fn dispatch_event(state: &mut UiState, event: &event::Event) {

    use event::Event::*;

    // Text events. Dispatch to the current focus
    if event.is_text() {
        return;
    }

    // Mouse events are dispatches to the matching widget, given position
    if event.is_mouse() {
        match event {
            MouseButtonUp { x, y, ..} => {
                let pos = Position {x: *x, y: *y};
                if let Some(id) = dispatched_widget_id(state, pos){
                    widget_handle_event(id, state, &event);
                    state.active_widget = None;
                }

            },
            MouseButtonDown { x, y, ..} => {
                let pos = Position {x: *x, y: *y};

                if let Some(id) = dispatched_widget_id(state, pos){
                    widget_handle_event(id, state, &event);
                    state.active_widget = Some(id);
                }
            },
            MouseMotion { x, y, ..} => {
                let pos = Position {x: *x, y: *y};

                if let Some(id) = dispatched_widget_id(state, pos){

                    widget_handle_event(id, state, &event);
                }

            }
            _ => {}
        };
        return;
    }
}

impl UiState {
    pub fn dispatch_widget_inputs(&mut self) {
        dispatch_widget_inputs(self);
    }
}


pub fn dispatch_widget_inputs(state: &mut UiState) {

    while let Some(wi) = state.widget_input_queue.0.pop_front() {
        state.widgets[wi.widget_id].handle_widget_input(wi.input);
    }

}

fn dispatched_widget_id(state: &UiState, pos: Position) -> Option<Id> {
    if let Some(id) = state.active_widget {
        return Some(id);
    }

    if let Some(id) = state.get_widget(pos) {
        return Some(id);
    }

    None
}


fn widget_handle_event(id: Id, state: &mut UiState, event: &event::Event, ) {
    let widget = &mut state.widgets[id];
    widget.handle_sdl_event(event, &state.geom[id], id, &mut state.widget_output_queue);
}

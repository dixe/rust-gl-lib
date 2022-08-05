use crate::na;
use crate::sdl_gui::components::base::{KeyInfo, ClickType, Component, ComponentEvent, OnTop};

pub enum HandleRes {
    Consumed,
    Unused,
}

type ComponentEvents = std::collections::VecDeque<InternalComponentEvent>;

pub type Components<Message> = std::collections::HashMap<usize, Component<Message>>;

pub struct ComponentContainer<Message> {
    next_id: usize,
    pub components: Components<Message>,
    component_events: ComponentEvents,
    pub messages: std::collections::VecDeque<Message>,
    focused_component: Option<usize>,
}

impl<Message> ComponentContainer<Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            next_id: 1,
            components: std::collections::HashMap::new(),
            component_events: std::collections::VecDeque::new(),
            messages: std::collections::VecDeque::new(),
            focused_component: None,
        }
    }

    pub fn reset(&mut self) {
        self.next_id = 1;
        self.components.clear();
        self.messages.clear();
        self.component_events.clear();
    }

    pub fn add_component(&mut self, component: Component<Message>) -> usize {
        let id = self.next_id;
        self.components.insert(id, component);
        self.next_id += 1;
        id
    }

    fn handle_events(&mut self) {
        let mut popped_event = self.component_events.pop_front();
        while let Some(event) = popped_event {
            let c = self.components.get_mut(&event.id);

            if let Some(comp) = c {
                // Internal handling like hover and clicks to focus
                let _ = match event.event {
                    ComponentEvent::Hover => {
                        comp.base_mut().hover = true;
                    }
                    ComponentEvent::HoverEnd => {
                        comp.base_mut().hover = false;
                    }
                    ComponentEvent::Clicked(click, _) => {
                        if click == ClickType::Left && comp.focus_on_click() {
                            self.focused_component = Some(event.id);
                        }
                        else {
                            self.focused_component = Some(event.id);
                        }
                    },
                    _ => {}
                };


                // Component specific handling of hover, clicks, text, ect.
                if let Some(msg) = comp.on_event(event.event) {
                    self.messages.push_back(msg.clone());
                }
            }

            popped_event = self.component_events.pop_front();
        }
    }

    pub fn handle_sdl_event(&mut self, event: sdl2::event::Event) -> HandleRes {
        use sdl2::event::Event;

        let mut res = HandleRes::Unused;

        match event {
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => match mouse_btn {
                sdl2::mouse::MouseButton::Left => {
                    res = push_component_event(
                        ComponentEvent::Clicked(ClickType::Left, na::Vector2::new(x, y)),
                        ComponentMatch::Position(x as f32, y as f32),
                        &self.components,
                        &mut self.component_events,
                        None,
                    );
                }
                sdl2::mouse::MouseButton::Right => {
                    res = push_component_event(
                        ComponentEvent::Clicked(ClickType::Right, na::Vector2::new(x, y)),
                        ComponentMatch::Position(x as f32, y as f32),
                        &self.components,
                        &mut self.component_events,
                        None,
                    );
                }
                _ => {}
            },
            Event::MouseMotion { x, y, .. } => {
                res = push_component_event(
                    ComponentEvent::Hover,
                    ComponentMatch::Position(x as f32, y as f32),
                    &self.components,
                    &mut self.component_events,
                    Some(hover_no_match),
                );
            }
            Event::KeyDown {keycode, keymod, ..} => {
                // only check if anything is in focus
                if let Some(focus_id) = self.focused_component {

                    if let Some(kc) = keycode {

                        let info = KeyInfo {
                            keycode: kc,
                            keymod
                        };

                        res = push_component_event(
                            ComponentEvent::KeyboardInput(info),
                            ComponentMatch::ById(focus_id),
                            &self.components,
                            &mut self.component_events,
                            None,
                        );
                    }
                }
            }
            _ => {}
        };

        self.handle_events();
        res
    }
}

fn hover_no_match<Message>(
    key: usize,
    component: &Component<Message>,
    component_events: &mut ComponentEvents,
) where
    Message: Clone,
{
    if component.base().hover {
        component_events.push_back(InternalComponentEvent {
            id: key,
            event: ComponentEvent::HoverEnd,
        });
    }
}

type NoMatchFn<Message> =
    fn(key: usize, component: &Component<Message>, component_events: &mut ComponentEvents);

#[derive(Debug, Clone, Copy)]
enum ComponentMatch {
    Position(f32, f32),
    ById(usize),
}

fn push_component_event<Message: Clone>(
    event: ComponentEvent,
    match_type: ComponentMatch,
    components: &Components<Message>,
    component_events: &mut ComponentEvents,
    no_match: Option<NoMatchFn<Message>>,
) -> HandleRes {
    let mut res = HandleRes::Unused;
    // TODO: Make this into a functions that takes the event to push
    // TODO: This is repeated and will get complicated

    match match_type {
        ComponentMatch::Position(x, y) => {
            for (key, comp) in components {
                match comp.on_top(x, y) {
                    OnTop::OnTop(_level) => {
                        res = HandleRes::Consumed;

                        if !comp.disabled() {
                            component_events.push_back(InternalComponentEvent { id: *key, event });
                        }
                    }
                    OnTop::No => {
                        if let Some(no_match_fn) = no_match {
                            no_match_fn(*key, comp, component_events);
                        }
                    }
                };
            }
        }
        ComponentMatch::ById(id) => {
            component_events.push_back(InternalComponentEvent { id, event });
        }
    }

    res
}

#[derive(Debug, Clone, Copy)]
struct InternalComponentEvent {
    id: usize,
    event: ComponentEvent,
}

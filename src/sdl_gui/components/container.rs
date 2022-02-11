use crate::sdl_gui::components::base::{OnTop, Component, ComponentEvent, ClickType};
use crate::na;

pub enum HandleRes {
    Consumed,
    Unused
}

type ComponentEvents = std::collections::VecDeque<InternalComponentEvent>;

pub type Components<Message> = std::collections::HashMap<usize, Component<Message>>;

pub struct ComponentContainer<Message> {
    next_id: usize,
    pub components: Components<Message>,
    component_events: ComponentEvents,
    pub messages: std::collections::VecDeque<Message>,
}


impl<Message> ComponentContainer<Message> where Message: Clone {

    pub fn new() -> Self {

        Self {
            next_id: 1,
            components: std::collections::HashMap::new(),
            component_events: std::collections::VecDeque::new(),
            messages: std::collections::VecDeque::new(),
        }
    }

    pub fn add_component(&mut self, component: Component<Message> )  -> usize {
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
                let _ = match event.event {
                    ComponentEvent::Hover => {
                        comp.base_mut().hover = true;
                    },
                    ComponentEvent::HoverEnd => {
                        comp.base_mut().hover = false;
                    },
                    _ => {},
                };

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
            Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        res = push_component_event(ComponentEvent::Clicked(ClickType::Left, na::Vector2::new(x,y)), x as f32, y as f32, &self.components, &mut self.component_events, None);
                    },
                    sdl2::mouse::MouseButton::Right => {
                        res = push_component_event(ComponentEvent::Clicked(ClickType::Right, na::Vector2::new(x,y)), x as f32, y as f32, &self.components, &mut self.component_events, None);

                    },
                    _ => {}

                }
            },
            Event::MouseMotion{x, y, .. }  => {
                res = push_component_event(ComponentEvent::Hover,
                                           x as f32,
                                           y as f32,
                                           &self.components,
                                           &mut self.component_events,
                                           Some(hover_no_match));
            }
            _ => {}

        };


        self.handle_events();
        res
    }
}

fn hover_no_match<Message>(key: usize, component: &Component<Message>, component_events: &mut ComponentEvents) where Message: Clone {

    if component.base().hover {
        component_events.push_back(InternalComponentEvent{
            id: key,
            event: ComponentEvent::HoverEnd
        });
    }
}

type NoMatchFn<Message> = fn (key: usize, component: &Component<Message>, component_events: &mut ComponentEvents);

fn push_component_event<Message: Clone>(event: ComponentEvent, event_x: f32, event_y: f32, components: &Components<Message>, component_events: &mut ComponentEvents, no_match: Option<NoMatchFn<Message>>) -> HandleRes {

    let mut res = HandleRes::Unused;
    // TODO: Make this into a functions that takes the event to push
    // TODO: This is repeated and will get complicated
    for (key, comp) in components {

        match comp.on_top(event_x, event_y) {
            OnTop::OnTop(_level) => {
                res = HandleRes::Consumed;

                component_events.push_back(InternalComponentEvent{
                    id: *key,
                    event,
                });

            },
            OnTop::No => {
                if let Some(no_match_fn) = no_match {
                    no_match_fn(*key, comp,  component_events);
                }
            }
        }
    }

    res
}


#[derive(Debug,Clone,Copy)]
struct InternalComponentEvent {
    id: usize,
    event: ComponentEvent
}

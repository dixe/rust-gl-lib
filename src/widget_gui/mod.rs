use std::collections::{VecDeque};
use std::any::Any;
use crate::text_rendering::font::Font;
use sdl2::event;


pub mod widgets;
pub mod render;
pub mod event_handling;

pub type Id = usize;

pub type Pixel = i32;

pub type Handler = Box<dyn FnMut(&event::Event, Id, &mut HandlerQueue)>;

pub type Listener = Box<dyn FnMut(Box::<dyn Any>, &mut ListenerCtx)>;

pub struct UiState {
    next_id: Id,
    widgets: Vec<Box<dyn Widget>>,
    children: Vec<Vec<Id>>,
    parents: Vec<Id>,
    pub geom: Vec<Geometry>,
    listeners: Vec<Listener>,
    size_constraints: Vec::<WidgetConstraint>,
    font: Font,
    handlers: Vec<Handler>,
    pub queues: Vec<EventQueue>,
    handler_queue: HandlerQueue
}


impl UiState {

    pub fn new() -> Self {
        UiState {
            next_id: 0,
            widgets: Vec::new(),
            children: Vec::new(),
            parents: Vec::new(),
            geom: Vec::new(),
            listeners: Vec::new(),
            size_constraints: Vec::new(),
            font: Default::default(),
            handlers: Vec::new(),
            queues: Vec::new(),
            handler_queue: VecDeque::new()
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, parent: Option<Id>, widget_constraint: Option::<WidgetConstraint>) -> Id {

        let id = self.next_id;
        self.next_id += 1;
        self.widgets.push(widget);
        self.children.push(Vec::new());
        self.queues.push(EventQueue::new());
        let eh = empty_handler;
        self.handlers.push(Box::new(eh));
        self.listeners.push(Box::new(empty_listener));

        let parent_id = match parent {
            Some(p_id) => p_id,
            None => id
        };

        self.parents.push(parent_id);

        if parent_id != id {
            self.children[parent_id].push(id);
        }

        self.size_constraints.push(match widget_constraint {
            Some(cons) => cons,
            None => WidgetConstraint::no_flex()
        });


        self.geom.push(Default::default());

        id
    }

    pub fn poll_widget_event(&mut self) -> Option<HandlerEvent> {
        self.handler_queue.pop_front()
    }

    pub fn set_widget_handler(&mut self, id: Id, handler: Handler) {
        self.handlers[id] = handler;
    }

    pub fn set_widget_listener(&mut self, id: Id, listener: Listener) {
        self.listeners[id] = listener;
    }

    pub fn widgets(&self) -> &[Box::<dyn Widget>] {
        &self.widgets
    }


}




pub type HandlerQueue = VecDeque::<HandlerEvent>;



#[derive(Debug)]
pub struct HandlerEvent {
    pub event: Box::<dyn Any>,
    pub target_id: Id
}


pub struct EventQueue {
    data: VecDeque<Box<dyn Any>>
}


impl EventQueue {

    pub fn new() -> Self {
        Self {
            data: VecDeque::new()
        }
    }

    pub fn push_value<T>(&mut self, event: T) where T: Sized + 'static {
        self.data.push_back(Box::new(event));
    }

    pub fn push_back(&mut self, event: Box<dyn Any>) {
        self.data.push_back(event);
    }

    pub fn pop_front(&mut self) -> Option<Box<dyn Any>> {
        self.data.pop_front()
    }


}


#[derive(Debug, Clone)]
pub struct WidgetConstraint {
    width: SizeConstraint,
    height: SizeConstraint
}


impl WidgetConstraint {

    pub fn no_flex() -> Self {
        Self {
            width: SizeConstraint::NoFlex,
            height: SizeConstraint::NoFlex
        }
    }


    pub fn flex_width(factor: u8) -> Self {
        Self {
            width: SizeConstraint::Flex(factor.into()),
            height: SizeConstraint::NoFlex
        }
    }

    pub fn flex_height(factor: u8) -> Self {
        Self {
            width: SizeConstraint::NoFlex,
            height: SizeConstraint::Flex(factor.into())
        }
    }

    pub fn constraint(&self, flex_dir: FlexDir) -> SizeConstraint {
        match flex_dir {
            FlexDir::X => self.width,
            FlexDir::Y => self.height,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeConstraint {
    NoFlex,
    Flex(u8),
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: Pixel,
    pub y: Pixel,
}

impl Position {
    pub fn add_by_flex(&mut self, val: Pixel, flex: FlexDir) {
        match flex {
            FlexDir::X => {
                self.x += val;
            },
            FlexDir::Y => {
                self.y += val;
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Geometry {
    pub pos: Position,
    pub size: Size
}


#[derive(Default)]
pub struct ListenerCtx<'a> {
    pub id: Id,
    pub widgets: &'a mut [Box<dyn Widget>],
}



#[derive(Debug, Clone, Copy)]
pub enum FlexDir {
    X,
    Y
}


#[derive(Default, Debug, Clone, Copy)]
pub struct BoxContraint {
    pub min_w: Pixel,
    pub min_h: Pixel,
    pub max_w: Pixel,
    pub max_h: Pixel
}

impl BoxContraint {

    pub fn new(w: Pixel, h:Pixel) -> Self {
        Self {
             min_w: 0,
             min_h: 0,
             max_w: w,
             max_h: h
        }
    }

    pub fn fixed_width(w: Pixel, h:Pixel) -> Self {
        Self {
             min_w: w,
             min_h: 0,
             max_w: w,
             max_h: h
        }
    }

        pub fn fixed_height(w: Pixel, h:Pixel) -> Self {
        Self {
             min_w: 0,
             min_h: h,
             max_w: w,
             max_h: h
        }
    }
}


#[derive(Debug, Clone)]
pub struct LayoutContext<'a> {
    widget_geometry : Vec::<Option<Geometry>>,
    size_constraints : &'a Vec::<WidgetConstraint>,
    font: &'a Font
}

impl<'a> LayoutContext<'a>{

    fn new(widgets: usize, size_constraints: &'a Vec::<WidgetConstraint>, font: &'a Font ) -> Self {
        let mut res = Self {
            widget_geometry: vec![],
            size_constraints,
            font
        };

        for _ in 0..widgets {
            res.widget_geometry.push(None);
        }

        res

    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub pixel_w: Pixel,
    pub pixel_h: Pixel,
}

impl Size {

    pub fn from_flex(&self, flex_dir: FlexDir) -> Pixel {
        match flex_dir {
            FlexDir::X => self.pixel_w,
            FlexDir::Y => self.pixel_h
        }
    }
}

pub trait Widget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult;


    fn render(&self, _: &Geometry, _: &mut render::RenderContext) {

    }

    fn handle_event(&mut self, _: Box::<dyn Any>) {

    }

}


pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub enum LayoutResult {
    Size(Size),
    RequestChild(Id, BoxContraint)
}

#[derive(Debug,Clone, Copy)]
struct ProcessData {
    id: Id,
    bc: BoxContraint
}

impl ProcessData {

    fn new(id: Id, bc: BoxContraint) -> Self {
        Self {id, bc}
    }
}

pub fn layout_widgets(root_bc: &BoxContraint, state: &mut UiState) {

    let mut process_queue = VecDeque::new();

    let mut ctx = LayoutContext::new(state.widgets.len(), &state.size_constraints, &state.font);

    // Start by processing the root
    process_queue.push_back(ProcessData::new(0, *root_bc));


    while let Some(process_data) = process_queue.pop_front() {

        let widget = &mut state.widgets[process_data.id];

        let children = &state.children[process_data.id];


        match widget.layout(&process_data.bc, children, &mut ctx) {
            LayoutResult::Size(size) => {
                let mut geom : Geometry = Default::default();
                geom.size = size;
                ctx.widget_geometry[process_data.id] = Some(geom);
                let geom = &mut state.geom[process_data.id];
                geom.size = size;
            },

            LayoutResult::RequestChild(child_id, child_constraint) => {
                process_queue.push_front(process_data);
                process_queue.push_front(ProcessData::new(child_id, child_constraint));
            }
        };


        for (id, geom) in ctx.widget_geometry.iter().enumerate() {

            if let Some(g) = geom {
                state.geom[id].pos = g.pos;
            }
        }
    }

}


fn empty_handler(_: &event::Event, _: Id, _: &mut HandlerQueue) {

}


fn empty_listener(_: Box::<dyn Any>, _: &mut ListenerCtx) {

}

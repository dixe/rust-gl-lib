use std::ops;
use std::collections::{VecDeque};
use std::any::Any;
use crate::text_rendering::font::Font;
use sdl2::event;


pub mod widgets;
pub mod render;
pub mod event_handling;
pub mod layout;
use layout::*;

pub type Id = usize;

pub type Pixel = i32;

pub type Dispatcher = Box<dyn FnMut(&event::Event, Id, &mut DispatcherQueue)>;

pub type Listener = Box<dyn FnMut(Box::<dyn Any>, &mut ListenerCtx)>;

pub struct UiState {
    next_id: Id,
    widgets: Vec<Box<dyn Widget>>,
    children: Vec<Vec<Id>>,
    parents: Vec<Id>,
    pub geom: Vec<Geometry>,
    listeners: Vec<Listener>,
    attributes: Vec::<LayoutAttributes>,
    font: Font,
    dispatchers: Vec<Dispatcher>,
    pub queues: Vec<EventQueue>,
    dispatcher_queue: DispatcherQueue,
    active_widget: Option<Id>
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
            attributes: Vec::new(),
            font: Default::default(),
            dispatchers: Vec::new(),
            queues: Vec::new(),
            dispatcher_queue: VecDeque::new(),
            active_widget: None,
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, parent: Option<Id>) -> Id {

        let id = self.next_id;
        self.next_id += 1;

        self.attributes.push(LayoutAttributes::for_widget(&widget));

        self.dispatchers.push(Box::new(widget.dispatcher()));
        self.widgets.push(widget);
        self.children.push(Vec::new());
        self.queues.push(EventQueue::new());
        self.listeners.push(Box::new(empty_listener));

        let parent_id = match parent {
            Some(p_id) => p_id,
            None => id
        };

        self.parents.push(parent_id);

        if parent_id != id {
            self.children[parent_id].push(id);
        }

        self.geom.push(Default::default());

        id
    }

    pub fn poll_widget_event(&mut self) -> Option<DispatcherEvent> {
        self.dispatcher_queue.pop_front()
    }

    pub fn set_widget_dispatcher(&mut self, id: Id, dispatcher: Dispatcher) {
        self.dispatchers[id] = dispatcher;
    }

    pub fn set_widget_listener(&mut self, id: Id, listener: Listener) {
        self.listeners[id] = listener;
    }

    pub fn widgets(&self) -> &[Box::<dyn Widget>] {
        &self.widgets
    }

    pub fn set_widget_attributes(&mut self, id: Id, attribs: LayoutAttributes) {
        self.attributes[id] = attribs;
    }

    fn get_widget(&self, pos: Position) -> Option<Id> {

        // For now use invariant that children has higher id than parents, so reverse the search order to highest id to lowest
        // First match is a leaf in the tree
        // return the deepest node in tree that macthes

        let mut match_id = None;

        for id in (0..self.geom.len()).rev() {
            if self.geom[id].contains_position(pos) {
                match_id = Some(id);
                //println!("{:?} MATCHED  id={:?} with geom =  {:?} ", pos, id, &self.geom[id]);
                break;
            }
            else {
                //println!("{:?} did not match id={:?} with geom =  {:?} ", pos, id, &self.geom[id]);
            }
        }

        match_id
    }
}

type DispatcherQueue = VecDeque::<DispatcherEvent>;



#[derive(Debug)]
pub struct DispatcherEvent {
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
pub struct LayoutAttributes {
    height: SizeConstraint,
    width: SizeConstraint,

}


impl LayoutAttributes {

    pub fn for_widget(widget: &Box<dyn Widget>) -> Self {
        Self {
            width: widget.default_width(),
            height: widget.default_height(),

        }
    }

    pub fn no_flex(mut self) -> Self {
        self.width = SizeConstraint::NoFlex;
        self.height = SizeConstraint::NoFlex;
        self
    }


    pub fn flex_width(mut self, factor: u8) -> Self {
        self.width =SizeConstraint::Flex(factor.into());
        self
    }

    pub fn flex_height(mut self, factor: u8) -> Self {
        self.height = SizeConstraint::Flex(factor.into());
        self
    }


    pub fn width(mut self, width: SizeConstraint) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, h: SizeConstraint) -> Self {
        self.height = h;
        self
    }

    pub fn constraint(&self, flex_dir: FlexDir) -> SizeConstraint {
        match flex_dir {
            FlexDir::X => self.width,
            FlexDir::Y => self.height,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SizeConstraint {
    #[default]
    NoFlex,
    Fixed(Pixel),
    Flex(u8),
}


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: Pixel,
    pub y: Pixel,
}

impl ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self {
        Position { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}


impl ops::AddAssign for Position {

    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
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

impl Geometry {

    fn contains_position(&self, pos: Position) -> bool {

        pos.x >= self.pos.x && pos.x <= (self.pos.x + self.size.pixel_w) &&
            pos.y >= self.pos.y && pos.y <= (self.pos.y + self.size.pixel_h)

    }

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
    //widget_geometry : Vec::<Option<Geometry>>,
    attributes : &'a Vec::<LayoutAttributes>,
    font: &'a Font,
    layout_geom: Vec::<Option<Geometry>>

}

impl<'a> LayoutContext<'a>{

    fn new(widgets: usize, attributes: &'a Vec::<LayoutAttributes>, font: &'a Font ) -> Self {
        let mut res = Self {
            //idget_geometry: vec![],
            attributes,
            font,
            layout_geom: vec![]
        };

        for _ in 0..widgets {
            //res.widget_geometry.push(None);
            res.layout_geom.push(None);
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

    fn dispatcher(&self) -> Dispatcher {
        Box::new(empty_dispatcher)
    }

    fn default_width(&self) -> SizeConstraint {
        SizeConstraint::NoFlex
    }

    fn default_height(&self) -> SizeConstraint {
        SizeConstraint::NoFlex
    }

    fn handle_sdl_event(&mut self, _event: &event::Event, _geom: &Geometry, _self_id: Id, _queue: &mut DispatcherQueue) {

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

    let mut ctx = LayoutContext::new(state.widgets.len(), &state.attributes, &state.font);

    // Start by processing the root
    process_queue.push_back(ProcessData::new(0, *root_bc));


    while let Some(process_data) = process_queue.pop_front() {

        let widget = &mut state.widgets[process_data.id];

        let children = &state.children[process_data.id];

        match widget.layout(&process_data.bc, children, &mut ctx) {
            LayoutResult::Size(mut size) => {

                match state.attributes[process_data.id].width {
                    SizeConstraint::NoFlex => {},
                    SizeConstraint::Fixed(px) => {
                        size.pixel_w = px;
                    }
                    SizeConstraint::Flex(_) => {
                        size.pixel_w = process_data.bc.max_w;
                    }
                };

                match state.attributes[process_data.id].height {
                    SizeConstraint::NoFlex => {},
                    SizeConstraint::Fixed(px) => {
                        size.pixel_h = px;
                    }
                    SizeConstraint::Flex(_) => {
                        size.pixel_h = process_data.bc.max_h;
                    }
                };

                let mut geom : Geometry = Default::default();
                geom.size = size;
                ctx.layout_geom[process_data.id]= Some(geom);
                let geom = &mut state.geom[process_data.id];
                geom.size = size;
            },




            LayoutResult::RequestChild(child_id, child_constraint) => {
                process_queue.push_front(process_data);
                process_queue.push_front(ProcessData::new(child_id, child_constraint));
            }


        };


        for (id, geom) in ctx.layout_geom.iter().enumerate() {
            if let Some(g) = geom {
                state.geom[id].pos = g.pos;

            }
        }
    }


    // propagate geom positions

    for id in 0..state.geom.len() {
        propagate_positions(id, &state.children[id], &mut state.geom);
    }
}


fn propagate_positions(id: Id, children: &[Id], geoms: &mut[Geometry]) {
    let pos = geoms[id].pos;

    for &child_id in children {
        geoms[child_id].pos += pos;
    }
}


fn empty_dispatcher(_: &event::Event, _: Id, _: &mut DispatcherQueue) {

}


fn empty_listener(_: Box::<dyn Any>, _: &mut ListenerCtx) {

}

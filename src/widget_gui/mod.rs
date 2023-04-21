use std::ops;
use std::collections::{VecDeque, HashMap};
use std::any::Any;
use crate::text_rendering::font::{Font, SdfFont};
use sdl2::event;
use std::fmt;
use crate::widget_gui::event_handling::dispatch_event;

pub mod widgets;
pub mod render;
pub mod event_handling;
pub mod layout;
use layout::*;

pub type Id = usize;

pub type Pixel = i32;


pub struct UiState {
    next_id: Id,

    widgets: Vec<Box<dyn Widget>>,
    children: Vec<Vec<Id>>,
    parents: Vec<Id>,
    pub geom: Vec<Geometry>,
    attributes: Vec::<LayoutAttributes>,
    font: Font,
    pub widget_input_queue: WidgetInputQueue,
    widget_output_queue: WidgetOutputQueue,
    active_widget: Option<Id>
}

impl fmt::Debug for UiState {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiState")
            .field("geom", &self.geom)
            .field("attributes", &self.attributes)
            .finish()
    }
}


impl UiState {

    pub fn new() -> Self {
        let sdf_font = Default::default();
        let font = Font::Sdf(sdf_font);
        UiState {
            next_id: 0,
            widgets: Vec::new(),
            children: Vec::new(),
            parents: Vec::new(),
            geom: Vec::new(),
            attributes: Vec::new(),
            font: font,
            widget_input_queue: WidgetInputQueue(VecDeque::new()),
            widget_output_queue: VecDeque::new(),
            active_widget: None,
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, parent: Option<Id>) -> Id {

        let id = self.next_id;
        self.next_id += 1;

        self.attributes.push(LayoutAttributes::for_widget(&widget));

        self.widgets.push(widget);
        self.children.push(Vec::new());

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

    pub fn poll_widget_outputs(&mut self) -> Option<WidgetOutput> {
        self.widget_output_queue.pop_front()
    }

    pub fn widgets(&self) -> &[Box::<dyn Widget>] {
        &self.widgets
    }

    pub fn set_widget_attributes(&mut self, id: Id, attribs: LayoutAttributes) {
        self.attributes[id] = attribs;
    }

    pub fn set_alignment(&mut self, id: Id, alignment: Alignment) {
        self.attributes[id].alignment = alignment;
    }

    pub fn set_alignment_x(&mut self, id: Id, a: AlignmentX) {
        self.attributes[id].alignment.x = a;
    }

    pub fn set_alignment_y(&mut self, id: Id, a: AlignmentY) {
        self.attributes[id].alignment.y = a;
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

    /// Skips the normal input queue and directly sends a message to a widget
    /// useful when the message might contain values that have lifetimes, and puting those into
    /// a queue will make it hard/imposible to do.
    pub fn push_input_to_widget(&mut self, wi: WidgetInput) {
        self.widgets[wi.widget_id].handle_widget_input(wi.input);

    }

}

pub type WidgetOutputQueue = VecDeque::<WidgetOutput>;

pub struct WidgetInputQueue(VecDeque::<WidgetInput>);

impl WidgetInputQueue {

    pub fn push_value<T: 'static>(&mut self, widget_id: usize, input: T) {
        self.0.push_back(WidgetInput { widget_id, input: Box::new(input)});
    }

    pub fn push_input(&mut self, widget_input: WidgetInput) {
        self.0.push_back(widget_input);
    }
}

#[derive(Debug)]
pub struct WidgetOutput {
    pub event: Box::<dyn Any>,
    pub widget_id: Id
}

#[derive(Debug)]
pub struct WidgetInput {
    pub input: Box::<dyn Any>,
    pub widget_id: Id
}


#[derive(Default, Debug, Clone)]
pub struct LayoutAttributes {
    height: SizeConstraint,
    width: SizeConstraint,
    alignment: Alignment
}


impl LayoutAttributes {

    pub fn for_widget(widget: &Box<dyn Widget>) -> Self {
        Self {
            width: widget.default_width(),
            height: widget.default_height(),
            alignment: Default::default()
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
            res.layout_geom.push(None);
        }

        res

    }
}


pub trait Widget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult;


    fn render(&self, _: &Geometry, _: &mut render::RenderContext) {

    }


    fn default_width(&self) -> SizeConstraint {
        SizeConstraint::NoFlex
    }

    fn default_height(&self) -> SizeConstraint {
        SizeConstraint::NoFlex
    }

    fn handle_widget_input(&mut self, _input: Box::<dyn Any>) {

    }

    fn handle_sdl_event(&mut self, _event: &event::Event, _geom: &Geometry, _self_id: Id, _queue: &mut WidgetOutputQueue) {

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

    layout_widgets_full(root_bc, state, Position { x:0, y: 0 });
}

pub fn layout_widgets_full(root_bc: &BoxContraint, state: &mut UiState, offset: Position ) {


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


    // set root position to offset
    state.geom[0].pos += offset;

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



// NEW WINDOWS STUFF


#[derive(Default)]
pub struct Ui<T> {
    pub windows: HashMap::<String, UiWindow<T>>,
    pub named_widget_input_queue: NamedWidgetInputQueue
}

impl<T>  Ui<T> {
    pub fn add_window(&mut self, name: String, window: UiWindow<T>) {
        self.windows.insert(name, window);
    }

    pub fn layout_all(&mut self) {
        for (_, window) in &mut self.windows {
            layout_widgets_full(&window.draw_box, &mut window.ui_state, window.root_pos);
        }
    }

    pub fn get_window(&self, name: &str) -> Option::<&UiWindow<T>> {
        self.windows.get(name)
    }

    pub fn render(&self, render_ctx: &mut render::RenderContext) {
        for (_, window) in &self.windows {
            render::render_ui(&window.ui_state, render_ctx);
        }
    }

    pub fn dispatch_event(&mut self, event: &event::Event) {
        // dispatch sdl events to widgets
        for (_, window) in &mut self.windows {
            dispatch_event(&mut window.ui_state, &event);
        }
    }

    pub fn handle_widget_events(&mut self, state: &mut T) {
        for (_, window) in &mut self.windows {
            let ui_state = &mut window.ui_state;
            while let Some(event) = ui_state.poll_widget_outputs() {
                match window.handler_functions.get(&event.widget_id) {
                    Some(handler) => {
                        handler(state, event, &mut self.named_widget_input_queue);
                    },
                    None => {}
                }
            }
        }

        while let Some(named_input) = self.named_widget_input_queue.pop_front() {
            // get window, and then widget name
            if let Some(window) = self.windows.get_mut(&named_input.to_window_name) {
                if let Some(widget_id) = window.named_widgets.get_mut(&named_input.to_widget_name) {
                    let mut widget = &mut window.ui_state.widgets[*widget_id];
                    println!("{:?}", named_input.input);
                    widget.handle_widget_input(named_input.input);
                }
            }
        }
    }
}

pub struct UiWindow<T> {
    pub ui_state: UiState,
    pub draw_box: BoxContraint,
    pub root_pos: Position,
    pub named_widgets: HashMap::<String,Id>,
    pub handler_functions: HashMap::<Id,  fn(&mut T, WidgetOutput, &mut NamedWidgetInputQueue)>
}

#[derive(Default)]
pub struct NamedWidgetInputQueue(VecDeque::<NamedWidgetInput>);

impl NamedWidgetInputQueue {

    pub fn pop_front(&mut self) -> Option<NamedWidgetInput> {
        self.0.pop_front()
    }

    pub fn send_value_to<T: 'static>(&mut self, window_name: &str,widget_name: &str, input: T) {
        self.0.push_back(NamedWidgetInput { to_window_name: window_name.to_string(), to_widget_name: widget_name.to_string(), input: Box::new(input)});
    }

    pub fn send_to(&mut self, window_name: &str, widget_name: &str, input: Box::<dyn Any>) {
        self.0.push_back(NamedWidgetInput { to_window_name: window_name.to_string(), to_widget_name: widget_name.to_string(), input});
    }
}

#[derive(Debug)]
pub struct NamedWidgetInput {
    pub to_window_name: String,
    pub to_widget_name: String,
    pub input: Box::<dyn Any>
}

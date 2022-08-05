use std::collections::{VecDeque, HashSet};
use std::any::Any;


pub mod widgets;

pub type Id = usize;

pub struct UiState {
    next_id: Id,
    widgets: Vec<Box<dyn Widget>>,
    children: Vec<Vec<Id>>,
    parents: Vec<Id>,
    pub geom: Vec<Geometry>,
    listeners: Vec<Box<FnMut(&mut Any, ListenerCtx)>>
}


impl UiState {

    pub fn new() -> Self {
        UiState {
            next_id: 0,
            widgets: Vec::new(),
            children: Vec::new(),
            parents: Vec::new(),
            geom: Vec::new(),
            listeners: Vec::new()
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, parent: Option<Id>) -> Id {

        let id = self.next_id;
        self.next_id += 1;
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


}



#[derive(Default, Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Clone)]
pub struct Geometry {
    pub pos: Position,
    pub size: Size
}


#[derive(Default, Debug, Clone)]
pub struct ListenerCtx {
}

#[derive(Default, Debug, Clone)]
pub struct BoxContraint {
    pub min_w: usize,
    pub min_h: usize,
    pub max_w: usize,
    pub max_h: usize
}



impl BoxContraint {

    pub fn screen(w: usize, h:usize) -> Self {
        Self {
             min_w: w,
             min_h: h,
             max_w: w,
             max_h: h
        }
    }
}


#[derive(Debug, Clone)]
pub struct LayoutContext {

    widget_sizes: Vec::<Option<Size>>,
}

impl LayoutContext {

    fn new(widgets: usize) -> Self {
        let mut res = Self {
            widget_sizes: vec![]
        };

        for _ in 0..widgets {
            res.widget_sizes.push(None);
        }

        res

    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub pixel_w: usize,
    pub pixel_h: usize,
}


pub trait Widget {
    fn layout(&mut self, bc: &BoxContraint, children: &[Id], ctx: &mut LayoutContext) -> LayoutResult;
}


pub enum LayoutResult {
    Size(Size),
    RequestChild(Id, BoxContraint)
}




pub fn layout_widgets(bc: &BoxContraint, state: &mut UiState) {


    let mut next_to_layout = 0;

    let mut process_queue = VecDeque::new();

    let mut ctx = LayoutContext::new(state.widgets.len());

    for id in 0..state.widgets.len() {
        process_queue.push_back(id);
    }


    while let Some(id) = process_queue.pop_front() {


        let widget = &mut state.widgets[id];

        let children = &state.children[id];


        match widget.layout(bc, children, &mut ctx) {
            LayoutResult::Size(size) => {
                ctx.widget_sizes[id] = Some(size);
                let geom = &mut state.geom[id];
                geom.size = size;

            },
            LayoutResult::RequestChild(child_id, child_constraints) => {
                process_queue.push_front(id);
                process_queue.push_front(child_id);
            }
        };
    }
}

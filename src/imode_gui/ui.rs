use crate::na::{self, Translation3};
use crate::text_rendering::{text_renderer::TextRenderer};
use crate::{gl::{self, viewport}, ScreenBox, ScreenCoords};
use crate::shader::{ TransformationShader, rounded_rect_shader::{self as rrs, RoundedRectShader}, circle_shader::{self as cs, CircleShader}};
use crate::objects::square;
use sdl2::event;
use crate::color::Color;
use crate::imode_gui::numeric::Numeric;
use crate::imode_gui::style::*;
use crate::text_rendering::font::{Font, FntFont};
use super::*;
use std::collections::HashMap;


#[derive(Default)]
pub struct Window {
    pub name: String,
    pub base_container_context: ContainerContext,
    pub container_contexts: std::collections::HashMap<Id, ContainerContext>,
    pub active_context: Option<Id>,
}

pub struct Ui {
    pub drawer2D: Drawer2D,
    pub mouse_pos: Pos,
    pub mouse_diff: Pos,

    pub mouse_down: bool,
    pub mouse_up: bool,
    pub mouse_down_pos: Pos,
    pub style: Style,

    pub windows: HashMap::<usize, Window>,
    pub frame_events: Vec::<event::Event>,
    pub current_window: Vec::<usize>, // index into windows, a stack
    pub window_to_id: HashMap::<String, usize>,
    pub next_window_id: usize
}

impl Ui {

    pub fn new(mut drawer2D: Drawer2D) -> Self {

        let mut base_window : Window = Default::default();
        base_window.name = "BASE".to_owned();

        base_window.base_container_context.width = drawer2D.viewport.w;

        let style = Default::default();

        // load fonts for small text size, and add to drawer2D font cache

        let small_font_inner = FntFont::default();

        let small_font = Font::fnt(&drawer2D.gl, small_font_inner);

        drawer2D.font_cache.add_font(small_font);

        let mut windows: HashMap::<usize, Window>  = Default::default();
        windows.insert(0, base_window);
        Self {
            drawer2D,
            mouse_pos: Pos::new(0,0),
            mouse_diff: Pos::new(0,0),
            mouse_down: false,
            mouse_down_pos: Pos::new(0,0),
            mouse_up: false,
            style,
            windows,
            frame_events: vec![],
            current_window: vec![0],
            window_to_id: Default::default(),
            next_window_id: 1
        }
    }

    pub fn set_hot(&mut self, id: Id) {
        self.ctx_fn(|ctx| ctx.set_hot(id));
    }

    pub fn is_hot(&mut self, id: Id) -> bool {
        self.ctx_fn(|ctx| ctx.is_hot(id))
    }

    pub fn is_active(&mut self, id: Id) -> bool {
        self.ctx_fn(|ctx| ctx.is_active(id))
    }

    pub fn set_active(&mut self, id: Id) {
        self.ctx_fn(|ctx| ctx.set_active(id))
    }

    pub fn set_not_active(&mut self, ) {

        // set not active
        if let Some(id) = self.ctx_fn(|ctx| ctx.set_not_active()) {
            // remove any containers that the widget had open
            self.remove_container_context(id);
        }
    }

    pub fn next_id(&mut self) -> u64 {
        self.ctx_fn(|ctx| ctx.next_id())
    }

    fn ctx_fn<T, F>(&mut self, ctx_fn: F) -> T where F: Fn(&mut ContainerContext) -> T {

        let last = self.current_window.last().unwrap();
        let window : &mut Window = self.windows.get_mut(last).unwrap();

        if let Some(active_ctx_id) = window.active_context {
            if let Some(ctx) = window.container_contexts.get_mut(&active_ctx_id) {
                return ctx_fn(ctx);
            } else {
                return ctx_fn(&mut window.base_container_context);
            }
        } else {
            return ctx_fn(&mut window.base_container_context);
        }
    }

    pub fn mouse_in_rect(&self, rect: &Rect) -> bool {
        self.mouse_pos.x >= rect.x
            && self.mouse_pos.x <= rect.x + rect.w
            && self.mouse_pos.y >= rect.y
            && self.mouse_pos.y <= rect.y + rect.h
    }


    pub fn mouse_down_in_rect(&self, rect: &Rect) -> bool {
        self.mouse_down_pos.x >= rect.x
            && self.mouse_down_pos.x <= rect.x + rect.w
            && self.mouse_down_pos.y >= rect.y
            && self.mouse_down_pos.y <= rect.y + rect.h
    }


    pub fn exit_active_context(&mut self, id: Id) {
        let window : &mut Window = self.windows.get_mut(self.current_window.last().unwrap()).unwrap();
        if let Some(ctx) = window.container_contexts.get(&id) {
            window.active_context = ctx.prev_active_context;
        }
    }

    pub fn remove_container_context(&mut self, id: Id) {
        let window : &mut Window = self.windows.get_mut(self.current_window.last().unwrap()).unwrap();
        window.container_contexts.remove(&id);


        let window2 = self.windows.get_mut(self.current_window.last().unwrap()).unwrap();
        let a = 2;
    }

    pub fn set_active_context(&mut self, id: Id, rect: Rect) {

        let window : &mut Window = self.windows.get_mut(self.current_window.last().unwrap()).unwrap();

        let cur = window.active_context;
        window.active_context = Some(id);

        if !window.container_contexts.contains_key(&id) {
            let mut ctx : ContainerContext = Default::default();

            ctx.anchor_pos.x = rect.x;
            ctx.anchor_pos.y = rect.y;
            ctx.width = rect.w;

            ctx.prev_active_context = cur;
            window.container_contexts.insert(id, ctx);
        }
    }

    pub fn layout_rect(&mut self, mut rect: Rect) -> Rect {
        let spacing = self.style.spacing;
        let auto_wrap = self.style.auto_wrap;
        self.ctx_fn(|ctx| ctx.layout_rect(spacing, auto_wrap, rect))
    }

    pub fn newline(&mut self) {
        self.ctx_fn(|ctx| ctx.newline());
    }


    pub fn get_frame_inputs(&self) -> &[event::Event] {
        // only return if no window is active
        for window in self.windows.values() {
            if window.base_container_context.active != None {
                return &[]
            }
        }

        return &self.frame_events
    }


    // TODO: Either return unused events only. Or return all events along with bool to indicate if the event is used/consumed by gui
    pub fn consume_events(&mut self, event_pump: &mut sdl2::EventPump) -> &[event::Event] {

        self.mouse_down = false;
        self.mouse_up = false;

        let mut any_hot = false;
        let mut any_active = false;
        for window in self.windows.values() {
            any_hot |= window.base_container_context.hot != None;
            any_active |= window.base_container_context.active != None;

            if any_hot && any_active {
                break;
            }
        }

        for window in &mut self.windows.values_mut() {
            clear_context(&mut window.base_container_context);
            for (_, ctx) in &mut window.container_contexts {
                clear_context(ctx);
            }
        }


        self.frame_events.clear();
        use event::Event::*;
        use sdl2::keyboard::Keycode::*;

        for event in event_pump.poll_iter() {
            match event {
                MouseButtonDown {x, y, ..} => {
                    self.mouse_down = true;
                    self.mouse_down_pos = Pos::new(x, y);

                    // Stop click when there are a hot or an active
                    if !any_hot && !any_active {
                        self.frame_events.push(event.clone());
                    }
                },
                MouseButtonUp {x, y, ..} => {
                    self.mouse_up = true;

                    if !any_active {
                        self.frame_events.push(event.clone());
                    }
                },
                MouseMotion {x,y, .. } => {
                    self.mouse_diff = Pos::new(x, y) - self.mouse_pos;
                    self.mouse_pos = Pos::new(x, y);

                    if !any_active {
                        self.frame_events.push(event.clone());
                    }
                },
                Window {win_event: event::WindowEvent::Resized(x,y), ..} => {
                    self.drawer2D.update_viewport(x, y);
                    self.windows.get_mut(&0).unwrap().base_container_context.width = x;
                },
                Quit { .. } => {
                    std::process::exit(0);
                },
                KeyDown { keycode: Some(Escape), ..} => {

                    if any_active {
                        self.set_not_active();
                    }
                    else {
                        self.frame_events.push(event.clone());
                    }
                },
                other => {
                    // maybe passing non mouse events it fine, when something is hot, but not active
                    if !any_hot && !any_active {
                        self.frame_events.push(other.clone());
                    }
                }
            }
        }

        return &self.frame_events;
    }
}

fn clear_context(ctx: &mut ContainerContext) {
    ctx.next_id = 0;
    ctx.hot = None;

    ctx.draw_offset = Pos::new(0, 0);
    ctx.max_y_offset = 0;

}

#[derive(Debug, Clone, Default, Copy)]
pub struct ContainerContext {
    /// Reset after each frame. Widget should use hot instead of just checking in the mouse is inside them
    /// since hot only gets set when there is not active widget or we out self are the active elemnt
    /// use to check fx release of mouse happens inside button.
    pub hot: Option<Id>,

    /// Persisted between frames
    pub active: Option<Id>,

    pub next_id: u64,

    pub prev_active_context: Option<Id>,

    pub anchor_pos: Pos,

    pub draw_offset: Pos,
    pub max_y_offset: i32,

    pub width: i32,

}


impl ContainerContext {
    pub fn set_hot(&mut self, id: Id) {
        if self.active == None || self.active == Some(id) {
            self.hot = Some(id);
        }
    }

    pub fn is_hot(&self, id: Id) -> bool {
        self.hot == Some(id)
    }

    pub fn is_active(&self, id: Id) -> bool {
        self.active == Some(id)
    }
    pub fn set_active(&mut self, id: Id) {
        // TODO: Should this also check if we are already hot?
        self.active = Some(id)
    }

    pub fn set_not_active(&mut self) -> Option<Id> {
        let cur = self.active;
        self.active = None;
        cur
    }

    pub fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn layout_rect(&mut self, spacing: Spacing, auto_wrap: bool, mut rect: Rect) -> Rect {

        // add spacing
        rect.x += spacing.x;
        rect.y += spacing.y;

        // check to see if wrap, if enabled
        if auto_wrap && self.draw_offset.x + rect.w > self.width {
            self.newline();
        }

        rect.x += self.draw_offset.x;
        rect.y += self.draw_offset.y;

        self.draw_offset.x = rect.x + rect.w;
        self.max_y_offset = i32::max(self.max_y_offset, rect.y + rect.h);

        rect.x += self.anchor_pos.x;
        rect.y += self.anchor_pos.y;

        rect

    }

    pub fn newline(&mut self) {
        self.draw_offset.x = 0;
        self.draw_offset.y = self.max_y_offset;
    }
}

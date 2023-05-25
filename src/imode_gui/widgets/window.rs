use super::*;


pub struct WindowRes {
    pub closed: bool,
    //expanded: bool,
}

impl Ui{

    /// Return struct with info about click on close button and collaped/expanded
    /// The widget it self does NOT contain logic to discard elements draw to it, when closed/collapsed
    /// Should be handled by applications
    pub fn window_begin(&mut self, text: &str) -> WindowRes {

        let mut res = WindowRes {
            closed: false,
            //expanded: false
        };

        let win_id = match self.window_to_id.get(text) {
            Some(id) => *id,
            None => {
                let window_id = self.next_window_id ;
                self.next_window_id += 1;

                self.window_to_id.insert(text.to_string(), window_id);
                let mut window : Window =  Default::default();

                window.id = window_id;
                window.top_bar_size = Pos::new(0, 20);
                window.base_container_context.anchor_pos = Pos::new(100, 100 + 20);
                window.base_container_context.next_id.window_id = window_id;
                self.windows.insert(window_id, window);
                window_id
            }
        };

        // windows can use 0, since they are the "base", this makes it so we always get the same id
        let id = Id {widget_id: 0, window_id: win_id };


        self.current_window.push(win_id);


        // update window pos when active, i.e. we are dragging
        if self.is_active(id) {
            self.set_window_pos(self.mouse_pos);
            if self.mouse_up {
                self.set_not_active();
            }
        }


        let window = self.windows.get_mut(&win_id).unwrap();
        let c_rect = close_rect(&window);
        let anchor = window.base_container_context.anchor_pos;
        let rect = Rect {
            x: anchor.x,
            y: anchor.y - window.top_bar_size.y,
            w: window.top_bar_size.x,
            h: window.top_bar_size.y,
        };



        // draw and then place close buttons
        self.draw(text, win_id);

        res.closed = self.button_at_empty(c_rect);


        if self.mouse_in_rect(&rect) {
            self.set_hot(id)
        }


        if self.is_active(id) {
            self.set_window_pos(self.mouse_pos);
            if self.mouse_up {
                self.set_not_active();
            }
        }
        else if self.is_hot(id) {
            if self.mouse_down {
                // check if we hit the close button


                if self.mouse_in_rect(&c_rect) {
                    self.set_active(id);
                }

                // otherwise we drag the window
                self.set_window_drag_point(self.mouse_pos - anchor);
                self.set_active(id);
            }
        }

        res

    }


    pub fn draw(&mut self, title: &str, win_id: usize) {

        let window = self.windows.get_mut(&win_id).unwrap();
        if !window.is_drawn {
            window.is_drawn = true;


            let c_rect = close_rect(&window);

            let anchor = window.base_container_context.anchor_pos;

            let bg_color = Color::Rgb(200, 200, 255);
            let color = Color::Rgb(255,255,255);

            // Background
            self.drawer2D.rounded_rect_color(anchor.x,
                                             anchor.y,
                                             window.base_container_context.width.size() + self.style.spacing.x * 2,
                                             window.base_container_context.height.size() + self.style.spacing.y,
                                             bg_color);

            // window Top Bar
            self.drawer2D.rounded_rect_color(anchor.x,
                                             anchor.y - window.top_bar_size.y,
                                             window.top_bar_size.x,
                                             window.top_bar_size.y,
                                             color);

            self.drawer2D.render_text(title, anchor.x + self.style.spacing.x, anchor.y - window.top_bar_size.y + self.style.spacing.y, 13);

            // window border
            let thickness = 3;
            let tl = anchor + Pos::new(0, -window.top_bar_size.y);
            let tr = anchor
                + Pos::new(window.base_container_context.width.size(), -window.top_bar_size.y)
                + Pos::new(self.style.spacing.x * 2, 0);

            let bl = anchor + Pos::new(0, window.base_container_context.height.size())
                + Pos::new(0, self.style.spacing.y);

            let br = anchor
                + Pos::new(window.base_container_context.width.size(), window.base_container_context.height.size())
                + Pos::new(self.style.spacing.x * 2, self.style.spacing.y);;


            // left vertical
            self.drawer2D.rounded_rect_color(tl.x, tl.y, thickness, bl.y - tl.y, color);

            // right vertical
            self.drawer2D.rounded_rect_color(tr.x, tr.y, thickness, br.y - tr.y, color);

            // bottom horizontal
            self.drawer2D.rounded_rect_color(bl.x, bl.y, br.x - bl.x + thickness, thickness, color);


            // Reset matchContent width and height
            window.base_container_context.width = match window.base_container_context.width {
                ContainerSize::Fixed(w) => ContainerSize::Fixed(w),
                ContainerSize::MatchContent(_) => ContainerSize::MatchContent(0)
            };

            window.base_container_context.height = match window.base_container_context.height {
                ContainerSize::Fixed(h) => ContainerSize::Fixed(h),
                ContainerSize::MatchContent(_) => ContainerSize::MatchContent(0)
            };
        }

    }


    pub fn window_end(&mut self, text: &str) {

        // draw window, but still we cannot, since layers?? but here in end we do have our size known, if it change dynamicly

        let window : &mut Window = self.windows.get_mut(self.current_window.last().unwrap()).unwrap();

        let mut ctx = &mut window.base_container_context;
        if let Some(active_ctx_id) = window.active_context {
            if let Some(active_ctx) = window.container_contexts.get_mut(&active_ctx_id) {
                ctx = active_ctx;
            }
        }

        // get window width, from max of container??
        window.top_bar_size.x = window.base_container_context.width.size() + self.style.spacing.x * 2;

        self.current_window.pop();



    }
}

pub fn close_rect(window: &Window) -> Rect {
    let anchor = window.base_container_context.anchor_pos;
    let marg = 3;
    let side = 14;
    Rect {
        x: anchor.x + window.top_bar_size.x - (side + marg),
        y: anchor.y - window.top_bar_size.y + marg,
        w: side,
        h: side,
    }

}

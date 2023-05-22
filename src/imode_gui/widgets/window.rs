use super::*;

impl Ui{
    pub fn window_begin(&mut self, text: &str) {
        let win_idx = match self.window_to_id.get(text) {
            Some(id) => *id,
            None => {
                let id = self.next_window_id ;
                self.next_window_id += 1;

                self.window_to_id.insert(text.to_string(), id);
                let mut window : Window =  Default::default();
                window.base_container_context.anchor_pos = Pos::new(100, 100);
                self.windows.insert(id, window);
                id
            }
        };

        self.current_window.push(win_idx);
    }

    pub fn window_end(&mut self, text: &str) {

        // draw window, but still we cannot, since layers?? but here in end we do have our size known, if it change dynamicly

        let window : &mut Window = self.windows.get_mut(self.current_window.last().unwrap()).unwrap();

        window.name = text.to_owned();
        let mut ctx = &mut window.base_container_context;
        if let Some(active_ctx_id) = window.active_context {
            if let Some(active_ctx) = window.container_contexts.get_mut(&active_ctx_id) {
                ctx = active_ctx;
            }
        }


        let color = self.style.button.color;

        self.drawer2D.rounded_rect_color(0, 0, ctx.width, ctx.max_y_offset, color);

        self.current_window.pop();
    }
}

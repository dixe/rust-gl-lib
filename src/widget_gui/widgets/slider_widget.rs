use crate::widget_gui::*;
use crate::widget_gui::render;
use crate::text_rendering::text_renderer::TextRenderer;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct SliderWidget {
    pub text_left: Option<String>,
    pub text_right: Option<String>,
    in_motion: bool,
    value: Rc<RefCell::<f64>>,
    max: f64,
    min: f64,
    circle_r: Pixel

}

impl SliderWidget {
    pub fn new(text_left: Option<String>, text_right: Option<String>, start: Rc<RefCell::<f64>>, min:f64, max: f64) -> Self {

        Self { text_left, text_right, in_motion: false, value: start, min, max, circle_r: 10}
    }

}

impl Widget for SliderWidget {

    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        let text_size = TextRenderer::render_box(ctx.font, "a", bc.max_w as f32, 1.0);

        // TODO: Implement infinite max width, here to let layout plugin sa, I don't care aboubt my width, but i want to be as wide as i can get to be
        LayoutResult::Size(Size {
            pixel_w: 300,
            pixel_h: Pixel::min(bc.max_h, Pixel::max(text_size.total_height as i32, bc.min_h))
        })
    }


    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_round_rect(geom, ctx);

        let circle_pos = (*self.value.borrow() - self.min) / (self.max - self.min) * geom.size.pixel_w as f64 ;
        let mut circle_geom = geom.clone();
        circle_geom.size.pixel_w = self.circle_r * 2;
        circle_geom.pos.x += circle_pos as Pixel - self.circle_r;

        render::render_circle(&circle_geom, 20, ctx);
    }


    fn dispatcher(&self) -> Dispatcher {
        Box::new(slider_dispatcher)
    }


    // this is basically just a dispatcher, by where we have access to self, and thus can store internal state
    fn handle_sdl_event(&mut self, event: &event::Event, geom: &Geometry, self_id: Id, queue: &mut DispatcherQueue) {
        use event::Event::*;
        match event {
            MouseButtonUp {..} => {
                queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
                self.in_motion = false;
            },
            MouseButtonDown { x, .. } => {
                // snap slider to current pos, register slider start, to react to mouse motion
                self.in_motion = true;
                let click_pos = (*x - geom.pos.x) as f64;
                *self.value.borrow_mut() = (click_pos / geom.size.pixel_w as f64) * (self.max - self.min);

            },

            MouseMotion { xrel, x, ..} => {
                if self.in_motion {


                    if *x <= geom.pos.x {
                        *self.value.borrow_mut() = self.min;
                    }

                    if *x >= (geom.pos.x + geom.size.pixel_w)  {
                        *self.value.borrow_mut() = self.max;
                    }

                    // check if position of mouse is inside slider, else ignore event
                    if *x >= geom.pos.x && *x <= (geom.pos.x + geom.size.pixel_w) {
                        let width_rel = (*xrel as f64 / geom.size.pixel_w as f64) * (self.max - self.min);
                        let new_value = f64::max(self.min, f64::min(self.max, *self.value.borrow() + width_rel));
                        *self.value.borrow_mut() = new_value;

                    }
                }
            },
            _ => {}
        };
    }
}




fn slider_dispatcher(event: &event::Event, self_id: Id, queue: &mut DispatcherQueue) {
    use event::Event::*;
    match event {
        MouseButtonUp {..} => {
            queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
        },

        _ => {}
    };
}

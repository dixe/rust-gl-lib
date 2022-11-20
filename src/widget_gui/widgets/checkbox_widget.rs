use crate::widget_gui::*;
use crate::widget_gui::render;


#[derive(Debug, Clone)]
pub struct CheckboxWidget {
    checked: bool,
}

impl CheckboxWidget {
    pub fn new(checked: bool) -> Self {

        Self { checked }
    }

}

impl Widget for CheckboxWidget {

    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], _ctx: &mut LayoutContext) -> LayoutResult {
        // set to max 25x, otherwise to the smallest w or h to keep square
        let s = Pixel::min(30, Pixel::min(bc.max_w, bc.max_h));
        LayoutResult::Size(Size {
            pixel_w: s,
            pixel_h: s,
        })
    }


    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {

        render::render_rect(geom, ctx);

        if self.checked {
            let circle_geom = geom.clone();
            render::render_circle(&circle_geom, geom.size.pixel_w / 2, ctx);
        }
    }


    // this is basically just a dispatcher, but where we have access to self, and thus can store internal state
    fn handle_sdl_event(&mut self, event: &event::Event, _geom: &Geometry, self_id: Id, queue: &mut WidgetOutputQueue) {
        use event::Event::*;
        match event {
            MouseButtonUp {..} => {
                self.checked = !self.checked;

                queue.push_back(WidgetOutput { widget_id: self_id, event: Box::new(self.checked)});

            },
            _ => {}
        };
    }
}

pub use crate::sdl_gui::components::base::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::{gl, shader::TransformationShader, objects::{RenderObject, square}, ScreenBox};
use std::fmt;
use crate::shader::rounded_rect_shader::{RoundedRectShader, Uniforms};


#[derive(Debug, Clone)]
pub struct Button<Message> {
    pub content: String, // Maybe use another compontent for content
    pub shader: RoundedRectShader,
    pub on_click_msg: Option<Message>,
    pub base: ComponentBase,
}


impl<Message> Button<Message> where Message: Clone {

    pub fn new(gl: &gl::Gl, content: &str, msg: Option<Message>) -> Box<Self> {

        let shader = RoundedRectShader::new(gl).unwrap();

        Box::new(Self {
            content: content.to_string(),
            shader,
            on_click_msg: msg,
            base: Default::default()
        })
    }
}

impl<Message> ComponentTrait<Message> for Button<Message> where Message: Clone + fmt::Debug {


    fn base(&self) -> &ComponentBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ComponentBase {
        &mut self.base
    }

    fn set_base(&mut self, base: ComponentBase) {
        self.base = base;
    }


    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        self.shader.shader.set_used();

        let transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);

        let color = self.base.color();

        self.shader.set_transform(transform);

        //println!(" h_half= {:?} w_half = {}", self.base.height / screen_h, self.base.width / screen_w);
        self.shader.set_uniforms(Uniforms { color,
                                            pixel_height: self.base.height,
                                            pixel_width: self.base.width,
                                            radius: 50.0
        });

        render_square.render(&gl);

        let button_screen_box = ScreenBox::new(self.base.x, self.base.y, self.base.width, self.base.height, screen_w, screen_h);

        tr.render_text(gl, &self.content, Default::default(), button_screen_box, 32);

    }

    fn update_content(&mut self, content: String) {
        self.content = content;
    }

    fn on_event(&self, event: ComponentEvent) -> Option<Message> {
        match event {
            ComponentEvent::Clicked(ClickType::Left, _) => self.on_click_msg.clone(),
            _ => None
        }
    }
}

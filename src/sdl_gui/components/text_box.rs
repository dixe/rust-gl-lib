pub use crate::sdl_gui::components::base::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::{gl, shader::{Shader, TransformationShader}, objects::square, ScreenBox};
use std::fmt;
use crate::shader::rounded_rect_shader::{RoundedRectShader, Uniforms};


#[derive(Debug,Clone)]
pub struct TextBox<Message> {
    pub content: String, // Maybe use another compontent for content
    pub shader: RoundedRectShader,
    pub on_input_msg: Option<Message>,
    pub base: ComponentBase,
}


impl<Message> TextBox<Message> where Message: Clone {

    pub fn new(gl: &gl::Gl, content: &str, msg: Option<Message>) -> Box<Self> {

        let shader = RoundedRectShader::new(gl).unwrap();

        Box::new(Self {
            content: content.to_string(),
            shader,
            on_input_msg: msg,
            base: Default::default()
        })
    }
}


impl<Message> ComponentTrait<Message> for TextBox<Message> where Message: Clone + fmt::Debug {

    fn base(&self) -> &ComponentBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ComponentBase {
        &mut self.base
    }

    fn set_base(&mut self, base: ComponentBase) {
        self.base = base;
    }

    fn update_content(&mut self, content: String) {
        self.content = content;
    }

    fn focus_on_click(&self) -> bool {
        true
    }

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {
        self.shader.shader.set_used();

        let transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);

        let color_scale = self.base.color_scale();

        self.shader.set_transform(transform);

        self.shader.set_uniforms(Uniforms { color_scale,
                                       h_half: self.base.height / screen_h,
                                       w_half: self.base.width / screen_w,
                                       radius: 0.3
        });

        render_square.render(&gl);

        let button_screen_box = ScreenBox::new(self.base.x, self.base.y, self.base.width, self.base.height, screen_w, screen_h);

        tr.render_text(gl, &self.content, Default::default(), button_screen_box, 1.0);
    }

    fn on_event(&self, event: ComponentEvent) -> Option<Message> {
        match event {
            // TODO: Use not clicked but something else
            ComponentEvent::AlphaNumChar(chr) => {
                println!("Pressed textBox {}", chr);
                self.on_input_msg.clone()
            },
            _ => None
        }

    }

}

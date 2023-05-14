pub use crate::sdl_gui::components::base::*;
use crate::shader::rounded_rect_shader::{RoundedRectShader, Uniforms};
use crate::text_rendering::text_renderer::{
    TextAlignment, TextAlignmentX, TextAlignmentY, TextRenderer,
};
use crate::{gl, objects::square, shader::TransformationShader, ScreenBox};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TextBox<Message> {
    pub content: String, // Maybe use another compontent for content
    pub shader: RoundedRectShader,
    pub on_change: fn(String) -> Message,
    pub base: ComponentBase,
}

impl<Message> TextBox<Message>
where
    Message: Clone,
{
    pub fn new(gl: &gl::Gl, content: &str, msg: fn(String) -> Message) -> Box<Self> {
        let shader = RoundedRectShader::new(gl).unwrap();

        Box::new(Self {
            content: content.to_string(),
            shader,
            on_change: msg,
            base: Default::default(),
        })
    }
}

impl<Message> ComponentTrait<Message> for TextBox<Message>
where
    Message: Clone + fmt::Debug,
{
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

    fn render(
        &self,
        gl: &gl::Gl,
        tr: &mut TextRenderer,
        render_square: &square::Square,
        screen_w: f32,
        screen_h: f32,
    ) {
        self.shader.shader.set_used();

        let transform = self
            .base
            .unit_square_transform_matrix(screen_w as f32, screen_h as f32);

        //TODO: don't use hover for text box. make this function take some flags to determine witch info is used. So we can ignore hover and only use disabled
        let color = self.base.color();

        self.shader.set_transform(transform);

        self.shader.set_uniforms(Uniforms {
            color,
            pixel_height: self.base.height,
            pixel_width: self.base.width,
            radius: 30.0
        });

        render_square.render(&gl);

        let button_screen_box = ScreenBox::new(
            self.base.x,
            self.base.y,
            self.base.width,
            self.base.height,
            screen_w,
            screen_h,
        );


        let align = TextAlignment {
            x: TextAlignmentX::Left,
            y: TextAlignmentY::Center,
        };
        tr.render_text(gl, &self.content, align, button_screen_box, 32);
    }

    fn on_event(&self, event: ComponentEvent) -> Option<Message> {
        match event {
            // Maybe handle all of this in the window, since we want to maybe capture some keys there??
            ComponentEvent::KeyboardInput(info) => {
                let val = info.keycode as i32;
                let kc = info.keycode as u8;
                if val < 256 {
                    // ascii char, do the easy thing
                    match kc {
                        // backspace
                        8 => {
                            let mut content = self.content.clone();
                            content.pop();
                            Some((self.on_change)(content))
                        },

                        // Sdl never send a uppercase char so 65 to 90 will never be seen here
                        // We can thus ignore the case
                        32..=96 => {
                            let mut content = self.content.clone();
                            content = content + &(kc as char).to_string();
                            Some((self.on_change)(content))
                        }
                        67..=122 => {
                            //check if mods combined should change case
                            // check shifts together and xor to upper, if also caps, the change down
                            let mut shift_mod = sdl2::keyboard::Mod::LSHIFTMOD;
                            shift_mod.set(sdl2::keyboard::Mod::RSHIFTMOD, true);


                            let mut upper = info.keymod.intersects(shift_mod);
                            println!("{:?}", (info.keymod, shift_mod, upper));
                            upper ^= info.keymod.contains(sdl2::keyboard::Mod::CAPSMOD);
                            let mut key = kc;
                            if upper {
                                key -= 32;
                            }

                            let mut content = self.content.clone();
                            content = content + &(key as char).to_string();
                            Some((self.on_change)(content))
                        },


                        _ => None,
                    }
                }
                else {
                    //println!("{:?}", (info.keycode, info.keycode as i32));
                    None
                }

            }
            _ => None,
        }
    }
}

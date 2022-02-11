pub use crate::sdl_gui::components::base::*;
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::{gl, shader::Shader, objects::square, ScreenBox};
use std::fmt;


#[derive(Debug,Clone)]
pub struct Button<Message> {
    pub content: String, // Maybe use another compontent for content
    pub shader: Shader,
    pub on_click_msg: Option<Message>,
    pub base: ComponentBase,
}


impl<Message> Button<Message> where Message: Clone {

    pub fn new(gl: &gl::Gl, content: &str, msg: Option<Message>) -> Box<Self> {

        let shader = default_shader(gl).unwrap();

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

        self.shader.set_used();

        let transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);


        let hover = if self.base.hover { 0.6 } else { 1.0 };

        self.shader.set_mat4(gl, "transform", transform);

        self.shader.set_f32(gl, "hover", hover);

        self.shader.set_f32(gl, "h_half", self.base.height / screen_h);

        self.shader.set_f32(gl, "w_half", self.base.width / screen_w);

        self.shader.set_f32(gl, "radius", 0.0); //0.1 / (f32::max(self.base.width / screen_w, self.base.height / screen_h)));

        render_square.render(&gl);

        let button_screen_box = ScreenBox::new(self.base.x, self.base.y, self.base.width, self.base.height, screen_w, screen_h);

        tr.render_text(gl, &self.content, Default::default(), button_screen_box, 1.0);

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

/*
impl<Message> From<Button<Message>> for Component<Message> where Message: Clone {

fn from(btn: Button<Message>) -> Self {
Self {
base: Default::default(),
comp_type: ComponentType::Btn(btn)
        }
    }
}
*/


/// Creates a basic default shader that takes a mat4 transformation uniform transform
pub fn default_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

    // default program for square
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

out VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} OUT;

void main()
{
    vec4 pos = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    OUT.FragPos = aPos.xy;
    OUT.Pos = aPos.xy;
    gl_Position = pos;

}";

    let frag_source = r"
#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float w_half;
uniform float h_half;

uniform float radius;

uniform float hover;

float roundedRectangle(vec2 uv, vec2 size, float radius, float thickness)
{
  float d = length(max(abs(uv), size) - size) - radius;
  return smoothstep(0.66, 0.33, d / thickness);
}



void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // multiply by 2 to get -1.0...1.0 range
    float u = IN.FragPos.x * 2.0;
    float v = IN.FragPos.y * 2.0;


    float aspect = w_half / h_half;

    vec2 uv = vec2(u * aspect, v);

    vec3 col = vec3(.8, 0.8, .8) * hover;

    // size = aspect - radius, 1.0 - radius
    vec2 size = vec2(aspect - radius, 1.0 - radius);

    // higher is more blur, and also thicker corners
    float aa = 0.05;
    float dist = roundedRectangle(uv, size, radius, aa);
    col =  col * dist;

    FragColor = vec4(col, smoothstep(0.9, 1.0, dist));

}";

    Shader::new(gl, vert_source, frag_source)
}


/*


insert into shader before FragColor to draw two diagonal lines to see center point
if ( abs(IN.Pos.x - IN.Pos.y) < 0.01 || abs(- IN.Pos.x - IN.Pos.y) < 0.01)
{
//col = vec3(1.0,0.0,0.0);
}
*/

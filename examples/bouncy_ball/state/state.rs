use gl_lib::{gl, na, objects::*, shader::{Shader, BaseShader}};

pub type Color = na::Vector3::<f32>;

pub struct State {
    square: square::Square,
    shader: BaseShader,
    color: Color,
    gl: gl::Gl,
}

impl State {
    pub fn new(gl: &gl::Gl) -> Self {
        Self {
            square: square::Square::new(gl),
            shader: shader(gl).unwrap(),
            color: na::Vector3::new(0.3, 0.5,0.9),
            gl: gl.clone(),
        }
    }

    pub fn render(&self) {
        self.shader.set_used();
        let transform = na::Matrix4::<f32>::identity();

        self.shader.set_mat4(&self.gl, "transform", transform);
        self.shader.set_vec3(&self.gl, "uColor", self.color);
        self.square.render(&self.gl);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }


    pub fn handle_sdl_event(&mut self, event: sdl2::event::Event) {
        use sdl2::event::Event;

        match event {
            Event::MouseButtonDown { x, y, ..} => {
                self.color = Color::new(x as f32 / 600.0, y as f32 / 600.0, 0.0);
            },
            _ => {}
        };

    }
}

pub fn shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {
    // default program for square
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;
uniform mat4 transform;
void main()
{
    gl_Position = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

    let frag_source = r"#version 330 core
                    uniform vec3 uColor;
                    out vec4 FragColor;
                    void main()
                    {
                        FragColor = vec4(uColor, 1.0f);
                    }";

    BaseShader::new(gl, vert_source, frag_source)
}

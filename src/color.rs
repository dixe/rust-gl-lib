use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    RgbA(u8, u8, u8, u8)
}

impl Color {

    pub fn as_vec4(&self) -> na::Vector4::<f32> {
        match *self {
            Color::Rgb(r,g,b) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0),
            Color::RgbA(r,g,b,a) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0),
        }
    }
}

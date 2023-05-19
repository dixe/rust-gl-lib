use crate::na;
use crate::color::Color;


pub struct Particle {
    pub pos: na::Vector2::<f32>,
    pub vel: na::Vector2::<f32>,
    pub total_life: f32,
    pub life: f32,
    pub size: f32,
    pub color: Color,
    pub color_from: Color,
    pub color_to: Color
}

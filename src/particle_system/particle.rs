use crate::na;
use crate::color::Color;
use crate::typedef::*;
use crate::imode_gui::drawer2d::Drawer2D;

pub trait Particle {

    fn total_life(&self) -> f32;

    fn set_total_life(&mut self, total: f32);

    fn life(&self) -> f32;

    fn set_life(&mut self, life: f32);

    fn update_life(&mut self, dt: f32);
}


pub struct ParticleCircle {
    pub pos: V3,
    pub vel: V3,
    pub total_life: f32,
    pub life: f32,
    pub size: f32,
    pub color: Color,
    pub color_from: Color,
    pub color_to: Color
}


impl std::default::Default for ParticleCircle {

    fn default() -> Self {
        Self {
            pos: Default::default(),
            vel: Default::default(),
            total_life: 0.0,
            life: 0.0,
            size: 1.0,
            color: Color::RgbA(0, 0, 0, 255),
            color_from: Color::RgbA(0, 0, 0, 255),
            color_to: Color::RgbA(0, 0, 0, 10)
        }
    }
}

impl Particle for ParticleCircle {

    fn set_life(&mut self, life: f32) {
        self.life = life;
    }

    fn update_life(&mut self, dt: f32) {
        self.life -= dt;
    }

    fn set_total_life(&mut self, life: f32) {
        self.total_life = life;
    }

    fn life(&self) -> f32 {
        self.life
    }

    fn total_life(&self) -> f32 {
        self.total_life
    }

}


pub fn render(p: &ParticleCircle, drawer2d: &mut Drawer2D) {
    drawer2d.circle(p.pos.x, p.pos.y, p.size, p.color);
}

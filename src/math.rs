use crate::na;
use std::ops::Mul;

pub fn lerp<T: std::ops::Mul<T2, Output = T> + std::ops::Add<Output = T>, T2>(a: T, b: T, t: T2) -> T
where T2: Mul<T, Output = T> + std::marker::Copy,
  f32: std::ops::Sub<T2, Output = T2>,
{
    let x :T = (1.0 - t) * a;
    let y :T = b * t;

    x + y
}


pub fn clamp01(t: f32, min: f32, max: f32) -> f32 {
    f32::max(f32::min(1.0, (t - min) / (max - min)), 0.0)
}


pub trait AsV2 {

    fn v2(&self) -> na::Vector2::<f32>;
}

impl AsV2 for na::Vector2::<i32> {

    fn v2(&self) -> na::Vector2::<f32> {
        na::Vector2::new(self.x as f32, self.y as f32)
    }

}


pub trait AsV2i {

    fn v2i(&self) -> na::Vector2::<i32>;
}

impl AsV2i for na::Vector2::<f32> {

    fn v2i(&self) -> na::Vector2::<i32> {
        na::Vector2::new(self.x as i32, self.y as i32)
    }

}

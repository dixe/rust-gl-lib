use crate::na;
use std::ops::Mul;

pub mod numeric;

use numeric::*;

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

impl<T : Numeric + std::fmt::Debug> AsV2 for na::Vector2::<T> {
    fn v2(&self) -> na::Vector2::<f32> {
        na::Vector2::new(self.x.to_f32(), self.y.to_f32())
    }
}

pub trait AsV2f64 {

    fn v2f64(&self) -> na::Vector2::<f64>;
}

impl<T : Numeric + std::fmt::Debug> AsV2f64 for na::Vector2::<T> {
    fn v2f64(&self) -> na::Vector2::<f64> {
        na::Vector2::new(self.x.to_f64(), self.y.to_f64())
    }
}


pub trait AsV2i {

    fn v2i(&self) -> na::Vector2::<i32>;
}

impl<T : Numeric + std::fmt::Debug> AsV2i for na::Vector2::<T> {

    fn v2i(&self) -> na::Vector2::<i32> {
        na::Vector2::new(self.x.to_f64() as i32, self.y.to_f64() as i32)
    }

}


pub trait Transform2 {
    fn transform(&self, transform: na::Matrix3::<f32>) -> Self;
}


impl Transform2 for na::Vector2::<f32> {
    fn transform(&self, transform: na::Matrix3::<f32>) -> Self {
        let transformed  = transform * self.homogeneous();
        transformed.xy() / transformed.z
    }
}


pub trait Homogeneous {

    fn homogeneous(&self) -> na::Vector3::<f32>;
}

impl Homogeneous for na::Vector2::<f32> {

    fn homogeneous(&self) -> na::Vector3::<f32> {
        self.push(1.0)
    }

}

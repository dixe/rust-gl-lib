use std::ops::Mul;

pub fn lerp<T: std::ops::Mul<T2, Output = T> + std::ops::Add<Output = T>, T2>(a: T, b: T, t: T2) -> T
where T2: Mul<T, Output = T> + std::marker::Copy,
  f32: std::ops::Sub<T2, Output = T2>,
{
    let x :T = (1.0 - t) * a;
    let y :T = b * t;

    x + y
}

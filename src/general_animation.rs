use crate::{math, na};
use serde::{Serialize, Deserialize};

type V2 = na::Vector2::<f32>;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Animation<T: Animatable> {
    pub frames: Vec::<Frame<T>>,
}

impl<T: Animatable> Animation<T> {


    pub fn frame(&self, frame: usize) -> T {
        self.frames[frame].data
    }

    pub fn total_seconds(&self) -> f32 {
        let mut seconds = 0.0;
        for i in 0..self.frames.len() {
            seconds += self.frames[i].frame_seconds;
        }
        seconds
    }

    pub fn at(&self, elapsed: f32) -> Option<(T, usize)> {
        let mut skipped = 0.0;
        for i in 0..self.frames.len() {
            skipped += self.frames[i].frame_seconds;
            if elapsed < skipped {

                let f1 = &self.frames[i];

                let f2 = if i == self.frames.len() - 1 {
                    &self.frames[0]
                } else {
                    &self.frames[i + 1]
                };

                // how far into the frame are we? Between 0 and 1
                let start = skipped - f1.frame_seconds;
                let end = skipped;

                let t = math::clamp01(elapsed,start, end);

                return Some((T::lerp(&f1.data, &f2.data, t), i));

            }
        }
        return None;
    }
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Frame<T: Animatable + Copy> {
    pub data:T,
    pub frame_seconds: f32,
}


pub trait Animatable : Copy {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

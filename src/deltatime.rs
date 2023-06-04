//! Library for calculating delta time. Most often used in games or simulations
//! # Example
use std::time::{
    Instant,
    Duration
};

pub struct Deltatime {
    value: Duration,
    last_update: Instant,
    time_speed: f32,

}

impl Deltatime {

    /// Instantiate a new delta time
    pub fn new() -> Self {
        Self {
            value: Duration::new(0, 0),
            last_update: Instant::now(),
            time_speed: 1.0
        }
    }


    /// Get the time passed
    pub fn time(&self) -> f32 {
        let ms = (self.value.as_millis() as f32 )/ 1000.0;
        ms  * self.time_speed
    }

    /// Update the time passed
    pub fn update(&mut self) {
        self.value = self.last_update.elapsed();
        self.last_update = Instant::now();
    }
}


impl Default for Deltatime {
    fn default() -> Self {
        Deltatime::new()
    }
}

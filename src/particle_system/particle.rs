pub trait Particle {

    fn total_life(&self) -> f32;

    fn set_total_life(&mut self, total: f32);

    fn life(&self) -> f32;

    fn set_life(&mut self, life: f32);

    fn update_life(&mut self, dt: f32);
}

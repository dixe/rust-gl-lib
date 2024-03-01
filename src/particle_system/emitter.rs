use crate::particle_system::particle::Particle;
use crate::particle_system::particle_circle::ParticleCircle;


use rand::Rng;
use crate::color::Color;
use crate::typedef::V3;

pub struct Emitter<T:  Particle + Sized> {
    particles: Vec::<T>,
    next_alive: usize,
    emit_fn: EmitFn<T>,
    update_fn: UpdateFn<T>
}


pub type EmitFn<T> = fn(&mut T, f32, f32);

pub type UpdateFn<T> = fn(&mut T, f32);

pub type RenderFn<T> = fn(&T);

impl<TParticle: std::default::Default + Particle + Sized> Emitter<TParticle> {
    pub fn new(max: usize, emit_fn: EmitFn<TParticle>, update_fn: UpdateFn<TParticle>) -> Self {

        let mut particles = vec![];

        for _ in 0..max {
            particles.push(TParticle::default());
        }

        Self {
            particles,
            next_alive: 0,
            emit_fn,
            update_fn,
        }
    }


    pub fn update(&mut self, dt: f32) {

        let mut i = 0;
        let max_particle_index = self.particles.len() - 1;
        while i < self.next_alive {
            // update
            self.particles[i].update_life(dt);
            if self.particles[i].life() < 0.0 {
                // if this is not the last particle to be checked, then swap with last, and decrement next_alive
                // bound to max_particle_index to avoid out of index error/panic
                let swap_idx = (if self.next_alive > 0 { self.next_alive - 1 } else { 0 }).min(max_particle_index);

                self.particles.swap(i, swap_idx);
                self.next_alive -= 1;
            } else {
                (self.update_fn)(&mut self.particles[i], dt);
                i += 1;
            }
        }
    }

    pub fn emit_from_fn<F : Fn(&mut TParticle, f32, f32)> (&mut self, x: f32, y: f32, emit_fn: F) {
        if self.next_alive >= self.particles.len() {
            return; // silently just not emit particle
        }

        (emit_fn)(&mut self.particles[self.next_alive], x, y);
        let life = self.particles[self.next_alive].life();
        self.particles[self.next_alive].set_total_life(life);
        self.next_alive += 1;
    }

    pub fn emit(&mut self, x: f32, y: f32) {
        if self.next_alive >= self.particles.len() {
            return; // silently just not emit particle
        }

        (self.emit_fn)(&mut self.particles[self.next_alive], x, y);
        self.next_alive += 1;
    }


    pub fn emit_new(&mut self, particle: TParticle) {
        if self.next_alive >= self.particles.len() {
            return; // silently just not emit particle
        }
        self.particles[self.next_alive] = particle;
        self.next_alive += 1;
    }


    pub fn draw_all(&self, mut render_fn: impl FnMut(&TParticle)) {
        for i in (0..self.next_alive).rev() {
            let p = &self.particles[i];
            render_fn(p)
        }
    }

    pub fn iter(&self) -> EmitterIter<TParticle>  {
        EmitterIter {
            next_alive: self.next_alive,
            current: 0,
            particles: &self.particles
        }
    }

}

pub struct EmitterIter<'a, T> {
    next_alive: usize,
    current: usize,
    particles: &'a Vec::<T>,
}


impl<'a, T> Iterator for EmitterIter<'a, T> {
    // We can refer to this type using Self::Item
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        self.current += 1;
        if self.current > self.next_alive {
            return None;
        }

        Some(&self.particles[self.current-1])
    }
}


pub fn update_1(p: &mut ParticleCircle, dt: f32) {

    let t = 1.0 - p.life / p.total_life;

    p.color = Color::lerp(p.color_from, p.color_to, t);

    p.pos = p.pos + p.vel * dt;
    p.size += dt;
    p.vel.x += dt;
    p.vel.y += dt;
}

pub fn emit_1(p: &mut ParticleCircle, x: f32, y: f32) {

    // TODO maybe have in struct
    let mut rng = rand::thread_rng();

    let angle : f32 = rng.gen::<f32>() * -std::f32::consts::PI;

    p.pos.x = x;
    p.pos.y = y;

    let x = angle.cos();
    let y = angle.sin();

    let dir = V3::new(x, y, 0.0).normalize();

    p.vel = dir * 40.0;

    p.life = 8.0;
    p.total_life = 8.0;

    // max size 10
    p.size = rng.gen::<f32>() * 1.1;
}

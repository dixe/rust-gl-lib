use crate::particle_system::particle::Particle;
use crate::imode_gui::drawer2d::Drawer2D;
use crate::gl;
use rand::Rng;
use crate::color::Color;


pub struct Emitter {
    particles: Vec::<Particle>,
    next_alive: usize,
    emit_fn: EmitFn,
    update_fn: UpdateFn
}


pub type EmitFn = fn(&mut Particle);

pub type UpdateFn = fn(&mut Particle, f32);

impl Emitter {
    pub fn new(max: usize, emit_fn: EmitFn, update_fn: UpdateFn) -> Self {

        let mut particles = vec![];

        for i in 0..max {
            particles.push(Particle {
                pos: Default::default(),
                vel: Default::default(),
                total_life: 0.0,
                life: 0.0,
                size: 1.0,
                color: Color::Rgb(0,0,0)
            });
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
        while i < self.next_alive {

            // update
            self.particles[i].life -= dt;

            if self.particles[i].life < 0.0 {
                self.particles.swap(i, self.next_alive);
                self.next_alive -= 1;
            } else {
                (self.update_fn)(&mut self.particles[i], dt);
                i += 1;
            }
        }
    }


    pub fn emit(&mut self) {

        if self.next_alive >= self.particles.len() {
            println!("Active particles full");
            return; // silently just not emit particle
        }

        (self.emit_fn)(&mut self.particles[self.next_alive]);
        self.next_alive += 1;

    }

    pub fn draw_all(&self, drawer2d: &Drawer2D) {
        for i in (0..self.next_alive).rev() {
            let p = &self.particles[i];
            drawer2d.circle(p.pos.x, p.pos.y, p.size, p.color);
        }
    }
}

pub fn update_1(p: &mut Particle, dt: f32) {
    p.pos = p.pos + p.vel * dt;
    p.size += dt;
    p.vel.x += dt;
    p.vel.y += dt;
}

pub fn emit_1(p: &mut Particle) {

    // TODO maybe have in struct
    let mut rng = rand::thread_rng();

    let angle : f32 = rng.gen::<f32>() * -std::f32::consts::PI;

    let x = angle.cos();
    let y = angle.sin();

    p.vel.x = x * 10.0;
    p.vel.y = y * 50.0;
    p.life = 8.0;
    p.total_life = 8.0;

    // spawn at 400, 400 for now
    p.pos.x = 400.0;
    p.pos.y = 400.0;

    // max size 10
    p.size = rng.gen::<f32>() * 1.1;

}
use crate::particle_system::particle::Particle;

use crate::scene_3d::MeshIndex;
use crate::typedef::V3;
use crate::scene_3d::RenderPipelineId;

// Static particles spawn at pos and keep drawing until life is up.
//
#[derive(Copy, Clone, Debug)]
pub struct ParticleScene {
    pub total_life: f32,
    pub life: f32,
    pub pos: V3,
    pub mesh_id: MeshIndex,
    pub render_pipeline_id: RenderPipelineId,
}



impl std::default::Default for ParticleScene {

    fn default() -> Self {
        Self {
            total_life: 0.0,
            life: 0.0,
            mesh_id: 0,
            pos: V3::new(0.0, 0.0, 0.0),
            render_pipeline_id: 0
        }
    }
}

impl Particle for ParticleScene {

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

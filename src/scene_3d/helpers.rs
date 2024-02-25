use crate::typedef::V3;
use crate::scene_3d::*;
use crate::na::Rotation2;

pub fn update_pos<A,B>(scene: &mut Scene<A,B>, id: EntityId, pos: V3) {
    if let Some(c) = scene.entities.get_mut(&id) {
        c.pos = pos;
    }
}


pub fn update_dir<A,B>(scene: &mut Scene<A,B>, id: EntityId, dir: V3) {
    if let Some(c) = scene.entities.get_mut(&id) {
        if dir.magnitude() > 0.0 {
                let new_angle = dir.y.atan2(dir.x);
            c.z_angle =  Rotation2::new(new_angle);
        }
    }
}

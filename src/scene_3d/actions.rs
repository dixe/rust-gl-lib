use std::rc::Rc;
use crate::scene_3d::scene_3d::EntityId;
use std::collections::VecDeque;



pub type ActionQueue = VecDeque::<Action>;

// Generic actions, so StartAnimation, Plays sound
// and not Attack, Roll ect.
pub enum Action {
    // TODO: Maybe don't use string, but use something morel lgiht weight like Rc::<str> or Anim or ids
    // but should still be easy for the user
    StartAnimation(EntityId, Rc::<str>, f32),
    StartAnimationLooped(EntityId, Rc::<str>, f32),
    PlaySound(Rc::<str>),
    //SpawnParticle(String"name", loc, other info if needed)
}

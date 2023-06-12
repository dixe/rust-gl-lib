use gl_lib::animations::sheet_animation::{Start, SheetAnimation, Sprite, SheetAnimationPlayer, AnimationId};
use gl_lib::typedef::*;
use crate::PlayerAssets;
use crate::inputs::Inputs;



pub enum EntityState {
    Idle(AnimationId),
    Attack(AnimationId),
    Roll(AnimationId),
    Recover(AnimationId),
}



pub struct Entity {
    pub state: EntityState,
    pub attack_counter: usize,
    pub pos: V2,
    pub vel: V2,
    pub inputs: Inputs,
    pub flip_y: f32
}

impl EntityState {
    pub fn animation_id(&self) -> AnimationId {
        match self {
            Self::Idle(id) => *id,
            Self::Recover(id) => *id,
            Self::Attack(id)=> *id,
            Self::Roll(id)=> *id,
        }
    }
}


// tell compiler that lifetime 'a (PlayerAssets) is atleast as long as 'b (AnimationPlayer)
pub fn update_entity<'a: 'b, 'b>(entity: &mut Entity,
                                 scale: f32,
                                 player_assets: &'a PlayerAssets,
                                 animation_player: &'b mut SheetAnimationPlayer<'a>,
                                 roll_speed: f32,
                                 dt: f32) {
    let flip_y = entity.flip_y < 0.0;
    match entity.state {
        EntityState::Idle(id) => {
            entity.vel.x = 0.0;
            if entity.inputs.left {
                entity.vel.x = -100.0;
                entity.flip_y = -1.0;
            }

            if entity.inputs.right {
                entity.vel.x = 100.0;
                entity.flip_y = 1.0;
            }


            if entity.inputs.attack() {
                let attack =  &player_assets.attack_1;
                entity.attack_counter = (entity.attack_counter + 1 ) % 2;
                entity.vel.x = 0.0;
                animation_player.remove(id);
                let anim_id = animation_player.start(Start {sheet: &attack, scale, repeat: false, flip_y});
                entity.state = EntityState::Attack(anim_id);
            }

            if entity.inputs.space {
                entity.vel.x = roll_speed;
                entity.vel.x *= entity.flip_y;
                animation_player.remove(id);
                let anim_id = animation_player.start(Start {sheet: &player_assets.roll, scale, repeat: false, flip_y});
                entity.state = EntityState::Roll(anim_id);
            }
        },
        EntityState::Recover(id) => {
            if animation_player.expired(id) {
                // TODO: Next state, could be run or attack, and not idle
                let anim_id = animation_player.start(Start {sheet: &player_assets.idle, scale, repeat: true, flip_y});
                entity.state = EntityState::Idle(anim_id);

                // clears input buffer for attack, if any
                entity.inputs.attack();
            }
        },
        EntityState::Attack(id) => {

            if animation_player.expired(id) {

                if entity.attack_counter > 0 && entity.inputs.attack() {
                    entity.attack_counter = (entity.attack_counter + 1) % 2;
                    let anim_id = animation_player.start(Start {sheet: &player_assets.attack_2, scale, repeat: false, flip_y});
                    entity.state = EntityState::Attack(anim_id);
                } else {
                    let sheet = if entity.attack_counter == 1 { &player_assets.attack_1_recover} else {&player_assets.attack_2_recover};
                    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                    entity.attack_counter = 0;
                    entity.state = EntityState::Recover(anim_id);
                }
            }
        },
        EntityState::Roll(id) => {
            if animation_player.expired(id) {
                let anim_id = animation_player.start(Start {sheet: &player_assets.idle, scale, repeat: true, flip_y});
                entity.state = EntityState::Idle(anim_id);
            }
        }
    }

    // update pos by vel
    entity.pos += entity.vel * dt;
}

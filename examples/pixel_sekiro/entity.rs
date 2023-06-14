use gl_lib::animations::sheet_animation::{Start, SheetAnimation, Sprite, SheetAnimationPlayer, SheetAssets, AnimationId};
use gl_lib::typedef::*;
use crate::PlayerAssets;
use crate::inputs::Inputs;
use crate::audio_player::AudioPlayer;
use std::collections::HashMap;


pub enum EntityState {
    Idle(AnimationId),
    Attack(AnimationId),
    Roll(AnimationId),
    Recover(AnimationId),
    Deflect(AnimationId),
}

pub type EntityId = usize;
pub type AttackId = usize;


pub trait Asset {
    fn attack(&self, asset_name: &str, counter: usize) -> &SheetAnimation;

    fn roll(&self, asset_name: &str) -> &SheetAnimation;

    fn attack_recover(&self,  asset_name: &str, counter: usize) -> &SheetAnimation;

    fn idle(&self, asset_name: &str) -> &SheetAnimation;

    fn deflect(&self, asset_name: &str) -> &SheetAnimation;
}

impl Asset for SheetAssets {
    fn attack(&self, asset_name: &str, counter: usize) -> &SheetAnimation {
        self.get(asset_name).unwrap().get("attack_1").unwrap()
    }

    fn deflect(&self, asset_name: &str) -> &SheetAnimation {
        self.get(asset_name).unwrap().get("deflect").unwrap()
    }

    fn attack_recover(&self,  asset_name: &str, counter: usize) -> &SheetAnimation {
        self.get(asset_name).unwrap().get("attack_1_recover").unwrap()
    }

    fn roll(&self, asset_name: &str) -> &SheetAnimation {
        self.get(asset_name).unwrap().get("roll").unwrap()
    }

    fn idle(&self, asset_name: &str) -> &SheetAnimation {
        self.get(asset_name).unwrap().get("idle").unwrap()
    }
}


pub struct Entity {
    pub id: EntityId,
    pub current_attack_id: AttackId,
    pub state: EntityState,
    pub attack_counter: usize,
    pub pos: V2,
    pub vel: V2,
    pub inputs: Inputs,
    pub flip_y: f32,
    pub asset_name: String,
    pub hit_map: HashMap::<EntityId, AttackId>,
    pub deflected: bool // if we deflected this frame
}

impl Entity {
    pub fn new(id: EntityId, state: EntityState, pos: V2, asset_name: String, flip_y: f32) -> Self {
        Self {
            id,
            state,
            attack_counter: 0,
            pos,
            vel: V2::identity(),
            inputs: Inputs::default(),
            flip_y,
            asset_name,
            hit_map: Default::default(),
            current_attack_id: 0
        }
    }

    pub fn new_attack(&mut self, anim_id: AnimationId) {
        self.current_attack_id += 1;
        self.attack_counter = (self.attack_counter + 1 ) % 2;
        self.vel.x = 0.0;
        self.state = EntityState::Attack(anim_id);
    }
}

impl EntityState {

    pub fn animation_id(&self) -> AnimationId {
        match self {
            Self::Idle(id) => *id,
            Self::Recover(id) => *id,
            Self::Attack(id)=> *id,
            Self::Roll(id)=> *id,
            Self::Deflect(id)=> *id,
        }
    }
}


pub fn deflected<'a: 'b, 'b>(
    entity: &mut Entity,
    scale: f32,
    assets: &'a SheetAssets,
    animation_player: &'b mut SheetAnimationPlayer<'a>) {

    // remove current animation
    animation_player.remove(entity.state.animation_id());

    let flip_y = entity.flip_y < 0.0;

    let sheet = &assets.idle(&entity.asset_name);
    let anim_id = animation_player.start(Start {sheet, scale, repeat: true, flip_y});

    entity.state = EntityState::Idle(anim_id);

}
// tell compiler that lifetime 'a (PlayerAssets) is atleast as long as 'b (AnimationPlayer)
pub fn update_entity<'a: 'b, 'b>(entity: &mut Entity,
                                 scale: f32,
                                 assets: &'a SheetAssets,
                                 animation_player: &'b mut SheetAnimationPlayer<'a>,
                                 roll_speed: f32,
                                 audio_player: &mut AudioPlayer,
                                 dt: f32) {
    let flip_y = entity.flip_y < 0.0;

    // reset deflected
    entity.deflected = false;

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
                animation_player.remove(id);
                let sheet = &assets.attack(&entity.asset_name, entity.attack_counter);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                entity.new_attack(anim_id);
            }

            if entity.inputs.deflect() {
                entity.deflected = true;
                entity.vel.x = 0.0;
                animation_player.remove(id);
                let sheet = &assets.deflect(&entity.asset_name);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                entity.state = EntityState::Deflect(anim_id);
            }

            if entity.inputs.space {
                entity.vel.x = roll_speed;
                entity.vel.x *= entity.flip_y;
                animation_player.remove(id);
                let sheet = &assets.roll(&entity.asset_name);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                entity.state = EntityState::Roll(anim_id);
            }
        },
        EntityState::Recover(id) => {
            if animation_player.expired(id) {
                // TODO: Next state, could be run or attack, and not idle
                let sheet = &assets.idle(&entity.asset_name);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: true, flip_y});
                entity.state = EntityState::Idle(anim_id);

                // clears input buffer for attack, if any
                entity.inputs.attack();
            }
        },
        EntityState::Attack(id) => {
            if animation_player.expired(id) {
                if entity.attack_counter > 0 && entity.inputs.attack() {

                    let sheet = &assets.attack(&entity.asset_name, entity.attack_counter);
                    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                    entity.new_attack(anim_id);

                } else {

                    let sheet = &assets.attack_recover(&entity.asset_name, entity.attack_counter);
                    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});

                    entity.attack_counter = 0;
                    entity.state = EntityState::Recover(anim_id);
                }
            }
        },
        EntityState::Deflect(id) => {
            if animation_player.expired(id) {
                // TODO: Next state, could be run or attack, and not idle
                let sheet = &assets.idle(&entity.asset_name);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: true, flip_y});
                entity.state = EntityState::Idle(anim_id);
            }
        },
        EntityState::Roll(id) => {
            if animation_player.expired(id) {
                let sheet = &assets.idle(&entity.asset_name);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: true, flip_y});
                entity.state = EntityState::Idle(anim_id);
            }
        }
    }

    // update pos by vel
    entity.pos += entity.vel * dt;
}

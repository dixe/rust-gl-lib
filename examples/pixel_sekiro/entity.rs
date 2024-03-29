#![allow(dead_code)]
use gl_lib::animations::sheet_animation::{Start, SheetAnimation, SheetAnimationPlayer, SheetAssets, AnimationId};
use gl_lib::typedef::*;
use crate::inputs::Inputs;
use crate::audio_player::AudioPlayer;
use std::collections::HashMap;
use crate::scene::FrameData;
use std::time::Instant;


pub enum EntityState {
    Idle(AnimationId),
    AttackWindup(AnimationId, Option<Deflection>),
    AttackDamage(AnimationId),
    AttackDeflected(AnimationId, bool),
    Roll(AnimationId),
    Recover(AnimationId),
    Deflect(AnimationId),
    Hurt(AnimationId),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Deflection {
    Interupt,
    Regular
}


impl Deflection {
    pub fn interupt(&self) -> bool {
        match self {
            Deflection::Regular => {
                false
            },
            Deflection::Interupt => {
                true
            }
        }
    }
}

pub type EntityId = usize;
pub type AttackId = usize;


pub trait Asset {
    fn attack_windup(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn attack_damage(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn attack_deflected(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn attack_recover(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn roll(&self, asset_name: &str) -> &SheetAnimation<FrameData>;

    fn hurt(&self, asset_name: &str) -> &SheetAnimation<FrameData>;

    fn idle(&self, asset_name: &str) -> &SheetAnimation<FrameData>;

    fn deflect(&self, asset_name: &str) -> &SheetAnimation<FrameData>;

    fn load_combo_asset(&self, combo: &Combo, format_str: &str, counter: usize) -> &SheetAnimation<FrameData>;
}

impl Asset for SheetAssets<FrameData> {

    fn attack_windup(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData> {
        self.load_combo_asset(combo, "windup", counter)
    }

    fn attack_damage(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData> {
        self.load_combo_asset(combo, "damage", counter)
    }

    fn attack_deflected(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData> {
        self.load_combo_asset(combo, "deflected", counter)
    }

    fn deflect(&self, asset_name: &str) -> &SheetAnimation<FrameData> {
        self.get(asset_name).unwrap().get("deflect").unwrap()
    }

    fn attack_recover(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData> {
        self.load_combo_asset(combo, "recover", counter)
    }

    fn roll(&self, asset_name: &str) -> &SheetAnimation<FrameData> {
        self.get(asset_name).unwrap().get("roll").unwrap()
    }

    fn hurt(&self, asset_name: &str) -> &SheetAnimation<FrameData> {
        self.get(asset_name).unwrap().get("hurt").unwrap()
    }

    fn idle(&self, asset_name: &str) -> &SheetAnimation<FrameData> {
        self.get(asset_name).unwrap().get("idle").unwrap()
    }

    fn load_combo_asset(&self, combo: &Combo, format_str: &str, counter: usize) -> &SheetAnimation<FrameData> {
        // create a string like "attack_1_windup",
        let asset_str = format!("{}_{counter}_{format_str}", &combo.combo_name);


        self.get(&combo.asset_name).unwrap().get(&asset_str).unwrap()
    }
}


pub struct Combo {
    pub attacks: usize,
    pub asset_name: String,
    pub combo_name: String,
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
    pub deflected: bool, // if we deflected this frame
    pub combos: Vec::<Combo>,
    pub active_combo: usize,
    pub last_attack_time: Instant
}

impl Entity {
    pub fn new(id: EntityId, state: EntityState, pos: V2, asset_name: String, flip_y: f32, combos: Vec::<Combo>) -> Self {
        let _combo_asset_name = asset_name.clone();
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
            current_attack_id: 0,
            deflected: false,
            combos,
            active_combo: 0,
            last_attack_time: Instant::now()
        }
    }


    pub fn has_next_combo_attack(&self) -> bool {
        self.attack_counter < self.combos[self.active_combo].attacks
    }

    pub fn reset_combo(&mut self) {
        self.attack_counter = 0;
    }
}

impl EntityState {

    pub fn animation_id(&self) -> AnimationId {
        match self {
            Self::Idle(id) => *id,
            Self::Recover(id) => *id,
            Self::AttackWindup(id, _)=> *id,
            Self::AttackDamage(id)=> *id,
            Self::AttackDeflected(id, _)=> *id,
            Self::Roll(id)=> *id,
            Self::Deflect(id)=> *id,
            Self::Hurt(id)=> *id,
        }
    }

    pub fn set_deflected(&mut self, def: Deflection) {
        match self {
            Self::AttackWindup(id, _) => {
                *self = Self::AttackWindup(*id, Some(def));
            },
            _ => {}
        }
    }
}


pub fn entity_idle<'a: 'b, 'b>( entity: &mut Entity,
                                  assets: &'a SheetAssets<FrameData>,
                                  animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>) {
    animation_player.remove(entity.state.animation_id());
    let sheet = &assets.idle(&entity.asset_name);
    let scale = 4.0;
    let flip_y = entity.flip_y < 0.0;
    let anim_id = animation_player.start(Start {sheet, scale, repeat: true, flip_y});
    entity.state = EntityState::Idle(anim_id);
}

pub fn entity_hurt<'a: 'b, 'b>( entity: &mut Entity,
                                  assets: &'a SheetAssets<FrameData>,
                                  animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>) {
    animation_player.remove(entity.state.animation_id());
    let sheet = &assets.hurt(&entity.asset_name);
    let scale = 4.0;
    let flip_y = entity.flip_y < 0.0;
    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
    entity.state = EntityState::Hurt(anim_id);
    entity.reset_combo();
    // clear attack buffer
    entity.inputs.attack();
    // clear deflect buffer
    entity.inputs.deflect();

}



pub fn entity_attack<'a: 'b, 'b>( entity: &mut Entity,
                                  assets: &'a SheetAssets<FrameData>,
                                  animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>) {

    // update attack id so enemies on hit knows to take damage
    entity.current_attack_id += 1;
    entity.attack_counter += 1 % (1 + entity.combos[entity.active_combo].attacks);
    entity.vel.x = 0.0;

    entity.last_attack_time = Instant::now();

    animation_player.remove(entity.state.animation_id());
    let sheet = &assets.attack_windup(&entity.combos[entity.active_combo], entity.attack_counter);
    let scale = 4.0;
    let flip_y = entity.flip_y < 0.0;
    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
    entity.state = EntityState::AttackWindup(anim_id, None);

}


// tell compiler that lifetime 'a (Assets) is atleast as long as 'b (AnimationPlayer)
pub fn update_entity<'a: 'b, 'b>(entity: &mut Entity,
                                 scale: f32,
                                 assets: &'a SheetAssets<FrameData>,
                                 animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>,
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
                entity_attack(entity, assets, animation_player);
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
        EntityState::AttackWindup(id, deflection) => {

            if animation_player.expired(id) {
                // attack deflected
                if let Some(def) = deflection {
                    audio_player.play_sound("deflect".into());

                    let interupt = def.interupt();
                    let sheet = &assets.attack_deflected(&entity.combos[entity.active_combo], entity.attack_counter);
                    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                    entity.state = EntityState::AttackDeflected(anim_id, interupt);

                } else { // attack should damage, if hit

                    let sheet = &assets.attack_damage(&entity.combos[entity.active_combo], entity.attack_counter);
                    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                    entity.state = EntityState::AttackDamage(anim_id);
                }
            }
        },
        EntityState::AttackDamage(id) => {
            if animation_player.expired(id) {
                damage_finished(entity, assets, animation_player);
            }
        },
        EntityState::AttackDeflected(id, interupt) => {
            if animation_player.expired(id) {
                if !interupt && entity.has_next_combo_attack() && entity.inputs.attack() {
                    entity_attack(entity, assets, animation_player);
                } else {
                    entity.reset_combo();
                    entity_idle(entity, assets, animation_player);
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
        },
        EntityState::Hurt(id) => {
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

pub fn damage_finished<'a: 'b, 'b>( entity: &mut Entity,
                                    assets: &'a SheetAssets<FrameData>,
                                    animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>) {
    if entity.has_next_combo_attack() && entity.inputs.attack() {
        entity_attack(entity, assets, animation_player);
    } else {

        let scale = 4.0;
        let flip_y = entity.flip_y < 0.0;
        let sheet = &assets.attack_recover(&entity.combos[entity.active_combo], entity.attack_counter);
        let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});

        entity.reset_combo();
        entity.state = EntityState::Recover(anim_id);
    }

}

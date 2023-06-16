#![allow(dead_code)]
use gl_lib::animations::sheet_animation::{Start, SheetAnimation, SheetAnimationPlayer, SheetAssets, AnimationId};
use gl_lib::typedef::*;
use crate::inputs::Inputs;
use crate::audio_player::AudioPlayer;
use std::collections::HashMap;
use crate::scene::FrameData;

pub enum EntityState {
    Idle(AnimationId),
    AttackWindup(AnimationId),
    AttackDamage(AnimationId),
    Roll(AnimationId),
    Recover(AnimationId),
    Deflect(AnimationId),
}

pub type EntityId = usize;
pub type AttackId = usize;


pub trait Asset {
    fn attack_windup(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn attack_damage(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn roll(&self, asset_name: &str) -> &SheetAnimation<FrameData>;

    fn attack_recover(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

    fn idle(&self, asset_name: &str) -> &SheetAnimation<FrameData>;

    fn deflected(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData>;

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

    fn deflected(&self, combo: &Combo, counter: usize) -> &SheetAnimation<FrameData> {
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

    fn idle(&self, asset_name: &str) -> &SheetAnimation<FrameData> {
        self.get(asset_name).unwrap().get("idle").unwrap()
    }

    fn load_combo_asset(&self, combo: &Combo, format_str: &str, counter: usize) -> &SheetAnimation<FrameData> {
        // create a string like "attack_1_windup,
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
    pub active_combo: usize
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
            active_combo: 0
        }
    }

}

impl EntityState {

    pub fn animation_id(&self) -> AnimationId {
        match self {
            Self::Idle(id) => *id,
            Self::Recover(id) => *id,
            Self::AttackWindup(id)=> *id,
            Self::AttackDamage(id)=> *id,
            Self::Roll(id)=> *id,
            Self::Deflect(id)=> *id,
        }
    }
}


pub fn entity_attack<'a: 'b, 'b>( entity: &mut Entity,
                                  assets: &'a SheetAssets<FrameData>,
                                  animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>) {

    // update attack id so enemies on hit knows to take damage
    entity.current_attack_id += 1;
    entity.attack_counter += 1 % (1 + entity.combos[entity.active_combo].attacks);
    entity.vel.x = 0.0;

    animation_player.remove(entity.state.animation_id());
    let sheet = &assets.attack_windup(&entity.combos[entity.active_combo], entity.attack_counter);
    let scale = 4.0;
    let flip_y = entity.flip_y < 0.0;
    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
    entity.state = EntityState::AttackWindup(anim_id);

}


pub fn deflected<'a: 'b, 'b>(
    entity: &mut Entity,
    scale: f32,
    assets: &'a SheetAssets<FrameData>,
    animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>,
    interupt: bool
) {

    // remove current animation
    animation_player.remove(entity.state.animation_id());


    let flip_y = entity.flip_y < 0.0;

    let sheet = &assets.deflected(&entity.combos[entity.active_combo], entity.attack_counter);
    let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});

    if interupt {
        entity.attack_counter = 0;
    }

    entity.state = EntityState::Recover(anim_id);

}
// tell compiler that lifetime 'a (Assets) is atleast as long as 'b (AnimationPlayer)
pub fn update_entity<'a: 'b, 'b>(entity: &mut Entity,
                                 scale: f32,
                                 assets: &'a SheetAssets<FrameData>,
                                 animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>,
                                 roll_speed: f32,
                                 _audio_player: &mut AudioPlayer,
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
        EntityState::AttackWindup(id) => {
            if animation_player.expired(id) {
                // if deflected then play deflected animation, if deflected interupt play interupt, otherwise play damagex
                let sheet = &assets.attack_damage(&entity.combos[entity.active_combo], entity.attack_counter);
                let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                entity.state = EntityState::AttackDamage(anim_id);
            }
        },
        EntityState::AttackDamage(id) => {
            if animation_player.expired(id) {
                damage_finished(entity, assets, animation_player);
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

pub fn damage_finished<'a: 'b, 'b>( entity: &mut Entity,
                                    assets: &'a SheetAssets<FrameData>,
                                    animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>) {
    if entity.attack_counter < entity.combos[entity.active_combo].attacks && entity.inputs.attack() {
        entity_attack(entity, assets, animation_player);
    } else {

        let scale = 4.0;
        let flip_y = entity.flip_y < 0.0;
        let sheet = &assets.attack_recover(&entity.combos[entity.active_combo], entity.attack_counter);
        let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});

        entity.attack_counter = 0;
        entity.state = EntityState::Recover(anim_id);
    }

}

#![allow(dead_code)]
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use crate::{inputs::{self}, entity::{self, Entity, EntityState, Combo, EntityId, update_entity, Deflection}};
use gl_lib::animations::sheet_animation::{Start, SheetAnimationPlayer, AnimationId, SheetAssets};
use gl_lib::typedef::*;
use gl_lib::collision2d::polygon::{PolygonTransform};
use crate::ai;
use crate::audio_player::AudioPlayer;

#[derive(Clone, Copy, Debug)]
pub struct FrameData {
    pub deflect: bool, // can register deflect
    pub deflect_interupt: bool // can register deflect, and will interrupt, so new animation is
}

pub fn frame_data_mapper(input: &str) -> FrameData {
    let deflect = input == "deflect";
    let deflect_interupt = input == "deflect_interupt";
    FrameData {
        deflect,
        deflect_interupt
    }
}


pub struct Scene<'a: 'b, 'b> {
    pub player: Entity,
    pub enemy: Option<Entity>,
    pub animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>,
    pub assets: &'a SheetAssets<FrameData>,
    pub show_col_boxes: bool,
    pub hits: usize,
    audio_player: AudioPlayer,
    next_entity_id: EntityId,
    scale: f32
}

pub fn new<'a: 'b, 'b>(animation_player: &'b mut SheetAnimationPlayer<'a, FrameData>,
                       assets: &'a SheetAssets<FrameData>,
                       audio_player: AudioPlayer) -> Scene<'a, 'b> {

    let scale = 4.0;
    let id = 1;
    let player_idle = assets.get("player").unwrap().get("idle").unwrap();


    // player attack combos
    let combos = vec![
        Combo {
            attacks: 2,
            asset_name: "player".to_string(),
            combo_name: "attack".to_string(),
        }];


    Scene {
        player : Entity::new(
            id,
            EntityState::Idle(animation_player.start(Start {sheet: player_idle, scale, repeat: true, flip_y: false})),
            V2::new(400.0, 600.0),
            "player".to_string(),
            1.0,
            combos),
        enemy: None,
        animation_player,
        assets,
        show_col_boxes: true,
        hits: 0,
        audio_player,
        next_entity_id: 2, // player is 1
        scale
    }
}


impl<'a: 'b, 'b> Scene<'a, 'b> {

    pub fn add_enemy(&mut self, name: &str, pos: V2) {
        let idle = self.assets.get(name).unwrap().get("idle").unwrap();
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        let combos = vec![
            Combo {
                attacks: 1,
                asset_name: name.to_string(),
                combo_name: "attack".to_string(),
            },

            Combo {
                attacks: 2,
                asset_name: name.to_string(),
                combo_name: "stab".to_string(),
            },
        ];

        let mut e = Entity::new(
            id,
            EntityState::Idle(self.animation_player.start(Start {sheet: idle, scale: self.scale, repeat: true, flip_y: true})),
            pos,
            name.to_string(),
            -1.0,
            combos);

        e.active_combo = 1;

        self.enemy = Some(e);
    }


    pub fn update(&mut self, ui: &mut Ui, dt: f32) {

        let roll_speed = 150.0;

        inputs::handle_inputs(ui, &mut self.player.inputs);
        update_entity(&mut self.player, self.scale, self.assets, self.animation_player, roll_speed, &mut self.audio_player, dt);

        if let Some(ref mut enemy) = self.enemy {
            // run ai to update inputs
            ai::skeleton_logic(enemy);
            update_entity(enemy, self.scale, self.assets, self.animation_player, roll_speed, &mut self.audio_player, dt);
        }

        // update flip  -- maybe do in normal match statement
        match self.player.state {
            EntityState::Idle(id) => {
                self.animation_player.flip_y(id, self.player.flip_y < 0.0);
            },
            // cannot rotate mid attack/roll
            _ => {}
        }


        // resolve deflections and update animations accordingly
        self.resolve_deflect();


        // resolve collisions
        self.collisions(ui);
    }

    pub fn draw(&self, drawer_2d: &mut Drawer2D) {
        self.animation_player.draw(drawer_2d, self.player.pos, self.player.state.animation_id());
        if let Some(enemy) = &self.enemy {
            self.animation_player.draw(drawer_2d, enemy.pos, enemy.state.animation_id());
        }
    }


    fn resolve_deflect(&mut self) {
        if let Some(ref mut enemy) = &mut self.enemy {
            // resolve for player

            if self.player.deflected {
                // TODO: check if in range of any enemies and facing them, i.e can deflect any
                // for now assume true

                //println!("{:?}",self.animation_player.get_framedata(enemy.state.animation_id()));

                let frame = self.animation_player.frame(enemy.state.animation_id()).unwrap();

                if let Some(&enemy_framedata) = self.animation_player.get_framedata(enemy.state.animation_id()) {
                    if enemy_framedata.deflect || enemy_framedata.deflect_interupt {
                        println!("Registed deflect at frame {}", frame);
                        let def = if enemy_framedata.deflect_interupt { Deflection::Interupt} else {Deflection::Regular };
                        enemy.state.set_deflected(def)
                    }
                }
            }
        }
    }

    fn collisions(&mut self, ui: &mut Ui,) {

        if let Some(ref mut enemy) = &mut self.enemy {
            if hit(&self.animation_player, ui, &self.player, enemy, self.show_col_boxes) {
                self.hits += 1;
            }

            if hit(&self.animation_player, ui, &enemy, &mut self.player, self.show_col_boxes) {
                println!("Player hit");
                entity::entity_hurt(&mut self.player, &self.assets, &mut self.animation_player);
            }
        }
    }

    pub fn destroy(self) -> AudioPlayer {
        self.audio_player
    }
}


fn deflect(animation_player: &SheetAnimationPlayer<FrameData>, ui: &mut Ui, deflector: &Entity, attacker: &Entity, show_col_boxes: bool) -> bool {
    let ct = CollisionTest {
        animation_player: animation_player,
        actor_1: deflector.state.animation_id(),
        actor_1_pos: deflector.pos,
        actor_1_name: &"deflect",
        actor_2: attacker.state.animation_id(),
        actor_2_pos: attacker.pos,
        actor_2_name: &"attack",
    };

    ui.drawer2D.z = 2.0;
    let res = collide_draw(ui, &ct, show_col_boxes);
    ui.drawer2D.z = 0.0;

    res
}



fn hit(animation_player: &SheetAnimationPlayer<FrameData>, ui: &mut Ui, attacker: &Entity, target: &mut Entity, show_col_boxes: bool) -> bool {

    let ct = CollisionTest {
        animation_player: animation_player,
        actor_1: target.state.animation_id(),
        actor_1_pos: target.pos,
        actor_1_name: &"body",
        actor_2: attacker.state.animation_id(),
        actor_2_pos: attacker.pos,
        actor_2_name: &"attack",
    };

    ui.drawer2D.z = 2.0;
    let mut res = collide_draw(ui, &ct, show_col_boxes);
    ui.drawer2D.z = 0.0;

    if res {
        if let Some(last_hit_id) = target.hit_map.get(&attacker.id) {
            // if current hit has been registered, don't allow it to be again
            res = *last_hit_id != attacker.current_attack_id;
        }
    }

    // update target hit map to register hit
    if res {
        if !target.hit_map.contains_key(&attacker.id) {
            target.hit_map.insert(attacker.id, attacker.current_attack_id);
        } else {
            let hit_id = target.hit_map.get_mut(&attacker.id).unwrap();
            *hit_id = attacker.current_attack_id;
        }
    }

    res

}



struct CollisionTest<'a, 'b> {
    animation_player: &'a SheetAnimationPlayer<'a, FrameData>,
    actor_1: AnimationId,
    actor_1_pos: V2,
    actor_1_name: &'b str,
    actor_2: AnimationId,
    actor_2_pos: V2,
    actor_2_name: &'b str,
}


fn collide_draw(ui: &mut Ui, ct: &CollisionTest, draw: bool) -> bool {

    let mut res = false;
    if let Some((actor_1, actor_1_scale, actor_1_flip_y)) = ct.animation_player.get_polygon(ct.actor_1, ct.actor_1_name) {

        let mut actor_1_transform = PolygonTransform::default();
        actor_1_transform.scale = actor_1_scale;
        actor_1_transform.translation = ct.actor_1_pos;
        actor_1_transform.flip_y = actor_1_flip_y;

        if draw {
            ui.view_polygon(&actor_1.polygon, &actor_1_transform);
        }

        if let Some((attack, actor_2_scale, actor_2_flip_y)) = ct.animation_player.get_polygon(ct.actor_2, ct.actor_2_name) {

            let frame = ct.animation_player.frame(ct.actor_2);
            if let Some(f) = frame {
                if f == 3 {
                    let _dbug = 2;
                }
            }

            let mut actor_2_transform = PolygonTransform::default();
            actor_2_transform.scale = actor_2_scale;
            actor_2_transform.translation = ct.actor_2_pos;
            actor_2_transform.flip_y = actor_2_flip_y;

            res = attack.collide_draw(&mut ui.drawer2D, &actor_2_transform.mat3(), actor_1, &actor_1_transform.mat3());


            if draw {
                ui.view_polygon(&attack.polygon, &actor_2_transform);
            }
        }
    }

    res
}


fn skeleton_logic(entity: &mut Entity, ) {
    let attack_r = rand::random::<f32>();

    if attack_r > 0.9 {
        entity.inputs.set_attack();
    }
}



/*

fn collide_draw<T: CollisionTest>(ui: &mut Ui, test: &CollisionTest, draw: bool) -> bool {

let mut res = false;



if let Some(passive)(passive_polygon, passive_anim_id, passive_scale, passive_flip_y)) = ct.passive_info() {      //ct.animation_player.get_polygon(ct.actor_1, "body") {{

let mut passive_transform = PolygonTransform::default();
passive_transform.scale = passive.scale;1

passive_transform.translation = passive.pos;
passive_transform.flip_y = passive.flip_y;

if draw {
ui.view_polygon(&passive.polygon, &passive_transform);
        }

        if let Some((active, active_scale, active_flip_y)) = ct.passive_polygon() { // ct.animation_player.get_polygon(ct.attacker, "attack") {

            let frame = ct.animation_player.frame(ct.active);
            if let Some(f) = frame {
                if f == 3 {
                    let dbug = 2;
                }
            }

            let mut active_transform = PolygonTransform::default();
            active_transform.scale = active_scale;
            active_transform.translation = ct.active_pos;
            active_transform.flip_y = active_flip_y;

            res = active.collide_draw(&mut ui.drawer2D, &active_transform.mat3(), passive, &passive_transform.mat3());

            if let Some(f) = frame {
                if !res && f == 3 {
                    let f2 = ct.animation_player.frame(ct.passive);
                    let dbug = 2;
                    println!("{:?}", f2);
                }
            }

            if draw {
                ui.view_polygon(&active.polygon, &active_transform);
            }
        }
    }

    res
}


*/

use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use crate::{inputs::{self}, entity::*};
use gl_lib::animations::sheet_animation::{Start, SheetAnimation, Sprite, SheetAnimationPlayer, AnimationId, SheetAssets};
use gl_lib::typedef::*;
use crate::PlayerAssets;
use gl_lib::collision2d::polygon::{PolygonTransform, ComplexPolygon};
use crate::ai;



pub struct Scene<'a: 'b, 'b> {
    pub player: Entity,
    pub enemy: Option<Entity>,
    pub player_assets: &'a PlayerAssets,
    pub animation_player: &'b mut SheetAnimationPlayer<'a>,
    pub assets: &'a SheetAssets,
    pub show_col_boxes: bool,
    pub hits: usize,
}

pub fn new<'a: 'b, 'b>(player_assets: &'a PlayerAssets,
                       animation_player: &'b mut SheetAnimationPlayer<'a>,
                       assets: &'a SheetAssets) -> Scene<'a, 'b> {
    let scale = 4.0;
    Scene {
        player :Entity {
            state: EntityState::Idle(animation_player.start(Start {sheet: &player_assets.idle, scale, repeat: true, flip_y: false})),
            attack_counter: 0,
            pos: V2::new(400.0, 600.0),
            vel: V2::identity(),
            inputs: inputs::Inputs::default(),
            flip_y: 1.0,
            asset_name: "player".to_string()
        },
        enemy: None,
        animation_player,
        player_assets,
        assets,
        show_col_boxes: true,
        hits: 0
    }
}


impl<'a: 'b, 'b> Scene<'a, 'b> {

    pub fn add_enemy(&mut self, name: &str, pos: V2) {
        let scale = 4.0;
        let idle = self.assets.get(name).unwrap().get("idle").unwrap();
        self.enemy = Some(Entity {
            state: EntityState::Idle(self.animation_player.start(Start {sheet: idle, scale, repeat: true, flip_y: true})),
            attack_counter: 0,
            pos,
            vel: V2::identity(),
            inputs: inputs::Inputs::default(),
            flip_y: -1.0,
            asset_name: name.to_string()
        });
    }


    pub fn update(&mut self, ui: &mut Ui, dt: f32) {

        let scale = 4.0;
        let roll_speed = 150.0;

        inputs::handle_inputs(ui, &mut self.player.inputs);
        update_entity(&mut self.player, scale, self.assets, self.animation_player, roll_speed, dt);

        if let Some(ref mut enemy) = self.enemy {
            // run ai to update inputs
            ai::skeleton_logic(enemy);
            update_entity(enemy, scale, self.assets, self.animation_player, roll_speed, dt);
        }

        // update flip  -- maybe do in normal match statement
        match self.player.state {
            EntityState::Idle(id) => {
                self.animation_player.flip_y(id, self.player.flip_y < 0.0);
            },
            // cannot rotate mid attack/roll
            _ => {}
        }


        // resolve collisions
        self.collisions(ui);
    }

    pub fn draw(&self, drawer2D: &mut Drawer2D) {
        self.animation_player.draw(drawer2D, self.player.pos, self.player.state.animation_id());
        if let Some(enemy) = &self.enemy {
            self.animation_player.draw(drawer2D, enemy.pos, enemy.state.animation_id());
        }
    }


    fn collisions(&mut self, ui: &mut Ui,) {

        if let Some(ref mut enemy) = &mut self.enemy {


            // check player deflect

            if deflect(&self.animation_player, ui, &self.player, &enemy, self.show_col_boxes) {
                println!("player DEFLECT enemy");
                let scale = 4.0;
                deflected(enemy, scale, &self.assets, self.animation_player);
                // update deflected to be in recover, so the attack cannot hit


            } else if deflect(&self.animation_player, ui, &enemy, &self.player, self.show_col_boxes){
                // update deflected to be in recover, so the attack cannot hit
            }

            if hit(&self.animation_player, ui, &self.player, &enemy, self.show_col_boxes) {
                self.hits += 1;

            }

            if hit(&self.animation_player, ui, &enemy, &self.player, self.show_col_boxes) {
                println!("enemy hit player");
            }
        }
    }



}


fn deflect(animation_player: &SheetAnimationPlayer, ui: &mut Ui, deflector: &Entity, attacker: &Entity, show_col_boxes: bool) -> bool {
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



fn hit(animation_player: &SheetAnimationPlayer, ui: &mut Ui, attacker: &Entity, target: &Entity, show_col_boxes: bool) -> bool {

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
    let res = collide_draw(ui, &ct, show_col_boxes);
    ui.drawer2D.z = 0.0;

    res

}



struct CollisionTest<'a, 'b> {
    animation_player: &'a SheetAnimationPlayer<'a>,
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
                    let dbug = 2;
                }
            }

            let mut actor_2_transform = PolygonTransform::default();
            actor_2_transform.scale = actor_2_scale;
            actor_2_transform.translation = ct.actor_2_pos;
            actor_2_transform.flip_y = actor_2_flip_y;

            res = attack.collide_draw(&mut ui.drawer2D, &actor_2_transform.mat3(), actor_1, &actor_1_transform.mat3());

            if let Some(f) = frame {
                if !res && f == 3 {
                    let f2 = ct.animation_player.frame(ct.actor_1);
                    let debug = 2;
                    println!("{:?}", f2);
                }
            }

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
passive_transform.scale = passive.scale;
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

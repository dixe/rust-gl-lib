use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use crate::{inputs::{self}, entity::*};
use gl_lib::animations::sheet_animation::{Start, SheetAnimation, Sprite, SheetAnimationPlayer, AnimationId, SheetAssets};
use gl_lib::typedef::*;
use crate::PlayerAssets;
use gl_lib::collision2d::polygon::{PolygonTransform, ComplexPolygon};

pub struct Scene<'a: 'b, 'b> {
    pub player: Entity,
    pub enemy: Option<Entity>,
    pub player_assets: &'a PlayerAssets,
    pub animation_player: &'b mut SheetAnimationPlayer<'a>,
    pub assets: &'a SheetAssets,
    pub show_col_boxes: bool,
    pub hits: usize
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

        inputs::handle_inputs(ui, &mut self.player.inputs);

        // TODO: Update enemy inpus

        let scale = 4.0;
        let roll_speed = 150.0;

        update_entity(&mut self.player, scale, self.assets, self.animation_player, roll_speed, dt);

        if let Some(ref mut enemy) = self.enemy {
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


        self.collisions(ui);
    }

    pub fn draw(&self, drawer2D: &mut Drawer2D) {
        self.animation_player.draw(drawer2D, self.player.pos, self.player.state.animation_id());
        if let Some(enemy) = &self.enemy {
            self.animation_player.draw(drawer2D, enemy.pos, enemy.state.animation_id());
        }
    }


    fn collisions(&mut self, ui: &mut Ui,) {

        if let Some(enemy) = &self.enemy {
            if self.hit(ui, &self.player, &enemy) {
                self.hits += 1;
                println!("player hit enemy");
            }

            if self.hit(ui, &enemy, &self.player) {
                println!("enemy hit player");
            }
        }
    }

    fn hit(&self, ui: &mut Ui, attacker: &Entity, target: &Entity) -> bool {
        let ct = CollisionTest {
                animation_player: &self.animation_player,
                attacker: attacker.state.animation_id(),
                target: target.state.animation_id(),
                target_pos: target.pos,
                attack_pos: attacker.pos
        };

        ui.drawer2D.z = 2.0;
        let res = collide_draw(ui, &ct, self.show_col_boxes);
        ui.drawer2D.z = 0.0;

        res

    }
}



struct CollisionTest<'a> {
    animation_player: &'a SheetAnimationPlayer<'a>,
    attacker: AnimationId,
    target: AnimationId,
    target_pos: V2,
    attack_pos: V2
}

fn collide_draw(ui: &mut Ui, ct: &CollisionTest, draw: bool) -> bool {

    let mut res = false;
    if let Some((target, target_scale, target_flip_y)) = ct.animation_player.get_polygon(ct.target, "body") {

        let mut target_transform = PolygonTransform::default();
        target_transform.scale = target_scale;
        target_transform.translation = ct.target_pos;
        target_transform.flip_y = target_flip_y;

        if draw {
            ui.view_polygon(&target.polygon, &target_transform);
        }

        if let Some((attack, attack_scale, attack_flip_y)) = ct.animation_player.get_polygon(ct.attacker, "attack") {

            let frame = ct.animation_player.frame(ct.attacker);
            if let Some(f) = frame {
                if f == 3 {
                    let dbug = 2;
                }
            }

            let mut attack_transform = PolygonTransform::default();
            attack_transform.scale = attack_scale;
            attack_transform.translation = ct.attack_pos;
            attack_transform.flip_y = attack_flip_y;

            res = attack.collide_draw(&mut ui.drawer2D, &attack_transform.mat3(), target, &target_transform.mat3());

            if let Some(f) = frame {
                if !res && f == 3 {
                    let f2 = ct.animation_player.frame(ct.target);
                    let dbug = 2;
                    println!("{:?}", f2);
                }
            }

            if draw {
                ui.view_polygon(&attack.polygon, &attack_transform);
            }
        }
    }

    res
}

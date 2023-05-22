use super::*;
use crate::damage_text::TextAnimations;
use rand::prelude::*;


pub type V2 = na::Vector2::<f32>;

pub struct State {
    mouse_down: bool,
    weapon: Weapon,
    mouse_pos: V2,
    arrow: Option<Arrow>,
    arrow_speed: f32,
    scale: f32,
    target: Option<Target>,

    text_anim: TextAnimations
}


#[derive(Debug, Clone, Copy)]
struct Arrow {
    center: V2,
    dir: V2,
    life: f32,
    size: V2
}

#[derive(Debug, Clone, Copy)]
struct Target {
    center: V2,
    dir: V2,
    life: f32,
    size: V2
}

impl Target {

    pub fn top_center(&self, scale: f32) -> V2 {
        self.center - V2::new(0.0, scale * self.size.y / 2.0 )
    }
}


struct Weapon {
    center: V2,
    dir: V2,
    size: V2
}


impl State {
    pub fn new() -> Self {
        Self {
            mouse_down: false,
            mouse_pos: V2::new(0.0, 0.0),
            weapon: Weapon {
                center: V2::new(400.0, 400.0),
                dir: V2::new(0.0, -1.0),
                size: V2::new(32.0, 32.0)
            },
            arrow: None,
            target: None,
            arrow_speed: 400.0,
            scale: 2.0,
            text_anim: TextAnimations::new()
        }
    }
}


pub fn shoot(ui: &mut Ui, assets: &Assets, state: &mut State, dt: f32) {
    handle_inputs(ui, state);

    // maybe get from assets, or pr item??
    let base_size = V2::new(32.0, 32.0);



    ui.label("Arrow_Speed");
    ui.slider(&mut state.arrow_speed, 1.0, 1000.0);

    ui.label("Scale");
    ui.slider(&mut state.scale, 1.0, 32.0);
    state.weapon.size = base_size * state.scale;


    state.text_anim.update(dt);
    state.weapon.dir = (state.mouse_pos - state.weapon.center).normalize();
    handle_arrow(ui, assets, state, dt);
    handle_target(ui, assets, state, dt);



    // Draw non ui stuff

    draw(ui, assets.weapon.texture_id, state.weapon.center, state.weapon.dir, state.weapon.size);
    state.text_anim.draw(&mut ui.drawer2D);

}

fn handle_target(ui: &mut Ui, assets: &Assets, state: &mut State, dt: f32) {
    let base_size = V2::new(32.0, 32.0);

    if let Some(ref mut target) = &mut state.target {
        draw(ui, assets.target.texture_id, target.center, target.dir, target.size * state.scale);

        // handle collision with arrow

        if let Some(ref mut arrow) = &mut state.arrow {
            // if collision remove both arrow and target

            let col = (arrow.center - target.center).magnitude() < 50.0;

            if col {
                let dmg = 3.0;
                // TODO: animate damage going up
                target.life -= dmg;
                state.arrow = None;
                state.text_anim.text(format!("{dmg}"), target.top_center(state.scale));
            }

            if target.life < 0.0 {
                state.target = None;
            }
        }
    } else {

        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>() * ui.drawer2D.viewport.w as f32;
        let y = rng.gen::<f32>() * ui.drawer2D.viewport.h as f32;
        let mut center = V2::new(x, y);

        if (center - state.weapon.center).magnitude() < 200.0 {
            center.x = 0.0;
        }

        state.target = Some(Target {
            center,
            dir : V2::new(0.0, 1.0),
            life: 16.0,
            size: base_size
        });

    }
}

fn handle_arrow(ui: &mut Ui, assets: &Assets, state: &mut State, dt: f32) {

    let base_size = V2::new(32.0, 32.0);

    if let Some(ref mut arrow) = &mut state.arrow {
        arrow.life -= dt;
        arrow.center += arrow.dir * dt * state.arrow_speed;

        draw(ui, assets.arrow.texture_id, arrow.center, arrow.dir, arrow.size * state.scale);

        if arrow.life < 0.0 {
            state.arrow = None;
        }

    } else if state.mouse_down {

        state.arrow = Some(Arrow {
            center: state.weapon.center,
            dir : state.weapon.dir,
            life: 4.0,
            size: base_size
        });
    }
}


fn draw(ui: &mut Ui, id: TextureId, center: V2, dir: V2, size: V2) {
    let angle = dir.x.atan2(dir.y);
    ui.drawer2D.render_img_rot(id,
                               center.x as i32 - (size.x / 2.0) as i32,
                               center.y as i32  - (size.y / 2.0) as i32,
                               angle,
                               size);

}


fn handle_inputs(ui: &Ui, state: &mut State) {

    use event::Event::*;

    for e in ui.get_frame_inputs() {
        match e {
            MouseButtonDown {x, y, ..} => {
                state.mouse_down = true;
            },
            MouseButtonUp {x, y, ..} => {
                state.mouse_down = false;
            },
            MouseMotion { x, y, ..} => {
                state.mouse_pos = V2::new(*x as f32, *y as f32);
            },

            _ => {}
        }
    }

}

use gl_lib::{gl, helpers, na};
use gl_lib_proc::sheet_assets;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use gl_lib::general_animation::{Animation, Animatable, Frame};
use gl_lib::animations::sheet_animation::{Start, SheetAnimation, Sprite, SheetAnimationPlayer, AnimationId};
use gl_lib::typedef::*;
use gl_lib::collision2d::polygon::{PolygonTransform, ComplexPolygon};
use gl_lib::math::AsV2;

mod inputs;
// generate assets struct
sheet_assets!{Assets "examples/2d_animation_player/assets/"}



fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let assets = Assets::load_all(&gl, "examples/2d_animation_player/assets/");

    let mut animation_player = SheetAnimationPlayer::new();


    let mut pos2 = V2i::new(500, 600);

    let mut playing = true;

    let mut inputs = inputs::Inputs::default();

    let scale = 4.0;
    let mut flip_y = false;
    let mut player = Entity {
        state: EntityState::Idle(animation_player.start(Start {sheet: &assets.idle, scale, repeat: true, flip_y})),
        attack_counter: 0,
        pos: V2::new(400.0, 600.0),
        vel: V2::identity(),
    };

    let mut anim_id2 = animation_player.start(Start {sheet: &assets.idle, scale, repeat: true, flip_y});

    let mut time_scale = 1.0;
    let mut poly_name = "body".to_string();
    let mut hits = 0;

    let mut roll_speed = 150.0;


    let mut show_col_boxes = false;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        ui.consume_events(&mut event_pump);
        inputs::handle_inputs(&mut ui, &mut inputs);
        let dt = ui.dt() * time_scale;;

        match player.state {

            EntityState::Idle(id) => {
                player.vel.x = 0.0;
                if inputs.left {
                    player.vel.x = -100.0;
                    flip_y = true;
                }

                if inputs.right {
                    player.vel.x = 100.0;
                    flip_y = false;
                }

                if inputs.mouse() {
                    let attack =  &assets.attack_1;
                    player.attack_counter = (player.attack_counter + 1 ) % 2;
                    player.vel.x = 0.0;
                    animation_player.remove(id);
                    let anim_id = animation_player.start(Start {sheet: &attack, scale, repeat: false, flip_y});
                    player.state = EntityState::Attack(anim_id);
                }

                if inputs.space {
                    player.vel.x = roll_speed;
                    player.vel.x *= if flip_y { -1.0 } else { 1.0 };
                    animation_player.remove(id);
                    let anim_id = animation_player.start(Start {sheet: &assets.roll, scale, repeat: false, flip_y});
                    player.state = EntityState::Roll(anim_id);
                }
            },

            // when roll or attacking, inputs does nothing
            _ => {}
        }

        // update pos by vel
        player.pos += player.vel * dt;



        // update flip  -- maybe do in normal match statement
        match player.state {
            EntityState::Idle(id) => {
                animation_player.flip_y(id, flip_y);
            },
            // cannot rotate mid attack/roll
            _ => {}

        }

        if ui.button("Play/Pause") {
            playing = !playing;
        }

        if ui.button("Flip y") {
            flip_y = !flip_y;
        }

        ui.label("Time scale");
        ui.slider(&mut time_scale, 0.1, 3.1);
        ui.small_text(&format!("{:?}", hits));

        ui.label("Show collision boxes");
        ui.checkbox(&mut show_col_boxes);


        ui.newline();
        ui.label("Roll speed");
        ui.slider(&mut roll_speed,  0.0, 300.0);
        ui.small_text(&format!("{:?}", roll_speed));




        ui.drawer2D.z = 1.0;
        // drag animation to be where we want
        //ui.drag_point(&mut pos, 10.0);

        ui.drag_point(&mut pos2, 10.0);
        ui.drawer2D.z = 0.0;

        // update animations

        if playing {
            animation_player.update(dt);
            match player.state {
                EntityState::Idle(_) => {},
                EntityState::Recover(id) => {
                    if animation_player.expired(id) {
                        // TODO: Next state, could be run or attack, and not idle
                        let anim_id = animation_player.start(Start {sheet: &assets.idle, scale, repeat: true, flip_y});
                        player.state = EntityState::Idle(anim_id);

                        // clears input buffer for mouse, if any
                        inputs.mouse();
                    }
                },
                EntityState::Attack(id) => {

                    if animation_player.expired(id) {

                        if player.attack_counter > 0 && inputs.mouse() {
                            player.attack_counter = (player.attack_counter + 1) % 2;
                            let anim_id = animation_player.start(Start {sheet: &assets.attack_2, scale, repeat: false, flip_y});
                            player.state = EntityState::Attack(anim_id);
                        } else {
                            let sheet = if player.attack_counter == 1 { &assets.attack_1_recover} else {&assets.attack_2_recover};
                            let anim_id = animation_player.start(Start {sheet, scale, repeat: false, flip_y});
                            player.attack_counter = 0;
                            player.state = EntityState::Recover(anim_id);
                        }
                    }
                },
                EntityState::Roll(id) => {
                    if animation_player.expired(id) {
                        let anim_id = animation_player.start(Start {sheet: &assets.idle, scale, repeat: true, flip_y});
                        player.state = EntityState::Idle(anim_id);
                    }
                }
            }
        }


        // draw animation frame at locations
        animation_player.draw(&mut ui.drawer2D, player.pos, player.state.animation_id());
        animation_player.draw(&mut ui.drawer2D, pos2, anim_id2);


        // draw polygons on top
        ui.drawer2D.z = 1.0;

        let ct = CollisionTest {
            animation_player: &animation_player,
            attacker: player.state.animation_id(),
            target: anim_id2,
            target_pos: pos2.v2(),
            attack_pos: player.pos
        };


        if collide_draw(&mut ui, &ct, show_col_boxes) {
            hits += 1;
        }

        // reset
        ui.drawer2D.z = 0.0;


        window.gl_swap_window();
    }
}

struct Scene {
    pub player: Entity,
}

struct Entity {
    pub state: EntityState,
    pub attack_counter: usize,
    pub pos: V2,
    pub vel: V2
}


enum EntityState {
    Idle(AnimationId),
    Attack(AnimationId),
    Roll(AnimationId),
    Recover(AnimationId),
}

impl EntityState {
    fn animation_id(&self) -> AnimationId {
        match self {
            Self::Idle(id) => *id,
            Self::Recover(id) => *id,
            Self::Attack(id)=> *id,
            Self::Roll(id)=> *id,
        }
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

        if draw {
            ui.view_polygon(&attack.polygon, &target_transform);
        }

        let mut target_transform = PolygonTransform::default();
        target_transform.scale = target_scale;
        target_transform.translation = ct.target_pos;
        target_transform.flip_y = target_flip_y;

        if let Some((attack, attack_scale, attack_flip_y)) = ct.animation_player.get_polygon(ct.attacker, "attack") {

            let mut attack_transform = PolygonTransform::default();
            attack_transform.scale = attack_scale;
            attack_transform.translation = ct.attack_pos;
            attack_transform.flip_y = attack_flip_y;

            res = attack.collide_draw(&mut ui.drawer2D, &attack_transform.mat3(), target, &target_transform.mat3());

            if draw {
                ui.view_polygon(&attack.polygon, &attack_transform);
            }

        }
    }

    res
}




/*

struct EntityCollidable<'a> {
    entity_id: usize,
    team_id: usize,
    collision_polygon : ComplexPolygon<'a>,
    transform: na::Matrix3::<f32> // homogeneous 3d matrixt for transforming V2 in 2d
}

/// Find weapon/spell collision with hittable part of target
fn find_collision(attacks: &[EntityCollidable], targets: &[EntityCollidable]) {

    for attack in attacks {
        for target in targets {
            if attack.team_id != target.team_id {
                if collision(attack.collision_polygon, attack.transform,
                             target.collision_polygon, target.transform) {

                    //add collision to output
                }
            }
        }
    }
}

 */
/*
/// Method 1
/// store polygons on animaiton_sheet, each frame has a hashmap<string, polygon>, get_polygon takes animation_id, and a polygon_name,
///returns Option<&Polygon>
fn get_attack_polygon(animation_id: usize, player: &SheetAnimationPlayer) {
    // animaiton_sheet_player know which frame and wich animation
    let attack_polygon = player.get_polygon(animation_id, "attack");

    let body_polygon = player.get_polygon(animation_id, "body");

}

*/

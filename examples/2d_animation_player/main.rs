use gl_lib::{gl, helpers, na};
use gl_lib_proc::sheet_assets;
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::imode_gui::Pos;
use gl_lib::general_animation::{Animation, Animatable, Frame};
use gl_lib::animations::sheet_animation::{SheetAnimation, Sprite, SheetAnimationPlayer, AnimationId};
use gl_lib::typedef::*;
use gl_lib::collision2d::polygon::{PolygonTransform, ComplexPolygon};
use gl_lib::math::AsV2;
use sdl2::event;


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

    let mut player = SheetAnimationPlayer::new();

    let mut anim_id2 = player.start(&assets.idle, 4.0, true);

    let mut pos = V2::new(400.0, 600.0);

    let mut pos2 = V2i::new(500, 600);

    let mut playing = true;

    let mut inputs = Inputs::default();

    let mut player_state = PlayerState::Idle(player.start(&assets.idle, 4.0, true));

    let mut time_scale = 1.0;
    let mut poly_name = "body".to_string();
    let mut hits = 0;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        ui.consume_events(&mut event_pump);
        handle_inputs(&mut ui, &mut inputs);
        let dt = ui.dt() * time_scale;;


            match player_state {
                PlayerState::Idle(_) => {
                    if inputs.left {
                        pos.x -= 100.0 * dt;
                    }

                    if inputs.right {
                        pos.x += 100.0 * dt;
                    }
                },
                PlayerState::Attack(_) => {
                }
            }

        if ui.button("Idle") {
            match player_state {
                PlayerState::Idle(_) => {},
                PlayerState::Attack(id) => {
                    player.remove(id);
                    let anim_id = player.start(&assets.idle, 4.0, true);
                    player_state = PlayerState::Idle(anim_id);

                }
            }
        }


        if ui.button("Attack") || inputs.mouse {
            match player_state {
                PlayerState::Idle(id) => {
                    player.remove(id);
                    let anim_id = player.start(&assets.attack, 4.0, false);
                    player_state = PlayerState::Attack(anim_id);
                },
                PlayerState::Attack(_) => {}
            }
        }

        if ui.button("Play/Pause") {
            playing = !playing;
        }

        ui.label("Time scale");
         ui.slider(&mut time_scale, 0.1, 3.1);

        ui.textbox(&mut poly_name);

        ui.drawer2D.z = 1.0;
        // drag animation to be where we want
        //ui.drag_point(&mut pos, 10.0);

        ui.drag_point(&mut pos2, 10.0);
        ui.drawer2D.z = 0.0;

        // update animations

        if playing {
            player.update(dt);

            match player_state {
                PlayerState::Idle(_) => {},
                PlayerState::Attack(id) => {
                    if player.expired(id) {
                        let anim_id = player.start(&assets.idle, 4.0, true);
                        player_state = PlayerState::Idle(anim_id);

                    }
                }
            }

        }


        ui.small_text(&format!("{:?}", hits));

        // draw animation frame at locations
        player.draw(&mut ui.drawer2D, pos, player_state.animation_id());
        player.draw(&mut ui.drawer2D, pos2, anim_id2);


        // draw polygons on top
        ui.drawer2D.z = 1.0;

        let ct = CollisionTest {
            player: &player,
            attacker: player_state.animation_id(),
            target: anim_id2,
            target_pos: pos2.v2(),
            attack_pos: pos.v2()
        };


        if collide_draw(&mut ui.drawer2D, &ct) {
            hits += 1;
        }

        // reset
        ui.drawer2D.z = 0.0;


        window.gl_swap_window();
    }
}


enum PlayerState {
    Idle(AnimationId),
    Attack(AnimationId)
}

impl PlayerState {
    fn animation_id(&self) -> AnimationId {
        match self {
            Self::Idle(id) => *id,
            Self::Attack(id)=> *id,
        }
    }
}

struct CollisionTest<'a> {
    player: &'a SheetAnimationPlayer<'a>,
    attacker: AnimationId,
    target: AnimationId,
    target_pos: V2,
    attack_pos: V2
}

fn collide_draw(drawer2d: &mut Drawer2D, ct: &CollisionTest) -> bool {

    let mut res = false;
    if let Some((target, target_scale)) = ct.player.get_polygon(ct.target, "body") {

        if let Some((attack, attack_scale)) = ct.player.get_polygon(ct.attacker, "attack") {

            let mut target_transform = PolygonTransform::default();
            target_transform.scale = target_scale;
            target_transform.translation = ct.target_pos;

            let mut attack_transform = PolygonTransform::default();
            attack_transform.scale = attack_scale;
            attack_transform.translation = ct.attack_pos;

            res = attack.collide_draw(drawer2d, &attack_transform.mat3(), target, &target_transform.mat3());
        }
    }

    res
}


#[derive(Default)]
struct Inputs {
    left: bool,
    right: bool,
    mouse: bool
}



fn handle_inputs(ui: &mut Ui, inputs: &mut Inputs) {

    use event::Event::*;
    use sdl2::keyboard::Keycode::*;

    inputs.mouse = false;

    for e in &ui.frame_events {
        match e {
            KeyDown { keycode: Some(D), ..} => {
                inputs.right = true;
            },
            KeyDown { keycode: Some(A), ..} => {
                inputs.left = true;
            },
            KeyUp { keycode: Some(D), ..} => {
                inputs.right = false;
            },
            KeyUp { keycode: Some(A), ..} => {
                inputs.left = false;
            },
            MouseButtonUp {..} => {
                inputs.mouse = true;
            }
            _ => {}
        }
    }
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

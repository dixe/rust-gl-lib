use gl_lib::{gl, deltatime, math::{self, AsV2, AsV2i}, na, helpers, color::Color};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::texture::TextureId;
use serde::{Serialize, Deserialize};
mod sheet_array_animation;
use sheet_array_animation as saa;
use gl_lib::general_animation::{Animation, Animatable, Frame};

pub type V2 = na::Vector2::<f32>;
pub type V2i = na::Vector2::<i32>;

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

    let mut size = na::Vector2::<f32>::new(32.0, 32.0);

    let mut delta_time = deltatime::Deltatime::new();

    let spritesheet = 2;


    let mut assets = load_assets(&mut ui);


    let mut pixel_h = 100;
    let mut elapsed = 0.0;

    let mut cur_anim_option = Some(ActiveAnimation {
        sheet: &assets.idle,
        repeat: true,
        elapsed: 0.0,
        cur_sprite: None,
    });

    let mut pos = V2i::new(300, 400);
    let mut marker =V2i::new(400, 500);
    let mut animation_status = AnimationStatus::Paused;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        delta_time.update();
        let dt = delta_time.time();


        if ui.button("Reload") {
            cur_anim_option = None;
            assets = load_assets(&mut ui);
            cur_anim_option = Some(ActiveAnimation {
                sheet: &assets.idle,
                repeat: true,
                elapsed: 0.0,
                cur_sprite: None,
            });

        }


        if let Some(ref mut cur_anim) = cur_anim_option {
            if ui.button("Idle") {
                cur_anim.sheet = &assets.idle;
            }

            if ui.button("Attack") {
                cur_anim.sheet = &assets.attack;
            }

            ui.newline();
            ui.label("Elapsed");
            ui.slider(&mut elapsed, 0.0, cur_anim.sheet.animation.total_seconds());

            ui.newline();
            ui.label("Pixel_h");
            ui.slider(&mut pixel_h, 10, 150);


            if animation_status != AnimationStatus::Running {
                if ui.button("Play") {
                    cur_anim.elapsed = 0.0;
                    animation_status = AnimationStatus::Running;
                }
            }


            if animation_status == AnimationStatus::Running {
                animation_status = cur_anim.update(dt);

                if ui.button("Pause") {
                    animation_status = AnimationStatus::Paused;
                }
            }



            render_full_sheet(&mut ui, &cur_anim.sheet, pixel_h);

            render_sheet_frame(&mut ui, &cur_anim.sheet, 0, pixel_h);


            ui.drag_point(&mut pos, 10.0);

            cur_anim.draw(&mut ui, pixel_h, pos);
            ui.drag_point(&mut marker, 5.0);
        }
        window.gl_swap_window();
    }
}



pub struct Assets {
    attack: SheetAnimation,
    idle: SheetAnimation,
}


struct ActiveAnimation<'a> {
    sheet: &'a SheetAnimation,
    repeat: bool,
    elapsed: f32,
    cur_sprite: Option<Sprite>
}

impl<'a> ActiveAnimation<'a> {

    fn update(&mut self, dt: f32) -> AnimationStatus {
        self.elapsed += dt;

        self.cur_sprite = self.sheet.animation.at(self.elapsed);

        if self.cur_sprite.is_none() {
            if self.repeat {
                // maybe to % sheet.animation.duration to
                self.elapsed = 0.0;
                self.cur_sprite = self.sheet.animation.at(self.elapsed);
            } else {
                return AnimationStatus::Finished;
            }
        }

        AnimationStatus::Running
    }

    pub fn draw(&self, ui: &mut Ui, pixel_h: i32, pos: V2i) {

        if let Some(s) = self.cur_sprite {

            let scale = 3.0;//pixel_h as f32 / self.sheet.size.y;
            println!("{:?}", scale);
            let sprite = SheetSubSprite {
                sheet_size: self.sheet.size,
                pixel_l: s.x,
                pixel_r: s.x + s.w,
                pixel_b: s.y,
                pixel_t: s.y + s.h,
            };

            let size = V2i::new(s.w, s.h).v2() * scale;
            println!("{:?}", size);


            ui.drawer2D.render_sprite_sheet_frame(self.sheet.texture_id, pos.x, pos.y, size, &sprite);
        }
    }
}

#[derive(PartialEq)]
enum AnimationStatus {
    Paused,
    Running,
    Finished
}

fn render_sheet_frame(ui: &mut Ui, sheet: &SheetAnimation, frame: usize, pixel_h: i32) {

    let f = &sheet.animation.frames[frame].data;
    let scale = pixel_h as f32 / sheet.size.y;
    let sprite = SheetSubSprite {
        sheet_size: sheet.size,
        pixel_l: f.x,
        pixel_r: f.x + f.w,
        pixel_b: f.y,
        pixel_t: f.y + f.h,
    };

    ui.drawer2D.render_sprite_sheet_frame(sheet.texture_id, 300, 300, V2i::new(f.w, f.h).v2() * scale, &sprite);
}

fn render_sheet_elapsed(ui: &mut Ui, sheet: &SheetAnimation, elapsed: f32, pixel_h: i32) {

    if let Some(f) = &sheet.animation.at(elapsed) {
        let scale = pixel_h as f32 / sheet.size.y;
        let sprite = SheetSubSprite {
            sheet_size: sheet.size,
            pixel_l: f.x,
            pixel_r: f.x + f.w,
            pixel_b: f.y,
            pixel_t: f.y + f.h,
        };

        ui.drawer2D.render_sprite_sheet_frame(sheet.texture_id, 300, 500, V2i::new(f.w, f.h).v2() * scale, &sprite);
    }
}


fn render_full_sheet(ui: &mut Ui, sheet: &SheetAnimation, pixel_h: i32) {

    let scale = pixel_h as f32 / sheet.size.y;

    let sprite = SheetSubSprite {
        sheet_size: sheet.size,
        pixel_l: 0,
        pixel_r: sheet.size.x as i32,
        pixel_b: 0,
        pixel_t: sheet.size.y as i32,
    };

    ui.drawer2D.render_sprite_sheet_frame(sheet.texture_id, 300, 400, sheet.size * scale, &sprite);
}



fn load_assets(ui: &mut Ui) -> Assets {
    Assets {
        idle: load_by_name(ui, "Player Sword Idle 48x48"),
        attack: load_by_name(ui, "player sword atk 64x64"),
    }
}



fn load_by_name(ui: &mut Ui, name: &str) -> SheetAnimation {

    let base_path = "examples/2d_animation_player/assets/";

    let anim_json = std::fs::read_to_string(format!("{base_path}{name}.json"));
    let sheet_anim : saa::SheetArrayAnimation = match anim_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            panic!("Error loading json file, creating default animation \n{:?}", err);
        }
    };

    let size = V2::new(sheet_anim.meta.size.w as f32, (sheet_anim.meta.size.h /2) as f32);

    let path = format!("{base_path}{}", &sheet_anim.meta.image);
    let img = image::open(&path).unwrap().into_rgba8();;

    let aspect = img.height() as f32 / img.width() as f32;
    let texture_id = ui.register_image(&img);

    let mut frames = vec![];


    for frame in &sheet_anim.frames {

        frames.push(Frame::<Sprite> {
            data: Sprite
            {
                x: frame.frame.x,
                y: frame.frame.y,
                w: frame.frame.w,
                h: frame.frame.h,
            },
            frame_seconds: frame.duration as f32 / 1000.0

        });
    }

    let anim = SheetAnimation {
        texture_id,
        size: V2::new(sheet_anim.meta.size.w as f32, sheet_anim.meta.size.h as f32),
        animation: Animation { frames },
    };

    anim
}

#[derive(Debug)]
struct SheetAnimation {
    texture_id: TextureId,
    animation: Animation<Sprite>,
    size: V2,
}


#[derive(Debug, Clone, Copy)]
struct Sprite
{
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}


impl Animatable for Sprite {

    // pixel art spritesheet animation don't interpolat between frames
    fn lerp(a: &Self, _b: &Self, _t: f32) -> Self {
        *a
    }

}

use gl_lib::{gl, math::{self, AsV2, AsV2i}, na, helpers, color::Color};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::texture::TextureId;
use deltatime;
use serde::{Serialize, Deserialize};

enum Mode {
    Edit(usize),
    Play(f32)
}

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

    let mut assets = load_assets(&mut ui);

    let mut delta_time = deltatime::Deltatime::new();

    let mut player = Player {
        pos: V2::new(400.0, 300.0)
    };

    let player_color = Color::Rgb(230,190,30);

    let anim_path = "examples/top_rpg/assets/weapon_anim_1.json";
    let mut animation = load(anim_path);

    let mut mode = Mode::Edit(0);

    let mut cur_transform = animation.frames[1].data;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        delta_time.update();
        let dt = delta_time.time();


        if ui.button("Save") {
            save(&animation, anim_path);
        }

        if ui.button("Load") {
            animation = load(anim_path);
        }

        if ui.button("Play") {
            mode = Mode::Play(0.0);
        }


        ui.drawer2D.rounded_rect_color(player.pos.x, player.pos.y, 30, 30, player_color);

        match mode {
            Mode::Play(elapsed) => {
                let new_elapsed = elapsed + dt;
                mode = Mode::Play(new_elapsed);

                if let Some(transform) = animation.at(elapsed) {
                    cur_transform = transform;
                } else {
                    mode = Mode::Edit(0);
                }

                assets.sword.draw(&mut ui.drawer2D, &player.pos, &cur_transform);
            },
            Mode::Edit(idx) => {

                ui.newline();
                if ui.button("New Frame") {
                    let mut frame : Frame::<VertexData> = Default::default();
                    frame.data.scale = 1.0;
                    animation.frames.push(frame);
                }
                for i in 0..animation.frames.len() {
                    edit_frame(&mut ui, &mut animation.frames[i], player.pos, &assets.sword, i);
                    if ui.button(&format!("{i}")) {
                        mode = Mode::Edit(i);
                    }
                }

                cur_transform = animation.frames[idx].data;

            }
        }

        window.gl_swap_window();
    }
}


fn edit_frame(ui: &mut Ui, frame: &mut Frame<VertexData>, offset: V2, sprite: &Sprite, frame_idx: usize) {

    sprite.draw(&mut ui.drawer2D, &offset, &frame.data);

    ui.window_begin(&format!("Frame {frame_idx}"));

    ui.label("Frame_time");
    ui.combo_box(&mut frame.frame_seconds, 0.01, 2.0);

    ui.newline();
    edit_vertex_data(ui, &mut frame.data, offset, sprite, frame_idx);

    ui.window_end(&format!("Frame {frame_idx}"));


}


fn edit_vertex_data(ui: &mut Ui, data: &mut VertexData, offset: V2, sprite: &Sprite, frame_idx: usize) {

    let center = V2::new(sprite.w as f32 / 2.0, sprite.h as f32 / 2.0) * data.scale;

    let absolute = data.translation + offset + center;
    let mut p = absolute.v2i();

    ui.drag_point(&mut p, 5.0);
    data.translation = p.v2() - offset - center;

    ui.label("Rotation");
    ui.newline();
    ui.slider(&mut data.rotation, -std::f32::consts::PI, std::f32::consts::PI);


    ui.newline();

    ui.label("Scale");
    ui.combo_box(&mut data.scale, -3.0, 3.0);
    ui.newline();
    ui.slider(&mut data.scale, -3.0, 3.0);

}

fn save(animation: &Animation<VertexData>, path: &str) {
    match serde_json::to_string(animation) {
        Ok(json) => {
            std::fs::write(path, json);
        },
        Err(err) => {
            println!("Fail to save\n{:?}", err);
        }
    }
}

fn load(path: &str) -> Animation<VertexData> {

    let anim_json = std::fs::read_to_string(path);
    match anim_json {
        Ok(json) => {
            serde_json::from_str(&json).unwrap()
        },
        Err(err) => {
            println!("Error loading json file, creating default animation \n{:?}", err);
            let mut frames : Vec::<Frame<VertexData>> = vec![Frame::default(), Frame::default()];
            frames[0].data.scale = 1.0;
            frames[0].frame_seconds = 0.3;

            frames[1].data.translation = V2::new(10.0, 20.0);
            frames[1].data.scale = 1.2;
            frames[1].data.rotation = -1.5;
            frames[1].frame_seconds = 0.2;

            Animation {
                frames: frames,
            }
        }
    }
}

// TODO serde serialize and deserialize
#[derive(Default, Debug, Serialize, Deserialize)]
struct Animation<T: Animatable> {
    frames: Vec::<Frame<T>>,
}

impl<T: Animatable> Animation<T> {

    pub fn at(&self, elapsed: f32) -> Option<T> {
        let mut skipped = 0.0;
        for i in 0..self.frames.len() {
            skipped += self.frames[i].frame_seconds;
            if elapsed < skipped {

                let f1 = &self.frames[i];

                let f2 = if i == self.frames.len() - 1 {
                    &self.frames[0]
                } else {
                    &self.frames[i + 1]
                };

                // how far into the frame are we? Between 0 and 1
                let start = skipped - f1.frame_seconds;
                let end = skipped;

                let t = math::clamp01(elapsed,start, end);

                return Some(T::lerp(&f1.data, &f2.data, t));

            }
        }
        return None;
    }
}




#[derive(Default, Debug, Serialize, Deserialize)]
struct Frame<T: Animatable + Copy> {
    data:T,
    frame_seconds: f32,
}

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone)]
struct VertexData {
    rotation: f32,
    translation: V2,
    scale: f32,
}

impl Animatable for VertexData {
    // linear interpolate
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            rotation: math::lerp(a.rotation, b.rotation, t),
            translation: a.translation.lerp(&b.translation, t),
            scale: math::lerp(a.scale, b.scale, t)
        }
    }
}

trait Animatable : Copy {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}




struct Player {
    pos: V2,
}

pub struct Sprite {
    texture_id: TextureId,
    w: i32,
    h: i32,
}

impl Sprite {
    fn draw(&self, drawer2D: &mut Drawer2D, pos: &V2, transform: &VertexData) {
        let p = (pos + transform.translation).v2i();
        drawer2D.render_img_rot(self.texture_id,
                                p.x as i32,
                                p.y as i32,
                                transform.rotation,
                                V2::new(64.0, 64.0) * transform.scale);
    }
}

pub struct Assets {
    sword: Sprite
}

impl Assets {
    pub fn all(&self) -> Vec::<&Sprite> {
        vec![&self.sword]
    }
}


fn load_assets(ui: &mut Ui) -> Assets {
    Assets {
        sword: load_by_name(ui, "Sword"),
    }
}

fn load_by_name(ui: &mut Ui, name: &str) -> Sprite {

    let img = image::open(format!("examples/top_rpg/assets/{name}.png")).unwrap().into_rgba8();
    let texture_id = ui.register_image_nearest(&img);

    Sprite {
        texture_id,
        w: img.width() as i32,
        h: img.height() as i32,
    }
}

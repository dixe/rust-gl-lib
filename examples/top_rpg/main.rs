use gl_lib::{gl, na, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;
use gl_lib::text_rendering::font::{Font, MsdfFont, FntFont};
use gl_lib::shader::BaseShader;
use image::{imageops, RgbaImage};
use sdl2::event;
use gl_lib::texture::TextureId;
use deltatime;
mod shoot;

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
    let mut state = State::AssetViewer;
    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ui.consume_events(&mut event_pump);
        delta_time.update();
        let dt = delta_time.time();

        if ui.button("Reload") {
            assets = load_assets(&mut ui);
        }


        if ui.button("Viewer") {
            state = State::AssetViewer;
        }
        if ui.button("Shoot") {
            state = State::Shoot(shoot::State::new());
        }

        match state {
            State::AssetViewer => {

                ui.slider(&mut size.x, 20.0, 400.0);
                size.y = size.x;

                ui.newline();

                viewer(&mut ui, &assets, size);
            },
            State::Shoot(ref mut state) => {
                shoot::shoot(&mut ui, &assets, state, dt);
            }
        }

        window.gl_swap_window();
    }
}


fn viewer(ui: &mut Ui, assets: &Assets, size: na::Vector2::<f32>) {
    for s in assets.all() {
        ui.image(s.texture_id, size);
    }
}



pub enum State {
    AssetViewer,
    Shoot(shoot::State)
}


pub struct Sprite {
    texture_id: TextureId,
    w: i32,
    h: i32
}

pub struct Assets {
    arrow: Sprite,
    target: Sprite,
    weapon: Sprite
}

impl Assets {
    pub fn all(&self) -> Vec::<&Sprite> {
        vec![&self.arrow, &self.target, &self.weapon]
    }
}


fn load_assets(ui: &mut Ui) -> Assets {
    Assets {
        arrow: load_by_name(ui, "arrow"),
        target: load_by_name(ui, "target"),
        weapon: load_by_name(ui, "weapon"),
    }
}

fn load_by_name(ui: &mut Ui, name: &str) -> Sprite {

    let img = image::open(format!("examples/top_rpg/assets/{name}.png")).unwrap().into_rgba8();
    let texture_id = ui.register_image_nearest(&img);

    Sprite {
        texture_id,
        w: img.width() as i32,
        h: img.height() as i32
    }
}

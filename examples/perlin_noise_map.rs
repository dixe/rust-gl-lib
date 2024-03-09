use gl_lib::{helpers};
use gl_lib::imode_gui::ui::*;
use gl_lib::math::numeric::Numeric;
use gl_lib::imode_gui::style::Style;
use noise::{NoiseFn, Perlin, Seedable};
use gl_lib::{gl, texture, shader, typedef::*};
use gl_lib::color::Color;
use rand::Rng;
use std::time::{Duration, SystemTime};
use rayon::prelude::*;


fn main() -> Result<(), failure::Error> {
    let mut sdl_setup = helpers::setup_sdl()?;
    let gl = sdl_setup.gl.clone();
    let mut ui = sdl_setup.ui();

    let mut settings = NoiseSettings {
        scale_x: 50.0,
        scale_y: 50.0,
        w: 512,
        h: 512,
        seed: 0,
        add: 1.0,
        div: 2.0,
        div_base: 255.0
    };


    let tex_id = texture::gen_texture_pbo(&gl, settings.w as i32, settings.h as i32);

    let mut noise = perlin_field_par(&settings);
    let mut update_time = update_pixels(&gl, tex_id, noise.w as i32, noise.h as i32, &noise.data);

    let mut color = Color::white();
    let mut use_par = true;

    loop {
        ui.start_frame(&mut sdl_setup.event_pump);


        ui.drawer2D.render_img(tex_id, 400, 50, V2::new(500.0, 500.0));

        let mut update = false;
        update |= ui.button("Noise");

        if ui.button("Texture shader") {
            shader::reload_object_shader("image", &gl, &mut ui.drawer2D.texture_shader.shader)
        }

        ui.newline();
        ui.label("Use par:");
        ui.checkbox(&mut use_par);

        ui.window_begin("Settings");

        ui.label("Scale_x: ");
        ui.newline();
        update |= ui.slider(&mut settings.scale_x, 0.1, 100.0,);
        ui.newline();


        ui.label("Scale_y: ");
        ui.newline();
        update |= ui.slider(&mut settings.scale_y, 0.1, 100.0,);
        ui.newline();

        ui.label("Add: ");
        ui.newline();
        update |= ui.slider(&mut settings.add, 0.0, 1.0);
        ui.newline();


        ui.label("Div: ");
        ui.newline();
        update |= ui.slider(&mut settings.div, 0.1, 2.0);
        ui.newline();

        ui.label("Div_base: ");
        ui.newline();
        update |= ui.slider(&mut settings.div_base, 1.0, 300.0);
        ui.newline();

        ui.label(&format!("Seed: {}", settings.seed));
        ui.newline();

        if ui.button("New Seed") {
            let mut rng = rand::thread_rng();
            settings.seed = rng.gen::<u32>();
            update |= true;
        }

        ui.newline();


        ui.label(&format!("Gen Time: {:?} ms", noise.generate_time.as_millis()));
        ui.newline();

        ui.label(&format!("Update Time: {:?} ms", update_time.as_millis()));
        ui.window_end("Settings");

        if update {

            if use_par {
                noise = perlin_field_par(&settings);
            } else {
                noise = perlin_field(&settings);
            }

            update_time = update_pixels(&gl, tex_id, noise.w as i32, noise.h as i32, &noise.data);

        }

        ui.end_frame() ;
    }
}




fn update_pixels(gl: &gl::Gl, tex_id: texture::TextureId, w: i32, h: i32, image_data: &[V4u8]) -> Duration {

    let now = SystemTime::now();
    unsafe {
        gl.BindTexture(gl::TEXTURE_2D, tex_id);
        gl.TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, w, h, gl::RGBA, gl::UNSIGNED_BYTE, image_data.as_ptr() as *const gl::types::GLvoid);
        gl.BindTexture(gl::TEXTURE_2D, 0);
    }

    now.elapsed().unwrap()
}

struct Noise {
    data: Vec::<V4u8>,
    w: usize,
    h: usize,
    max: f64,
    min: f64,
    generate_time: Duration
}

struct NoiseSettings {
    w: usize,
    h: usize,
    seed: u32,
    scale_x: f64,
    scale_y: f64,
    add: f64,
    div: f64,
    div_base: f64
}


fn perlin_field_par(settings: &NoiseSettings) -> Noise {

    let timer = SystemTime::now();

    let mut perlin = Perlin::new(settings.seed);

    let mut res = vec![V4u8::new(0,0,0,0); settings.w * settings.h];

    let scale_x = 1.0 / (settings.w as f64) * settings.scale_x;
    let scale_y = 1.0 / (settings.h as f64) * settings.scale_y;

    res = res.par_iter().enumerate().map(|(idx, pixel)| {

        let (x, y) = from_index(idx, settings.w);
        let noise = perlin.get([x as f64 * scale_x, y as f64 * scale_y]);
        let u8_noise = ((noise + settings.add) * settings.div_base / settings.div) as u8;
        V4u8::new(u8_noise, u8_noise, u8_noise, 255)
    }).collect();

    Noise {
        data: res,
        w: settings.w,
        h: settings.h,
        min: -1.0,
        max: 1.0,
        generate_time: timer.elapsed().unwrap()
    }
}

fn perlin_field(settings: &NoiseSettings) -> Noise {

    let timer = SystemTime::now();

    let mut perlin = Perlin::new(settings.seed);

    let mut res = vec![];

    let mut min = 0.0;
    let mut max = 0.0;

    let scale_x = 1.0 / (settings.w as f64) * settings.scale_x;
    let scale_y = 1.0 / (settings.h as f64) * settings.scale_y;
    for i in 0..settings.h {
        for j in 0..settings.w {
            let noise = perlin.get([i as f64 * scale_x, j as f64 * scale_y]);
            min = noise.min(min);
            max = noise.max(max);

            // noise is in range [-1.0; 1.0]
            // map to 0..255


            let u8_noise = ((noise + settings.add) * settings.div_base / settings.div) as u8;
            res.push(V4u8::new(u8_noise, u8_noise, u8_noise, 255));
        }
    }

    Noise {
        data: res,
        w: settings.w,
        h: settings.h,
        min,
        max,
        generate_time: timer.elapsed().unwrap()
    }
}


fn slider_for<T>(ui: &mut Ui, txt: &str, t: T, min: T, max: T) -> T where T : Numeric + std::ops::SubAssign<i32> + std::ops::AddAssign<i32>{
    let mut val = t;
    ui.label(txt);
    ui.slider(&mut val, min, max);
    if ui.button("+") {
        val += 1;
    }
    if ui.button("-") {
        val -= 1;
    }
    val
}


fn save_noise_to_image(noise: & Noise) -> Result<(), image::ImageError> {

    let new_scale = 256.0 / 2.0;

    let mut imgbuf = image::ImageBuffer::new(noise.w as u32, noise.h as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let index = to_index(x,y, noise.w as u32) as usize;
        // + scale to make min 0
        // Now max value is scale * 2


        let p = noise.data[index];

        *pixel = image::Rgb([p.x, p.y, p.z]);
    }

    imgbuf.save("noise_image.png").unwrap();

    Ok(())
}

    fn from_index(idx: usize, w: usize) -> (usize, usize) {
        let x = idx % w;
        let y = idx / w;
        (x,y)
    }

fn to_index(i: u32, j: u32, w: u32) -> u32 {
    i * w + j
}

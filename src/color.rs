use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    RgbA(u8, u8, u8, u8),
    RgbAf32(f32, f32, f32, f32),
    Hsv(f32, f32, f32) // hsv with h in [0..360] and v,s in [0..1]
}

impl Color {

    pub fn as_vec4(&self) -> na::Vector4::<f32> {
        match *self {
            Color::Rgb(r,g,b) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0),
            Color::RgbA(r,g,b,a) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0),
            Color::RgbAf32(r,g,b,a) =>na::Vector4::new(r,g,b,a),
            Color::Hsv(h,s,v) => hsv_to_rgba_vec(h, s, v)
        }
    }

    pub fn to_hsv(&self) -> na::Vector3::<f32> {
        match *self {
            Color::Rgb(r,g,b) => rgb_to_hsv_vec(r, g, b),
            Color::RgbA(r,g,b,_) => rgb_to_hsv_vec(r, g, b),
            Color::RgbAf32(r, g, b, _) => rgb_to_hsv_vec((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8),
            Color::Hsv(h, s,v) => na::Vector3::new(h, s, v)
        }
    }
}


fn hsv_to_rgba_vec(h: f32, s: f32, v: f32) -> na::Vector4::<f32> {

    let hp = h/60.0;
    let c = v * s;
    let x = c * (1.0 - f32::abs((hp % 2.0) - 1.0));
    let m = v - c;


    let (r,g,b) = match h {
        a if a < 60.0 => (c, x, 0.0),
        a if a < 120.0 => (x, c, 0.0),
        a if a < 180.0 => (0.0, c, x),
        a if a < 240.0 => (0.0, x, c),
        a if a < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x)
    };

    na::Vector4::new(r + m, g + m, b + m, 1.0)
}

fn rgb_to_hsv_vec(r: u8, g: u8, b: u8) -> na::Vector3::<f32> {

    let rm = r as f32 / 255.0;
    let gm = g as f32 / 255.0;
    let bm = b as f32 / 255.0;

    let c_max = u8::max(r, u8::max(g,b)) as f32 / 255.0;

    let c_min = u8::min(r, u8::min(g,b)) as f32 / 255.0;
    let delta = c_max - c_min;

    let h = if delta == 0.0 { 0.0 } else
    {
        match c_max {
            x if x == rm => 60.0* (((gm - bm) / delta) % 6.0),
            x if x == gm => 60.0* (((bm - rm) / delta) + 2.0),
            _  => 60.0* (((rm - gm) / delta) + 4.0)
        }
    };


    let s = if c_max == 0.0 { 0.0 } else { delta / c_max };

    let v = c_max;

    na::Vector3::new(h, s, v)
}

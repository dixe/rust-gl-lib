use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    RgbA(u8, u8, u8, u8),
    RgbAf32(f32, f32, f32, f32),
    Hsv(f32, f32, f32, f32) // hsv with h in [0..360] and v,s in [0..1]
}

impl Default for Color {

    fn default() -> Self {
        Self::Rgb(0,0,0)
    }
}

impl Color {

    pub fn lerp(from: Color, to: Color, mut t: f32) -> Self {
        // scale t to [0;1]
        t = f32::min(1.0, f32::max(t, 0.0));

        let c1 = from.as_vec4();
        let c2 = to.as_vec4();

        Self::from_vec4((1.0 - t) * c1 + t * c2)
    }



    pub fn from_vec4(c: na::Vector4::<f32>) -> Self {
        Color::RgbAf32(c.x, c.y, c.z, c.w)
    }

    pub fn as_rgba(&self) -> na::Vector4::<u8> {
        match *self {
            Color::Rgb(r,g,b) => na::Vector4::new(r, g, b, 255),
            Color::RgbA(r,g,b,a) => na::Vector4::new(r, g, b, a),
            Color::RgbAf32(r,g,b,a) => na::Vector4::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8),
            Color::Hsv(h,s,v,a) => {
                let rgba_f32 = hsv_to_rgba_vec(h, s, v, a);
                let r = rgba_f32.x;
                let g = rgba_f32.y;
                let b = rgba_f32.z;
                let a = rgba_f32.w;
                na::Vector4::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8)
            }
        }
    }

    pub fn as_vec4(&self) -> na::Vector4::<f32> {
        match *self {
            Color::Rgb(r,g,b) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0),
            Color::RgbA(r,g,b,a) => na::Vector4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0),
            Color::RgbAf32(r,g,b,a) => na::Vector4::new(r,g,b,a),
            Color::Hsv(h,s,v,a) => hsv_to_rgba_vec(h, s, v, a)
        }
    }

    pub fn to_hsv(&self) -> na::Vector4::<f32> {
        match *self {
            Color::Rgb(r,g,b) => rgb_to_hsv_vec(r, g, b, 255),
            Color::RgbA(r,g,b,a) => rgb_to_hsv_vec(r, g, b, a),
            Color::RgbAf32(r, g, b, a) => rgb_to_hsv_vec((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8),
            Color::Hsv(h, s, v, a) => na::Vector4::new(h, s, v, a)
        }
    }

    pub fn alpha(&self) -> f32 {
        match *self {
            Color::Rgb(_, _, _ ) => 1.0,
            Color::RgbA(_,_ , _, a) => a as f32 / 255.0,
            Color::RgbAf32(_, _ , _, a) => a,
            Color::Hsv(_, _, _, a) => a,
        }
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        let r = match *self {
            Color::Rgb(r,g,b) => Color::RgbA(r,g,b,(alpha * 255.0) as u8),
            Color::RgbA(r,g,b,_) => Color::RgbA(r,g,b,(alpha * 255.0) as u8),
            Color::RgbAf32(r, g, b, _) => Color::RgbAf32(r, g, b, alpha),
            Color::Hsv(h, s,v, _) => Color::Hsv(h, s,v, alpha)
        };

        *self = r;
    }


    // Colors
    pub fn black() -> Self {
        Color::Rgb(0,0,0)
    }

    pub fn white() -> Self {
        Color::Rgb(255, 255, 255)
    }

    pub fn red() -> Self {
        Color::Rgb(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::Rgb(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::Rgb(0, 0, 255)
    }
}


fn hsv_to_rgba_vec(h: f32, s: f32, v: f32, alpha: f32) -> na::Vector4::<f32> {

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

    na::Vector4::new(r + m, g + m, b + m, alpha)
}

fn rgb_to_hsv_vec(r: u8, g: u8, b: u8, a: u8) -> na::Vector4::<f32> {

    let rm = r as f32 / 255.0;
    let gm = g as f32 / 255.0;
    let bm = b as f32 / 255.0;

    let c_max = u8::max(r, u8::max(g,b)) as f32 / 255.0;

    let c_min = u8::min(r, u8::min(g,b)) as f32 / 255.0;
    let delta = c_max - c_min;

    let h = if delta == 0.0 { 0.0 } else
    {
        match c_max {
            x if x == rm => 60.0* (((gm - bm) / delta).rem_euclid(6.0)),
            x if x == gm => 60.0* (((bm - rm) / delta) + 2.0),
            _  => 60.0* (((rm - gm) / delta) + 4.0)
        }
    };


    let s = if c_max == 0.0 { 0.0 } else { delta / c_max };

    let v = c_max;

    na::Vector4::new(h, s, v, a as f32 / 255.0)
}

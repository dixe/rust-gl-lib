use image::{self, Rgba};

pub trait PreMulAlpha {
    fn pre_multiply_alpha(&mut self);
}

impl PreMulAlpha for image::RgbaImage {

    fn pre_multiply_alpha(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = self.get_pixel_mut(x, y);

                let a = p.0[3] as f32 / 255.0;

                *p = Rgba([(p.0[0] as f32 * a) as u8,
                           (p.0[1] as f32 * a) as u8,
                           (p.0[2] as f32 * a) as u8,
                           p.0[3]]);

            }
        }
    }
}

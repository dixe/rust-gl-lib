use super::*;
use gl_lib::color::Color;
use crate::shoot::V2;

struct Text {
    text: String,

    start_pos: V2,
    end_pos: V2,

    pos: V2,
    color_start: Color,
    color_end: Color,
    color: Color,

    life: f32,
    total_life: f32
}


pub struct TextAnimations {
    animations: Vec::<Text>,
    delete: Vec::<usize>,

    color_start: Color,
    color_end: Color,

    pixel_size: i32
}

impl TextAnimations {

    pub fn new() -> Self {
        Self {
            animations: vec![],
            delete: vec![],
            color_start: Color::RgbA(247, 241, 52, 255),
            color_end: Color::RgbA(255, 222, 77, 150),
            pixel_size: 40
        }
    }


    pub fn update(&mut self, dt: f32) {
        self.delete.clear();

        let count = self.animations.len();
        for idx in (0..count).rev() {
            let text = &mut self.animations[idx];
            text.life -= dt;

            if text.life >= 0.0 {

                let t = text.life / text.total_life;
                text.pos = text.start_pos * t + (1.0 - t) * text.end_pos;
                text.color = Color::lerp(text.color_start, text.color_end, t);



            } else{
                self.delete.push(idx)
            }
        }

        // idx is decending, so this find, since idx retain place.
        for idx in &self.delete {
            self.animations.swap_remove(*idx);
        }
    }

    pub fn draw(&self, drawer2D: &mut Drawer2D) {
        for text in &self.animations {
            drawer2D.render_text_with_color(&text.text, text.pos.x as i32, text.pos.y as i32, self.pixel_size, text.color);
        }
    }

    pub fn text(&mut self, text: String, start_pos: V2) {

        self.animations.push(Text {
            text,

            start_pos,
            pos: start_pos,

            end_pos: start_pos + V2::new(0.0, -100.0),
            color_start: self.color_start,
            color_end: self.color_end,
            color: self.color_start,

            life: 2.0,
            total_life: 2.0,
        });

    }
}

use super::*;
use crate::imode_gui::style::TextStyles;
use crate::texture::{self, TextureId};
use image::RgbaImage;

impl Ui {

    pub fn image(&mut self, id: TextureId, size: na::Vector2::<f32>){
        let mut rect = Rect { x: 0, y: 0, w: size.x as i32, h: size.y as i32 };

        rect = self.layout_rect(rect);

        self.image_at(id, size, rect.x, rect.y);
    }

    pub fn image_at(&mut self, id: TextureId, size: na::Vector2::<f32>, x: i32, y: i32) {
        self.drawer2D.render_img(id, x, y, size);
    }

    pub fn register_image(&self, img: &RgbaImage) -> TextureId {
        let id = texture::gen_texture_rgba(&self.drawer2D.gl, img);

        id
    }

     pub fn register_image_nearest(&self, img: &RgbaImage) -> TextureId {
        let id = texture::gen_texture_rgba_nearest(&self.drawer2D.gl, img);

        id
    }

}

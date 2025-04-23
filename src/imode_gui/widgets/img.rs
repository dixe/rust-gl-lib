use super::*;
use crate::imode_gui::style::TextStyles;
use crate::texture::{self, TextureId};
use image::RgbaImage;
use crate::typedef::V2;


pub struct ZoomData {
    pub texture_id: TextureId,
    pub size: V2,
    pub zoom: f32,
    pub zoom_point: V2
}

impl Ui {

    pub fn image(&mut self, id: TextureId, size: na::Vector2::<f32>){

        let mut rect = Rect { x: 0, y: 0, w: size.x as i32, h: size.y as i32 };
        rect = self.layout_rect(rect);

        self.image_at(id, size, rect.x, rect.y);
    }

    pub fn image_zoom(&mut self, zoom_data: &mut ZoomData){

        let mut rect = Rect { x: 0, y: 0, w: zoom_data.size.x as i32, h: zoom_data.size.y as i32 };
        rect = self.layout_rect(rect);

        let widget_id = self.next_id();

        if self.mouse_in_rect(&rect) {

            self.set_hot(widget_id);
        }


        self.drawer2D.render_img_zoom(zoom_data.texture_id, rect.x, rect.y, zoom_data.size, zoom_data.zoom, zoom_data.zoom_point);


    }

    pub fn image_at(&mut self, id: TextureId, size: na::Vector2::<f32>, x: i32, y: i32) {
        self.drawer2D.render_img(id, x, y, size);
    }

    pub fn register_image(&self, img: &RgbaImage) -> TextureId {
        texture::gen_texture_rgba(&self.drawer2D.gl, img)
    }

     pub fn register_image_nearest(&self, img: &RgbaImage) -> TextureId {
         texture::gen_texture_rgba_nearest(&self.drawer2D.gl, img)
    }

}

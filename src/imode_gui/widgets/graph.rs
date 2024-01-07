use super::*;
use crate::math::numeric::Numeric;



pub struct GraphInfo {
    pub w: i32,
    pub h: i32,
    pub start: f32,
    pub end: f32
}


impl std::convert::From<&GraphInfo> for Rect {
    fn from(gi: &GraphInfo) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: gi.w,
            h: gi.h
        }
    }
}


impl Ui{

    pub fn graph<F: Fn(f32) -> f32>(&mut self, info: &GraphInfo, f: F) {

        let id = self.next_id();

        let rect = self.layout_rect(info.into());

        let bg_color = Color::Rgb(240, 240, 240);

        let graph_color = Color::Rgb(0, 0, 0);

        self.drawer2D.rect_color(rect.x , rect.y, rect.w, rect.h, bg_color);


        // find step, ie. how much x increases when we step 1 pixel
        let range = info.end - info.start;

        let step = range / info.w as f32;

        let min_y = info.start;
        let max_y = info.end;
        // how many pixels graph is heigh
        let data_range = max_y - min_y;


        let coord_color = Color::Rgb(0, 0, 200);

        // draw a line in y=0
        let mapped_y_0 = map_to_pixel_space(0.0, info.h as f32, data_range, min_y) + rect.y as f32;

        if mapped_y_0 > rect.y as f32 && mapped_y_0 < (rect.y + rect.h) as f32 {
            self.drawer2D.rect_color(rect.x, mapped_y_0 - 1.0, info.w, 2, coord_color);
        }

        // draw a line in y=0

        if info.start <= 0.0 && info.end >= 0.0 {
            let mapped_x = (-info.start) / (info.end - info.start) * info.w as f32 + rect.x as f32;
            self.drawer2D.rect_color(mapped_x - 1.0, rect.y, 2, info.h, coord_color);
        }



        let mut x = info.start;
        let mut i = 0;
        while x < info.end {
            let y = f(x);
            let x1 = rect.x + i;

            let mapped_y = map_to_pixel_space(y, info.h as f32, data_range, min_y) + rect.y as f32;
            self.drawer2D.rect_color(x1 , mapped_y, 1, 1, graph_color);
            i += 1;
            x += step;
        }

    }
}

fn map_to_pixel_space(y: f32, h: f32, data_range: f32, min_y: f32) -> f32 {
    h as f32 -  (y - min_y)/data_range * h as f32
}

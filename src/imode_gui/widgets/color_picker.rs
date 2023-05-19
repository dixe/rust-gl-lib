use super::*;
use crate::objects::color_square::SquareColors;

impl Ui{

    pub fn color_picker(&mut self, color: &mut Color) {

        let id = self.next_id();

        // when active we show the picker
        if self.is_active(id) {

            let picker_w = 200;

            let pixel_size = self.style.text_styles.small.pixel_size;
            let font_name = &self.style.text_styles.small.font_name;

            let text_box = self.drawer2D.text_render_box_with_font_name("R: 255", pixel_size, font_name);

            let text_w = text_box.total_width as i32;

            let mut rect = Rect {
                x: 0,
                y: 0,
                w: picker_w + text_w + self.style.padding.x() + self.style.spacing.x,
                h: 230 + self.style.padding.y() * 2
            };

            rect = self.layout_rect(rect);

            // draw a background for our colorpicker
            self.drawer2D.rounded_rect_color(rect.x, rect.y, rect.w, rect.h, Color::Rgb(162, 179, 171));

            self.set_active_context(id, rect);

            let color_square_rect = self.color_square(color);

            let mut c_vec4 = color.as_vec4();


            self.newline();
            self.color_line(color);


            let x = color_square_rect.x + color_square_rect.w + self.style.spacing.x;
            let y = rect.y;

            let font_name = &self.style.text_styles.small.font_name;
            self.drawer2D.render_text_from_font_name(&format!("R: {}", (c_vec4.x * 255.0) as u8), x, y, pixel_size, font_name);
            self.drawer2D.render_text_from_font_name(&format!("G: {}", (c_vec4.y * 255.0) as u8), x, y + 20, pixel_size, font_name);
            self.drawer2D.render_text_from_font_name(&format!("B: {}", (c_vec4.z * 255.0) as u8), x, y + 40, pixel_size, font_name);
            self.drawer2D.render_text_from_font_name(&format!("A: {}", (c_vec4.w * 255.0) as u8), x, y + 60, pixel_size, font_name);

            self.drawer2D.render_text_from_font_name("A: ", x, y + 60, pixel_size, font_name);
            self.combo_box(&mut c_vec4.w, 0.0, 1.0);

            color.set_alpha(c_vec4.w);

            self.exit_active_context(id);

            let in_rect = self.mouse_in_rect(&rect);
            if self.mouse_down && !in_rect {
                self.set_not_active();
            }

        } else { // else we just show a square of the color

            let mut rect = Rect {
                x: 0,
                y: 0,
                w: 30,
                h: 30
            };

            rect = self.layout_rect(rect);

            if self.mouse_in_rect(&rect) {
                self.set_hot(id);
            }

            if self.is_hot(id) {
                if self.mouse_down {
                    self.set_active(id);
                }
            }
            self.drawer2D.rounded_rect_color(rect.x, rect.y, rect.w, rect.h, *color)
        }

    }

    fn color_line(&mut self, color: &mut Color) {
        // TODO: do this rect work all in color_picker and have these two functions take rects

        let id = self.next_id();


        let mut rect = Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 30
        };

        rect = self.layout_rect(rect);

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }


        let hsv = color.to_hsv();
        let mut h = hsv.x;

        if self.is_active(id) {
            h = (self.mouse_pos.x - rect.x) as f32 / rect.w as f32 * 360.0;

            h = f32::max(0.0, f32::min(360.0, h));

            let hsv = color.to_hsv();

            if self.mouse_up {
                self.set_not_active();
            }
            *color = Color::Hsv(h, hsv.y, hsv.z, hsv.w)
        }


        // Draw H space line
        self.drawer2D.hsv_h_line(rect.x, rect.y, rect.w, rect.h);

        // Draw target
        let x = (rect.w as f32 * h / 360.0) as i32;
        self.drawer2D.rounded_rect_color(rect.x + x, rect.y, 5, rect.h, Color::Rgb(30, 30, 30));
    }

    fn color_square(&mut self, color: &mut Color) -> Rect {

        // figure out button layout

        let id = self.next_id();

        // border box, with space for padding for text content
        let mut rect = Rect {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        };

        rect = self.layout_rect(rect);

        // Calc state with hot and active
        // Hot is hover, active is we pressed down
        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_active(id) {
            if self.is_hot(id) {
                *color = pos_to_color(self.mouse_pos, rect, *color);
            }
            if self.mouse_up {
                if self.is_hot(id) {
                    *color = pos_to_color(self.mouse_pos, rect, *color);
                }
                self.set_not_active();

            }
        }
        else if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        let hsv = color.to_hsv();

        // From hsv we can calc where to put the marker for selected color

        let colors = SquareColors {
            top_left: Color::Hsv(hsv.x, 0.0, 1.0, hsv.w),
            top_right: Color::Hsv(hsv.x, 1.0, 1.0, hsv.w),
            bottom_left: Color::Hsv(hsv.x, 0.0, 0.0, hsv.w),
            bottom_right: Color::Hsv(hsv.x, 1.0, 0.0, hsv.w),
        };

        // update colors for color_square
        self.drawer2D.color_square.sub_data(&self.drawer2D.gl, -0.5, 0.5, 0.5, -0.5, colors);

        // draw color square
        self.drawer2D.color_square(rect.x, rect.y, rect.w, rect.h);

        // hsv.y = s of hsv  and .z is v

        let center_x = rect.x as f32 + hsv.y * (rect.w as f32);
        let center_y = rect.y as f32 + (1.0 - hsv.z) * (rect.h as f32);
        self.drawer2D.circle(center_x as i32, center_y as i32, 6, Color::Rgb(200, 200, 200));

        rect
    }
}


fn pos_to_color(m_pos: Pos, rect: Rect, color: Color) -> Color {

    let hsv = color.to_hsv();
    let h = hsv.x;
    // sdl has y=0 at top, opengl at bottom, so inverse v
    let v = 1.0 - (m_pos.y - rect.y) as f32 / rect.h as f32;
    let s = (m_pos.x - rect.x) as f32 / rect.w as f32;

    Color::Hsv(h, s, v, hsv.w)
}

use super::*;
use crate::imode_gui::style::TextStyles;

impl Ui {

    pub fn body_text(&mut self, text: &str)
    {
        let pixel_size = self.style.text_styles.body.pixel_size;
        self.text(text, pixel_size, |styles| &styles.body.font_name);
    }


    pub fn heading_text(&mut self, text: &str)
    {
        let pixel_size = self.style.text_styles.heading.pixel_size;
        self.text(text, pixel_size, |styles| &styles.body.font_name);
    }

    pub fn small_text(&mut self, text: &str)
    {
        let pixel_size = self.style.text_styles.small.pixel_size;
        self.text(text, pixel_size, |styles| &styles.small.font_name);
    }


    pub fn text(&mut self, text: &str, pixel_size: i32, font_name_fn: fn(&TextStyles) -> &str) {

        let font_name = font_name_fn(&self.style.text_styles);

        let rb = self.drawer2D.text_render_box_with_font_name(text, pixel_size, font_name);

        let mut rect = Rect { x: 0, y: 0, w: rb.total_width as i32, h: rb.total_height as i32 };

        rect = self.layout_rect(rect);

        // Required to call again, with let, so we overwrite the old, and does not cause lifetime issues with
        let font_name = font_name_fn(&self.style.text_styles);
        self.drawer2D.render_text_from_font_name(text, rect.x, rect.y, pixel_size, font_name);
    }

    pub fn textbox(&mut self, data: &mut String) {
        let pixel_size = self.style.text_styles.body.pixel_size;

        let h = pixel_size + 6;

        let mut rect = Rect { x: 0, y: 0, w: pixel_size * 10 as i32, h: h as i32};
        rect = self.layout_rect(rect);

        let id = self.next_id();

        if self.mouse_in_rect(&rect) {
            self.set_hot(id);
        }

        if self.is_hot(id) {
            if self.mouse_down {
                self.set_active(id);
            }
        }

        let mut active = self.is_active(id);

        if active {
            if self.mouse_down {
                if !self.mouse_in_rect(&rect) {
                    self.set_not_active();
                    active = false;
                }
            }
        }



        if active {

            // process text input and add it to the inpit data string

            use event::Event::*;
            use sdl2::keyboard::Keycode::*;
            for event in &self.widget_frame_events {
                match event {
                    KeyDown { keycode: Some(Backspace), ..} => {
                        let len = data.len();
                        if len > 0 {
                            data.remove(len - 1);
                        }
                    },
                    TextEditing { text, ..} => {
                        println!("{:?}",text);
                    },
                    TextInput { text, .. } => {
                        data.push_str(text);
                    },
                    _ => {}
                }
            }
        }


        // get max 17 chars from string
        // TODO: get &Vec::<CharPosInfo> from text_renderer, so we can inspect the size out self

        let start = i32::max(0, (data.len() as i32 - 17)) as usize;
        let txt = &data[start..data.len()];

        let font_name = &self.style.text_styles.body.font_name;

        let color = Color::Rgb(250, 250, 250);
        // background
        self.drawer2D.rect_color(rect.x , rect.y, rect.w, rect.h, color);

        if active {
        // cursor
            let render_box = self.drawer2D.text_render_box_with_font_name(txt, pixel_size, font_name);

            let cursor_color = Color::Rgb(5,5,5);
            self.drawer2D.rect_color(rect.x + 2 + render_box.total_width as i32, rect.y + 2, 5, pixel_size, cursor_color);
        }
        // draw the text input
        self.drawer2D.render_text_from_font_name(txt, rect.x, rect.y, pixel_size, font_name);
    }
}

use gl_lib::sdl_gui::components::base;
use gl_lib::{
    gl,
    na,
    na::Translation3,
    objects::square,
    ScreenBox,
    shader::BaseShader,
    shader::Shader,
    text_rendering::{ text_renderer::{TextRenderer, TextAlignment, TextAlignmentX, TextAlignmentY} },
};

use std::fmt;
use crate::game::*;


#[derive(Debug)]
pub struct GameComponent<Message> {
    pub grid_shader: BaseShader,
    pub hidden_shader: BaseShader,
    pub base: base::ComponentBase,
    columns: usize,
    rows: usize,
    left_clicked_message: fn(Point) -> Message,
    right_clicked_message: fn(Point) -> Message,
    game_info: GameInfo
}


impl<Message> GameComponent<Message> where Message: Clone  {

    pub fn new(gl: &gl::Gl, game_info: GameInfo, left_clicked_message: fn(Point) -> Message, right_clicked_message: fn(Point) -> Message) -> Box<Self> {
        let grid_shader = grid_shader(gl).unwrap();
        let hidden_shader = hidden_tile_shader(gl).unwrap();

        Box::new(Self {
            grid_shader,
            hidden_shader,
            columns: 9,
            rows: 9,
            base: Default::default(),
            left_clicked_message,
            right_clicked_message,
            game_info,
        })
    }


    fn render_grid(&self, gl: &gl::Gl, transform: na::Matrix4::<f32>, render_square: &square::Square) {

        self.grid_shader.set_used();

        self.grid_shader.set_mat4(gl, "transform", transform);

        self.grid_shader.set_f32(gl, "height", self.base.height);

        self.grid_shader.set_f32(gl, "width", self.base.width);

        render_square.render(&gl);
    }


    fn render_hidden(&self, gl: &gl::Gl, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        for (i, tile) in self.game_info.tiles.iter().enumerate() {
            if *tile == Tile::Hidden || *tile == Tile::Flag {

                let p = Point::new(i % 9, i / 9);
                let transform = self.hidden_tile_transform_matrix(p, screen_w, screen_h);
                self.hidden_shader.set_used();

                self.hidden_shader.set_mat4(gl, "transform", transform);

                self.hidden_shader.set_f32(gl, "height", self.base.height / self.rows as f32);

                self.hidden_shader.set_f32(gl, "width", self.base.width / self.columns as f32);

                self.hidden_shader.set_vec3(gl, "u_color", na::Vector3::new(0.9, 0.9, 0.9));

                render_square.render(&gl);
            }
        }
    }

    fn render_suggestion(&self, gl: &gl::Gl, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        for (i, tile) in self.game_info.tiles.iter().enumerate() {
            if *tile == Tile::Suggestion {

                let p = Point::new(i % 9, i / 9);
                let transform = self.hidden_tile_transform_matrix(p, screen_w, screen_h);
                self.hidden_shader.set_used();

                self.hidden_shader.set_mat4(gl, "transform", transform);

                self.hidden_shader.set_f32(gl, "height", self.base.height / self.rows as f32);

                self.hidden_shader.set_f32(gl, "width", self.base.width / self.columns as f32);

                self.hidden_shader.set_vec3(gl, "u_color", na::Vector3::new(0.2, 0.9, 0.2));

                render_square.render(&gl);
            }
        }

    }

    fn render_numbered(&self, gl: &gl::Gl, tr: &mut TextRenderer, screen_w: f32, screen_h: f32) {

        let grid_tile_h = self.base.height / 9.0;
        let grid_tile_w = self.base.width / 9.0;

        let alignment = TextAlignment {x: TextAlignmentX::Center, y: TextAlignmentY::Center };
        for (i, tile) in self.game_info.tiles.iter().enumerate() {

            match tile {
                Tile::Numbered(bombs) => {

                    let tile_x = i % 9;
                    let tile_y = i / 9;
                    let x = grid_tile_w * tile_x as f32 + self.base.x;
                    let y = grid_tile_h * tile_y as f32 + self.base.y;
                    tr.render_text(gl, &format!("{}", bombs), alignment, ScreenBox::new(x, y, grid_tile_w, grid_tile_h, screen_w, screen_h), 1.0);

                },
                _ => {},
            };
        }
    }


    fn render_flagged(&self, gl: &gl::Gl, tr: &mut TextRenderer, screen_w: f32, screen_h: f32) {

        let grid_tile_h = self.base.height / 9.0;
        let grid_tile_w = self.base.width / 9.0;

        let alignment = TextAlignment {x: TextAlignmentX::Center, y: TextAlignmentY::Center };
        for (i, tile) in self.game_info.tiles.iter().enumerate() {

            match tile {
                Tile::Flag => {
                    let tile_x = i % 9;
                    let tile_y = i / 9;
                    let x = grid_tile_w * tile_x as f32 + self.base.x;
                    let y = grid_tile_h * tile_y as f32 + self.base.y;
                    tr.render_text(gl, "F", alignment, ScreenBox::new(x, y, grid_tile_w, grid_tile_h, screen_w, screen_h), 1.0);

                },
                _ => {},
            };
        }
    }


    fn render_bombs(&self, gl: &gl::Gl, tr: &mut TextRenderer, screen_w: f32, screen_h: f32) {
        let grid_tile_h = self.base.height / 9.0;
        let grid_tile_w = self.base.width / 9.0;


        let alignment = TextAlignment {x: TextAlignmentX::Center, y: TextAlignmentY::Center };
        for p in self.game_info.bombs.iter() {
            let x = grid_tile_w * p.x as f32 + self.base.x;
            let y = grid_tile_h * p.y as f32 + self.base.y;
            tr.render_text(gl, &format!("B"), alignment, ScreenBox::new(x, y, grid_tile_w, grid_tile_h, screen_w, screen_h), 1.0);
        }
    }

    pub fn hidden_tile_transform_matrix(&self, point: Point, screen_w: f32, screen_h: f32) -> na::Matrix4::<f32> {

        let sc_top_left = base::ComponentBase::window_to_screen_coords(self.base.x, self.base.y, screen_w, screen_h);

        let screen_x_scale = self.base.width  / screen_w  * 2.0;
        let screen_y_scale = self.base.height / screen_h * 2.0;

        let x_scale = screen_x_scale * (1.0 / self.columns as f32);
        let y_scale = screen_y_scale * (1.0 / self.rows as f32);
        let mut model = na::Matrix4::<f32>::identity();

        // Scale to size
        model[0] = x_scale;
        model[5] = y_scale;

        // move to position

        // p.x = 0 should move - self.rows/2  p.x = rows should move self.rows/2

        // scales p goes from -1 .. 1, not really 1 but (rows-1)/rows,
        let scaled_p_x = point.x as f32 / self.rows as f32 * 2.0 - 1.0; // TODO: when sc_top_left = -1 then 2.0, when 0.0 then 1.0, when 0.8 then 0.2;

        let scaled_p_y = point.y as f32 / self.columns as f32 * (sc_top_left.y + 1.0);

        //panic!();
        let x_move = scaled_p_x + x_scale * 0.5;
        let y_move = sc_top_left.y -y_scale * 0.5 - scaled_p_y;// scaled_p_y;// + scaled_p_y;//- y_scale * 0.5;

        let trans = Translation3::new(x_move, y_move, 0.0);

        model = trans.to_homogeneous() * model;

        model
    }
}


impl<Message> base::ComponentTrait<Message> for GameComponent<Message> where Message: Clone + fmt::Debug {

    fn base(&self) -> &base::ComponentBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut base::ComponentBase {
        &mut self.base
    }

    fn set_base(&mut self, base: base::ComponentBase) {
        self.base = base;
    }


    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        let grid_transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);

        self.render_grid(gl, grid_transform, render_square);

        self.render_hidden(gl, render_square, screen_w, screen_h);

        self.render_numbered(gl, tr, screen_w, screen_h);

        self.render_flagged(gl, tr, screen_w, screen_h);

        self.render_suggestion(gl, render_square, screen_w, screen_h);

        if self.game_info.died {
            self.render_bombs(gl, tr, screen_w, screen_h);
        }
    }

    fn update_content(&mut self, _: String) {

    }

    fn on_event(&self, event: base::ComponentEvent) -> Option<Message> {
        match event {
            base::ComponentEvent::Clicked(click_type, vec2) => {

                let offset = na::Vector2::new(self.base.x as i32, self.base.y as i32);
                let relative = vec2 - offset;

                let x = ((relative.y as f32 / self.base.height ) * self.rows as f32) as usize;
                let y = ((relative.x as f32 / self.base.width ) * self.columns as f32) as usize;

                match click_type {
                    base::ClickType::Left => Some((self.left_clicked_message)(Point::new(x,y))),
                    base::ClickType::Right => Some((self.right_clicked_message)(Point::new(x,y)))
                }
            },
            _ => None
        }
    }
}


/// Creates a shader for rendering a grid on a square (two triangle)
pub fn grid_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = std::include_str!("grid_shader.vert");
    let frag_source = std::include_str!("grid_shader.frag");

    BaseShader::new(gl, vert_source, frag_source)
}


/// Creates a shader for rendering a hidden tile on a square (two triangle)
pub fn hidden_tile_shader(gl: &gl::Gl) -> Result<BaseShader, failure::Error> {

    // default program for square
    let vert_source = std::include_str!("hidden_tile_shader.vert");
    let frag_source = std::include_str!("hidden_tile_shader.frag");

    BaseShader::new(gl, vert_source, frag_source)
}

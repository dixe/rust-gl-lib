use gl_lib::na;
use gl_lib::sdl_gui as gls;
use failure;

use rand::prelude::*;


mod game;
use game::*;


#[derive(Debug, Clone)]
pub enum Message {
    Restart,
    LeftClick(game::Point),
    RightClick(game::Point),
}

fn main() -> Result<(), failure::Error> {
    let width = 600;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("MineSweeper", width, height).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut state = GameLogic::default();

    while !window.should_quit() {
        window.update(&mut state);
    }

    Ok(())
}


#[derive(Debug, Clone)]
struct GameLogic {
    initialized: bool,
    game_info: GameInfo
}

impl GameLogic {
    pub fn uncover_cell(&mut self, i: i32) {

        if self.game_info.is_bomb(self.index_to_point(i as usize)) {
            return;
        }

        // check adjecent
        let mut bombs = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                let index = i + (c  * 9) + r;
                if index >= 0 && index < 81 {
                    if self.game_info.is_bomb(self.index_to_point(index as usize)) {
                        bombs += 1;
                    }
                }
            }
        }

        self.game_info.tiles[i as usize] = match bombs {
            0 => Tile::UnCovered,
            x => Tile::Numbered(x)
        };
    }



    fn uncover_cells(&mut self, index: i32, visited: &mut std::collections::HashSet<i32>) {
        visited.insert(index);
        self.uncover_cell(index);

        if self.game_info.tiles[index as usize] != Tile::UnCovered {
            return;
        }

        for r in -1..=1 {
            for c in -1..=1 {
                let new_index = index + (c  * 9) + r;
                if new_index >= 0 && new_index < 81 && !visited.contains(&new_index) {
                    self.uncover_cells(new_index, visited);
                }
            }
        }

    }

    fn point_to_index(p: Point) -> usize {
        p.x * 9 + p.y
    }

    fn index_to_point(&self, i: usize) -> Point {
        Point::new(i % 9, i / 9)
    }

    fn has_won(&self) -> bool {
        for tile in &self.game_info.tiles {
            if *tile == Tile::Hidden {
                return false;
            }
        }

        true
    }

    fn reset(&mut self) {
        for tile in self.game_info.tiles.iter_mut() {
            *tile = Tile::Hidden;
        }

        self.game_info.died = false;
        self.initialized = false;
    }


    fn initialize(&mut self, pressed_index: usize) {
        self.reset();
        self.initialized = true;


        let mut rng = rand::thread_rng();
        let mut indexes: Vec<usize> = (1..81).collect();
        indexes.shuffle(&mut rng);

        let mut bombs = 0;
        let mut i = 0;
        while bombs < 10 {

            if indexes[i] != pressed_index {
                self.game_info.bombs[bombs] = self.index_to_point(indexes[i]);
                println!("Bomb at {:?}", self.game_info.bombs[bombs]);
                bombs += 1;
            }

            i += 1;
        }
    }
}

impl Default for GameLogic {
    fn default() -> Self {
        Self {
            initialized: false,
            game_info: GameInfo {
                tiles: [Tile::Hidden; 9*9],
                died: false,
                bombs: [na::Vector2::new(0,0); 10],
            }
        }
    }
}




impl gls::State<Message> for GameLogic {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Restart => {
                self.reset();
            },

            Message::LeftClick(p) => {

                if self.has_won() {
                    println!("YOU HAVE WON");
                    return;
                }

                if self.game_info.died {
                    return;
                }
                if !self.initialized {
                    self.initialize(Self::point_to_index(*p));
                }

                if self.game_info.is_bomb(*p) {
                    self.game_info.died = true;
                    return;
                }
                self.uncover_cells(GameLogic::point_to_index(*p) as i32, &mut std::collections::HashSet::new());
            },

            Message::RightClick(p) => {

                let tile = self.game_info.tiles[Self::point_to_index(*p)];
                if tile == Tile::Hidden {
                    self.game_info.tiles[Self::point_to_index(*p)] = Tile::Flag;
                }

                if tile == Tile::Flag {
                    self.game_info.tiles[Self::point_to_index(*p)] = Tile::Hidden;

                }
            },
        }
    }



    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;

        use Length::*;

        let col = Column::new()
            .add(Row::new()
                 .padding(5.0)
                 .width(Fill)
                 .add(Button::new("Time", None)
                      .height(Px(50))
                 )
                 .add(Button::new("Restart", Some(Message::Restart))
                      .height(Px(50))
                      .align_center()
                 )
                 .add(Button::new("Sugget Move", None)
                      .height(Px(50))
                      .align_right()
                 ))
            .add(GameLayout::new(self.game_info.clone(), Message::LeftClick, Message::RightClick)
                 .height(Fill));
        col.into()
    }
}

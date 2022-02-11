use gl_lib::na;


mod component;
pub use self::component::*;

mod layout_element;
pub use self::layout_element::*;

pub type Point = na::Vector2::<usize>;



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Hidden,
    UnCovered,
    Numbered(u8),
    Suggestion,
    Flag
}

#[derive(Debug, Clone, Copy)]
pub struct GameInfo {
    pub tiles: [Tile; 9*9],
    pub died: bool,
    pub bombs: [Point; 10]
}

impl GameInfo {

    pub fn is_bomb(&self, p: Point) -> bool {
        for b in &self.bombs {
            if *b == p {
                return true;
            }
        }
        false
    }

}

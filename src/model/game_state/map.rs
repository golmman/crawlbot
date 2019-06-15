use std::fmt::Debug;
use crate::model::game_state::tile::Tile;

#[derive(Clone)]
pub struct Map {
    pub focus: (i64, i64),
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            focus: (0, 0),
            tiles: vec![Tile::new(); 80 * 70],
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Map {{")?;
        writeln!(f, "    focus: {:?},", self.focus)?;
        write!(f, "}}")?;
        Ok(())
    }
}

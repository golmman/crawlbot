use std::collections::HashMap;
use crate::model::game_state::monster::Monster;
use crate::model::game_state::tile::Tile;
use crate::model::game_state::GameState;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Map {
    pub focus: (i64, i64),
    // TODO: refactor to a simple array with e.g. 256 elements
    pub monsters_visible: HashMap<i64, Monster>,
    pub monsters_assumed: HashMap<i64, Monster>,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            focus: (0, 0),
            monsters_visible: HashMap::new(),
            monsters_assumed: HashMap::new(),
            tiles: vec![Tile::new(); (GameState::MAP_WIDTH * GameState::MAP_HEIGHT) as usize],
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Map {{")?;
        writeln!(f, "    focus: {:?},", self.focus)?;
        writeln!(f, "    monsters_visible: {:?},", self.monsters_visible)?;
        write!(f, "}}")?;
        Ok(())
    }
}

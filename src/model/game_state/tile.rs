use crate::model::cws::cell::CwsCell;

#[derive(Debug, Clone)]
pub struct Tile {
    pub glyph: String,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            glyph: String::from(" "),
        }
    }

    pub fn from_cell(cell: CwsCell) -> Self {
        Self {
            glyph: cell.g.unwrap_or(String::from(" ")),
        }
    }
}

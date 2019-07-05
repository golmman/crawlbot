use crate::model::cws::cell::CwsCell;

#[derive(Debug, Clone)]
pub struct Tile {
    pub color: i64,
    pub glyph: String,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            color: 0,
            glyph: String::from(" "),
        }
    }

    pub fn update(&mut self, cell: &CwsCell) {
        if let Some(color) = &cell.col {
            self.color = *color;
        }

        if let Some(glyph) = &cell.g {
            self.glyph = glyph.clone();
        }
    }
}

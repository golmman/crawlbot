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
}

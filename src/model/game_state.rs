#[derive(Debug, Clone)]
pub struct GameState {
    paused: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            paused: true,
        }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}

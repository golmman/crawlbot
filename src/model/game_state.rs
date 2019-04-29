#[derive(Debug, Clone, Copy)]
pub struct GameState {
    paused: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            paused: true,
        }
    }

    pub fn get_paused(self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}

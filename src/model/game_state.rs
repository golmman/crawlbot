#[derive(Debug, Clone, Copy)]
enum InputMode {
    Wait = 0,
    Game = 1,
    More = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    paused: bool,
    enemy_number_in_sight: u32,
    input_mode: InputMode,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            paused: true,
            enemy_number_in_sight: 0,
            input_mode: InputMode::Wait,
        }
    }

    pub fn is_paused(self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn get_enemy_number_in_sight(self) -> u32 {
        self.enemy_number_in_sight
    }

    pub fn set_enemy_number_in_sight(&mut self, n: u32) {
        self.enemy_number_in_sight = n;
    }

    pub fn dec_enemy_number_in_sight(&mut self) {
        self.enemy_number_in_sight -= 1;
    }

    pub fn inc_enemy_number_in_sight(&mut self) {
        self.enemy_number_in_sight += 1;
    }
}

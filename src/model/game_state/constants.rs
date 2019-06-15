use crate::model::game_state::GameState;

#[allow(dead_code)]
impl GameState {
    pub const MAP_WIDTH: i64 = 80;
    pub const MAP_HEIGHT: i64 = 70;
    pub const MAP_CELLS_COUNT: i64 = Self::MAP_WIDTH * Self::MAP_HEIGHT;
}

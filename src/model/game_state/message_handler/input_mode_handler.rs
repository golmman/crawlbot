use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::input_mode::InputMode;

impl GameState {
    pub fn update_input_mode(&mut self, msg: CwsMsg) {
        if let Some(mode) = msg.mode {
            self.set_input_mode(InputMode::from_i64(mode));
        }
    }
}

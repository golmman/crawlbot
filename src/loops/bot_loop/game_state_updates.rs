use crate::model::cws::msg::CwsMsg;
use crate::loops::bot_loop::BotLoopState;
use crate::model::game_state::input_mode::InputMode;
use crate::{log_crawl, log_debug};

impl BotLoopState {
    pub fn update_input_mode(&mut self, msg: CwsMsg) {
        if let Some(mode) = msg.mode {
            self.game_state.set_input_mode(InputMode::from_i64(mode));
        }
    }

    pub fn update_game_state_with_cells(&mut self, msg: CwsMsg) {
        // if let Some(cells) = message.cells {
        //     self.game_state.clear_monsters_in_sight();
        //     for cell in cells {
        //         if cell.mon.is_some() {
        //             // self.game_state.add_monster_in_sight(mon);
        //             self.game_state.update_cell_cache(cell);
        //         }
        //     }
        // }
        self.game_state.update_map(msg);
    }

    pub fn update_game_state_with_msgs(&mut self, msg: CwsMsg) {
        if let Some(messages) = msg.messages {
            for log in messages {
                if let Some(text) = log.text {
                    self.update_game_state_with_msgs_text(text.as_str());
                }
            }
        }   
    }

    pub fn update_game_state_with_msgs_text(&mut self, message_text: &str) {
        if message_text.contains("Done exploring.") {
            self.game_state.set_explored(true);
        }

        if message_text.contains("<lightred>") {
            self.game_state.inc_monster_number_in_sight();
            log_debug!(
                "++monster_number_in_sight: {}",
                self.game_state.get_monster_number_in_sight()
            );
        }

        if message_text.contains("<red>You kill") {
            self.game_state.dec_monster_number_in_sight();
            log_debug!(
                "--monster_number_in_sight: {}",
                self.game_state.get_monster_number_in_sight()
            );
        }

        if message_text.contains("No target in view!") {
            self.game_state.set_monster_number_in_sight(0);
            log_debug!(
                "0 monster_number_in_sight: {}",
                self.game_state.get_monster_number_in_sight()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    fn create_mock_bot_loop_state() -> BotLoopState {
        let (s1, r1) = channel();
        let (_s2, r2) = channel();
        BotLoopState::new(r1, r2, s1)
    }

    // TODO: remove?
    // #[test]
    fn update_game_state_with_cells_complete() {
        // prepare
        let mut bot_loop_state = create_mock_bot_loop_state();
        let crawl_message = serde_json::from_str(
            r#"
            {
                "msg": "map",
                "cells": [
                    {
                        "test": 1
                    },
                    {
                        "mon": {
                            "name": "Crazy Yiuf",
                            "threat": 2
                        }
                    }
                ]
            }
            "#,
        )
        .unwrap();

        // execute
        bot_loop_state.update_game_state_with_cells(crawl_message);

        // expect

        // assert_eq!(bot_loop_state.game_state.get_monster_number_in_sight(), 1);
    }

    // TODO: remove?
    // #[test]
    fn update_game_state_with_cells_incomplete() {
        // prepare
        let mut bot_loop_state = create_mock_bot_loop_state();
        let crawl_message = serde_json::from_str(
            r#"
            {
                "msg": "map",
                "cells": [
                    {
                        "test": 1
                    },
                    {
                        "mon": {
                            "name": "Crazy Yiuf",
                            "nonsense": "123"
                        }
                    }
                ]
            }
            "#,
        )
        .unwrap();

        // execute
        bot_loop_state.update_game_state_with_cells(crawl_message);

        // expect
        // assert_eq!(bot_loop_state.game_state.get_monster_number_in_sight(), 1);
    }

    #[test]
    fn update_game_state_with_msgs_one_message() {
        // prepare
        let mut bot_loop_state = create_mock_bot_loop_state();
        let crawl_message = serde_json::from_str(
            r#"
            {
                "msg": "msgs",
                "messages": [
                    {
                        "text": "<lightred>A kobold is nearby!<lightgrey>",
                        "turn":17,"channel":6
                    }
                ]
            }
            "#,
        )
        .unwrap();

        // execute
        bot_loop_state.update_game_state_with_msgs(crawl_message);

        // expect
        assert_eq!(bot_loop_state.game_state.get_monster_number_in_sight(), 1);
    }

    #[test]
    fn update_game_state_with_msgs_missing_messages() {
        // prepare
        let mut bot_loop_state = create_mock_bot_loop_state();
        let crawl_message = serde_json::from_str(
            r#"
            {
                "msg": "msgs",
                "nonsense": 1
            }
            "#,
        )
        .unwrap();

        // execute
        bot_loop_state.update_game_state_with_msgs(crawl_message);

        // expect
        assert_eq!(bot_loop_state.game_state.get_monster_number_in_sight(), 0);
    }
}

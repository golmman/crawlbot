use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;

impl GameState {
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
            self.set_explored(true);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_game_state_with_msgs_done_exploring() {
        // prepare
        let mut game_state = GameState::new();

        let crawl_message = serde_json::from_str(
            r#"
            {
                "msg": "msgs",
                "messages": [
                    {
                        "text": "<lightred>Done exploring.<lightgrey>"
                    }
                ]
            }
            "#,
        )
        .unwrap();

        // execute
        game_state.update_game_state_with_msgs(crawl_message);

        // expect
        assert!(game_state.is_explored());
    }
}

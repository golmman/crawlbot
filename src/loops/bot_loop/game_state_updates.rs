use crate::loops::bot_loop::BotLoopState;
use crate::model::game_state::InputMode;
use crate::{log_crawl, log_debug};
use serde_json::Value;

impl BotLoopState {
    pub fn update_input_mode(&mut self, crawl_message: Value) {
        if let Some(m) = crawl_message["mode"].as_i64() {
            self.game_state.set_input_mode(InputMode::from_i64(m));
        }
    }

    pub fn update_game_state_with_cells(&mut self, mut crawl_message: Value) {
        let empty: &mut Vec<Value> = &mut Vec::new();
        let cells = crawl_message["cells"].as_array().unwrap_or(empty);

        for cell in cells {
            if let Some(mon) = cell["mon"].as_object() {
                let mon_name = mon["name"].as_str().expect("name to be a string");
                let mon_threat = mon["threat"].as_i64().expect("threat to be an integer");
                log_debug!("MONSTER SIGHTED: {}, THREAT: {}", mon_name, mon_threat);
            }

            // plant: threat = 0
            // yusuf: threat = 2

            // done exploring!
            // {\"msgs\":[{\"msg\":\"msgs\",\"messages\":[{\"text\":\"<lightgrey>Done exploring.<lightgrey>\",\"turn\":631,\"channel\":0}]}\n,{\"msg\":\"input_mode\",\"mode\":1}\n]}
        }
    }

    pub fn update_game_state_with_msgs(&mut self, mut crawl_message: Value) {
        let empty: &mut Vec<Value> = &mut Vec::new();
        let messages = crawl_message["messages"].as_array().unwrap_or(empty);

        for message in messages {
            let text = message["text"].as_str().expect("text to be a string");
            self.update_game_state_with_msgs_text(text);
        }
    }

    pub fn update_game_state_with_msgs_text(&mut self, message_text: &str) {
        if message_text.contains("Done exploring.") {
            self.game_state.set_explored(true);
        }

        if message_text.contains("<lightred>") {
            self.game_state.inc_enemy_number_in_sight();
            log_debug!(
                "++enemy_number_in_sight: {}",
                self.game_state.get_enemy_number_in_sight()
            );
        }

        if message_text.contains("<red>You kill") {
            self.game_state.dec_enemy_number_in_sight();
            log_debug!(
                "--enemy_number_in_sight: {}",
                self.game_state.get_enemy_number_in_sight()
            );
        }

        if message_text.contains("No target in view!") {
            self.game_state.set_enemy_number_in_sight(0);
            log_debug!(
                "0 enemy_number_in_sight: {}",
                self.game_state.get_enemy_number_in_sight()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    use serde_json::Result;

    fn create_mock_bot_loop_state() -> BotLoopState {
        let (s1, r1) = channel();
        let (_s2, r2) = channel();
        BotLoopState::new(r1, r2, s1)
    }

    fn print_value_option(o: Option<Value>) {
        println!("{:?}", o);
    }

    fn print_value(v: &Value) {
        println!("{:?}", v);
    }

    #[test]
    fn t12345() {
        // prepare
        let mut bot_loop_state = create_mock_bot_loop_state();
        let crawl_message: Value = serde_json::from_str(
            r#"
            {
                "msg": "msgs",
                "messages": [
                    {
                        "text": "<lightred>A kobold is nearby!<lightgrey>",
                        "turn":17,
                        "channel":6
                    },
                    {
                        "info": 123
                    }
                ]
            }
            "#,
        )
        .unwrap();

        println!("{:?}", crawl_message.pointer("/msg1"));
        println!("{:?}", crawl_message.pointer("/msg"));
        println!("{:?}", crawl_message.pointer("/messages/1/info"));

        let a = crawl_message["messages"].as_array().unwrap();

        for x in a.iter() {
            print_value(x);
        }
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
        assert_eq!(bot_loop_state.game_state.get_enemy_number_in_sight(), 1);
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
        assert_eq!(bot_loop_state.game_state.get_enemy_number_in_sight(), 0);
    }
}

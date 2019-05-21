extern crate websocket;

use super::super::model::instruction::Instruction;
use super::super::model::instruction::Routine;
use super::super::model::GameState;
use super::super::model::InputMode;
use super::super::routines::*;
use super::super::*;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct BotLoopState {
    receiver_stdin: Receiver<Instruction>,
    receiver_websocket: Receiver<Instruction>,
    pub sender_websocket: Sender<Instruction>,

    pub exit_loop: bool,
    game_state: GameState,
    primary_queue: VecDeque<Instruction>,
    secondary_queue: VecDeque<Instruction>,
}

impl BotLoopState {
    pub fn new(
        receiver_stdin: Receiver<Instruction>,
        receiver_websocket: Receiver<Instruction>,
        sender_websocket: Sender<Instruction>,
    ) -> Self {
        BotLoopState {
            receiver_stdin,
            receiver_websocket,
            sender_websocket,

            exit_loop: false,
            game_state: GameState::new(),
            primary_queue: VecDeque::new(),
            secondary_queue: VecDeque::new(),
        }
    }

    pub fn enqueue_routine(&mut self, routine_supplier: fn() -> Routine) {
        self.primary_queue.append(&mut routine_supplier())
    }

    fn enqueue_instruction(&mut self, instruction_supplier: fn() -> Instruction) {
        self.primary_queue.push_back(instruction_supplier())
    }

    fn pong(&self) {
        let _ = self
            .sender_websocket
            .send(Instruction::CrawlOutput("{\"msg\":\"pong\"}".to_string()));
    }

    fn update_input_mode(&mut self, crawl_message: Value) {
        if let Some(m) = crawl_message["mode"].as_i64() {
            self.game_state.set_input_mode(InputMode::from_i64(m));
        }
    }

    fn update_game_state_with_msgs(&mut self, mut crawl_message: Value) {
        let empty: &mut Vec<Value> = &mut Vec::new();
        let messages = crawl_message["messages"].as_array_mut().unwrap_or(empty);

        while !messages.is_empty() {
            let text = &messages.remove(0)["text"];
            self.update_game_state_with_msgs_text(text.as_str().unwrap());
        }
    }

    fn update_game_state_with_msgs_text(&mut self, message_text: &str) {
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

    fn fill_primary_queue(&mut self) {
        let message_stdin = self
            .receiver_stdin
            .try_recv()
            .unwrap_or(Instruction::Nothing);

        let message_websocket = self
            .receiver_websocket
            .try_recv()
            .unwrap_or(Instruction::Nothing);

        if message_stdin.is_something() {
            self.primary_queue.push_front(message_stdin);
        }

        if message_websocket.is_something() {
            self.primary_queue.push_front(message_websocket);
        }

        if !self.game_state.is_paused() {
            if self.primary_queue.is_empty() {
                log_debug!(
                    "primary_queue empty, idle_ticks: {}",
                    self.game_state.get_idle_ticks()
                );
                self.game_state.inc_idle_ticks();
                if self.game_state.get_idle_ticks() > 3 {
                    log_debug!("primary_queue empty, pausing.");
                    self.game_state.pause();
                }
            } else {
                self.game_state.set_idle_ticks(0);
            }

            if !self.secondary_queue.is_empty() {
                let routine_message = self.secondary_queue.pop_front().unwrap();
                log_debug!("Popping from routine queue: {:?}", routine_message);
                self.primary_queue.push_back(routine_message);
            }
        }
    }

    fn get_next_instruction(&mut self) -> Instruction {
        let message = self
            .primary_queue
            .pop_front()
            .unwrap_or(Instruction::Nothing);

        if message.is_something() {
            let m = format!("{:?}", message);
            log_debug!("Processing {:80.80} ... {} total chars", m, m.len());
        }

        message
    }
}

// impl BotLoopState {
//     fn abandon(&mut self) {
//         self.enqueue_routine(supply_routine_abandon);
//     }

//     fn close(&mut self) {
//         let _ = self.sender_websocket.send(Instruction::Close);
//         self.exit_loop = true;
//     }
// }

impl LoopState<String, String> for BotLoopState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e);
    }

    fn run_loop(&mut self) -> Result<String, String> {
        std::thread::sleep(std::time::Duration::from_millis(100));

        self.fill_primary_queue();

        match self.get_next_instruction() {
            Instruction::Abandon => self.abandon(),
            Instruction::ClearRoutines => {
                self.secondary_queue.clear();
            }
            Instruction::Close => self.close(),
            Instruction::CrawlInput(crawl_message) => {
                let crawl_msg = &crawl_message["msg"];

                match crawl_msg.as_str().unwrap() {
                    "ping" => self.pong(),
                    "msgs" => self.update_game_state_with_msgs(crawl_message),
                    "input_mode" => self.update_input_mode(crawl_message),
                    _ => {}
                }
            }
            Instruction::CrawlOutput(data) => {
                let _ = self.sender_websocket.send(Instruction::CrawlOutput(data));
            }
            Instruction::GetStatus => {
                log_debug!("--- STATUS ---");
                println!("secondary_queue = {:#?}", self.secondary_queue);
                println!("game_state = {:#?}", self.game_state);
                log_debug!("--------------");
            }
            Instruction::Idle10 => {
                self.enqueue_routine(supply_routine_idle10);
            }
            Instruction::Idle5 => {
                self.enqueue_routine(supply_routine_idle5);
            }
            Instruction::IfThenElse(check, then_routine, else_routine) => {
                if check(self.game_state) {
                    self.enqueue_routine(then_routine);
                } else {
                    self.enqueue_routine(else_routine);
                }
            }
            Instruction::Main => {
                self.enqueue_routine(supply_routine_main);
            }
            Instruction::Nothing => {}
            Instruction::Pause => {
                self.game_state.pause();
            }
            Instruction::PickMiFi => {
                self.enqueue_routine(supply_routine_pick_mifi);
            }
            Instruction::PickTrBe => {
                self.enqueue_routine(supply_routine_pick_trbe);
            }
            Instruction::Ping(data) => {
                let _ = self.sender_websocket.send(Instruction::Pong(data));
            }
            Instruction::Script(evaluate) => {
                self.secondary_queue.append(&mut evaluate(self.game_state))
            }
            Instruction::Start => {
                self.enqueue_routine(supply_routine_start);
            }
            Instruction::Unpause => {
                self.game_state.unpause();
            }
            _ => {
                log_warn!("Unknown message.");
            }
        };

        if self.exit_loop {
            Err(String::from("Exiting loop_bot..."))
        } else {
            Ok(String::from(""))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_bot_loop_state() -> BotLoopState {
        let (s1, r1) = channel();
        let (_s2, r2) = channel();
        BotLoopState::new(r1, r2, s1)
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

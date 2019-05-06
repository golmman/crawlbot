extern crate websocket;

use super::super::model::instruction::Instruction;
use super::super::model::CrawlInputMsgs;
use super::super::model::GameState;
use super::super::routines::*;
use super::super::*;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub trait LoopState<T, E> {
    fn assess_error(&self, e: E);
    fn assess_iteration(&self, t: T);
    fn run_loop(&mut self) -> Result<T, E>;

    fn start_loop(&mut self) {
        loop {
            match self.run_loop() {
                Ok(t) => {
                    self.assess_iteration(t);
                }
                Err(e) => {
                    self.assess_error(e);
                    break;
                }
            }
        }
    }
}

pub struct LoopBotState {
    receiver_stdin: Receiver<Instruction>,
    receiver_websocket: Receiver<Instruction>,
    sender_websocket: Sender<Instruction>,

    game_state: GameState,
    message_queue: VecDeque<Instruction>,
    routine_queue: VecDeque<Instruction>,
}

impl LoopBotState {
    pub fn new(
        receiver_stdin: Receiver<Instruction>,
        receiver_websocket: Receiver<Instruction>,
        sender_websocket: Sender<Instruction>,
    ) -> Self {
        LoopBotState {
            receiver_stdin,
            receiver_websocket,
            sender_websocket,

            game_state: GameState::new(),
            message_queue: VecDeque::new(),
            routine_queue: VecDeque::new(),
        }
    }

    fn abc(&self) -> i32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let (s1, r1) = channel();
        let (_s2, r2) = channel();
        let x = LoopBotState::new(r1, r2, s1);
        x.assess_error(String::from(""));
        assert_eq!(x.abc(), 10);
    }
}

impl LoopState<String, String> for LoopBotState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e)
    }

    fn run_loop(&mut self) -> Result<String, String> {
        std::thread::sleep(std::time::Duration::from_millis(100));

        let message_stdin = self
            .receiver_stdin
            .try_recv()
            .unwrap_or(Instruction::Nothing);
        let message_websocket = self
            .receiver_websocket
            .try_recv()
            .unwrap_or(Instruction::Nothing);

        if message_stdin.is_something() {
            self.message_queue.push_front(message_stdin);
        }

        if message_websocket.is_something() {
            self.message_queue.push_front(message_websocket);
        }

        if !self.game_state.is_paused() {
            if self.message_queue.is_empty() {
                log_debug!(
                    "message_queue empty, idle_ticks: {}",
                    self.game_state.get_idle_ticks()
                );
                self.game_state.inc_idle_ticks();
                if self.game_state.get_idle_ticks() > 3 {
                    log_debug!("message_queue empty, pausing.");
                    self.game_state.pause();
                }
            } else {
                self.game_state.set_idle_ticks(0);
            }

            if !self.routine_queue.is_empty() {
                let routine_message = self.routine_queue.pop_front().unwrap();
                log_debug!("Popping from routine queue: {:?}", routine_message);
                self.message_queue.push_back(routine_message);
            }
        }

        let message = self
            .message_queue
            .pop_front()
            .unwrap_or(Instruction::Nothing);

        if message.is_something() {
            let m = format!("{:?}", message);
            log_debug!("Processing {:80.80} ... {} total chars", m, m.len());
        }

        match message {
            Instruction::ClearRoutines => {
                self.routine_queue.clear();
            }
            Instruction::Close => {
                let _ = self.sender_websocket.send(Instruction::Close);
                return Err(String::from("Exiting loop_bot..."));
            }
            Instruction::Pause => {
                self.game_state.pause();
            }
            Instruction::Unpause => {
                self.game_state.unpause();
            }
            Instruction::IfThenElse(check, then_routine, else_routine) => {
                if check(self.game_state) {
                    push_routine(&mut self.routine_queue, then_routine);
                } else {
                    push_routine(&mut self.routine_queue, else_routine);
                }
            }
            Instruction::Script(evaluate) => {
                push_routine(&mut self.routine_queue, evaluate(self.game_state));
            }
            Instruction::Abandon => {
                push_routine(&mut self.routine_queue, create_routine_abandon);
            }
            Instruction::Idle10 => {
                push_routine(&mut self.routine_queue, create_routine_idle10);
            }
            Instruction::Idle5 => {
                push_routine(&mut self.routine_queue, create_routine_idle5);
            }
            Instruction::Main => {
                push_routine(&mut self.routine_queue, create_routine_main);
            }
            Instruction::PickMiFi => {
                push_routine(&mut self.routine_queue, create_routine_pick_mifi);
            }
            Instruction::PickTrBe => {
                push_routine(&mut self.routine_queue, create_routine_pick_trbe);
            }
            Instruction::Start => {
                push_routine(&mut self.routine_queue, create_routine_start);
            }
            Instruction::GetStatus => {
                log_debug!("--- STATUS ---");
                println!("routine_queue = {:#?}", self.routine_queue);
                println!("game_state = {:#?}", self.game_state);
                log_debug!("--------------");
            }

            Instruction::Ping(data) => {
                let _ = self.sender_websocket.send(Instruction::Pong(data));
            }
            Instruction::CrawlOutput(data) => {
                let _ = self.sender_websocket.send(Instruction::CrawlOutput(data));
            }
            Instruction::CrawlInput(crawl_message) => {
                let crawl_input: Value = serde_json::from_str(crawl_message.as_str()).unwrap();
                let crawl_msg = &crawl_input["msg"];

                match crawl_msg.as_str().unwrap() {
                    "ping" => {
                        let _ = self
                            .sender_websocket
                            .send(Instruction::CrawlOutput("{\"msg\":\"pong\"}".to_string()));
                    }
                    "msgs" => update_game_state_with_msgs(&mut self.game_state, crawl_message),
                    _ => {}
                }
            }
            Instruction::Nothing => {}
            _ => {
                log_warn!("Unknown message.");
            }
        };

        Ok(String::from(""))
    }
}

fn update_game_state_with_msgs(game_state: &mut GameState, crawl_message: String) {
    let input: CrawlInputMsgs = serde_json::from_str(crawl_message.as_str()).unwrap();

    for message in input.messages {
        update_game_state_with_msgs_text(game_state, message.text);
    }
}

fn update_game_state_with_msgs_text(game_state: &mut GameState, message_text: String) {
    if message_text.find("Done exploring.").is_some() {
        game_state.set_explored(true);
    }

    if message_text.find("<lightred>").is_some() {
        game_state.inc_enemy_number_in_sight();
        log_debug!(
            "++enemy_number_in_sight: {}",
            game_state.get_enemy_number_in_sight()
        );
    }

    if message_text.find("<red>You kill").is_some() {
        game_state.dec_enemy_number_in_sight();
        log_debug!(
            "--enemy_number_in_sight: {}",
            game_state.get_enemy_number_in_sight()
        );
    }

    if message_text.find("No target in view!").is_some() {
        game_state.set_enemy_number_in_sight(0);
        log_debug!(
            "0 enemy_number_in_sight: {}",
            game_state.get_enemy_number_in_sight()
        );
    }
}

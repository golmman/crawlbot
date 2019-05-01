extern crate websocket;

use super::super::model::CrawlInputMsgs;
use super::super::model::GameState;
use super::super::routines::*;
use super::super::*;
use super::internal_message::Instruction;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub fn run_loop_bot(
    receiver_stdin: Receiver<Instruction>,
    receiver_websocket: Receiver<Instruction>,
    sender_websocket: Sender<Instruction>,
) {
    let mut game_state = GameState::new();
    let mut message_queue: VecDeque<Instruction> = VecDeque::new();
    let mut routine_queue: VecDeque<Instruction> = VecDeque::new();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        let message_stdin = receiver_stdin.try_recv().unwrap_or(Instruction::Nothing);
        let message_websocket = receiver_websocket
            .try_recv()
            .unwrap_or(Instruction::Nothing);

        if message_stdin.is_something() {
            message_queue.push_front(message_stdin);
        }

        if message_websocket.is_something() {
            message_queue.push_front(message_websocket);
        }

        if routine_queue.is_empty() {
            // log_debug!("Routine queue is empty, pausing now!");
            game_state.set_paused(true);
        } else {
            // log_debug!("Routine queue is not empty, unpausing now!");
            game_state.set_paused(false);
        }

        if !game_state.is_paused() {
            let routine_message = routine_queue.pop_front().unwrap();
            log_debug!("Popping from routine queue: {:?}", routine_message);
            message_queue.push_back(routine_message);
        }

        let message = message_queue.pop_front().unwrap_or(Instruction::Nothing);

        if message.is_something() {
            let m = format!("{:?}", message);
            log_debug!("Processing {:80.80} ... {} total chars", m, m.len());
        }

        match message {
            Instruction::ClearRoutines => {
                routine_queue.clear();
            }
            Instruction::Close => {
                let _ = sender_websocket.send(Instruction::Close);
                break;
            }
            Instruction::Pause => {
                game_state.set_paused(true);
            }
            Instruction::Unpause => {
                game_state.set_paused(false);
            }
            Instruction::IfThenElse(check, then_routine, else_routine) => {
                if check(game_state) {
                    push_routine(&mut routine_queue, then_routine);
                } else {
                    push_routine(&mut routine_queue, else_routine);
                }
            }
            Instruction::Abandon => {
                push_routine(&mut routine_queue, create_routine_abandon);
            }
            Instruction::Idle10 => {
                push_routine(&mut routine_queue, create_routine_idle10);
            }
            Instruction::Idle5 => {
                push_routine(&mut routine_queue, create_routine_idle5);
            }
            Instruction::Main => {
                push_routine(&mut routine_queue, create_routine_main);
            }
            Instruction::PickMiFi => {
                push_routine(&mut routine_queue, create_routine_pick_mifi);
            }
            Instruction::PickTrBe => {
                push_routine(&mut routine_queue, create_routine_pick_trbe);
            }
            Instruction::Start => {
                push_routine(&mut routine_queue, create_routine_start);
            }
            Instruction::GetStatus => {
                log_debug!("--- STATUS ---");
                println!("routine_queue = {:#?}", routine_queue);
                println!("game_state = {:#?}", game_state);
                log_debug!("--------------");
            }

            Instruction::Ping(data) => {
                let _ = sender_websocket.send(Instruction::Pong(data));
            }
            Instruction::CrawlOutput(data) => {
                let _ = sender_websocket.send(Instruction::CrawlOutput(data));
            }
            Instruction::CrawlInput(crawl_message) => {
                let crawl_input: Value = serde_json::from_str(crawl_message.as_str()).unwrap();
                let crawl_msg = &crawl_input["msg"];

                match crawl_msg.as_str().unwrap() {
                    "ping" => {
                        let _ = sender_websocket
                            .send(Instruction::CrawlOutput("{\"msg\":\"pong\"}".to_string()));
                    }
                    "msgs" => {
                        let input: CrawlInputMsgs =
                            serde_json::from_str(crawl_message.as_str()).unwrap();

                        for message in input.messages {
                            // if message.text.find("comes into view.").is_some() {
                            //     game_state.inc_enemy_number_in_sight();
                            //     log_debug!(
                            //         "++enemy_number_in_sight: {}",
                            //         game_state.get_enemy_number_in_sight()
                            //     );
                            // }
                            // if message.text.find("is nearby!").is_some() {
                            //     game_state.inc_enemy_number_in_sight();
                            //     log_debug!(
                            //         "++enemy_number_in_sight: {}",
                            //         game_state.get_enemy_number_in_sight()
                            //     );
                            // }
                            if message.text.find("<lightred>").is_some() {
                                game_state.inc_enemy_number_in_sight();
                                log_debug!(
                                    "++enemy_number_in_sight: {}",
                                    game_state.get_enemy_number_in_sight()
                                );
                            }

                            if message.text.find("<red>You kill").is_some() {
                                game_state.dec_enemy_number_in_sight();
                                log_debug!(
                                    "--enemy_number_in_sight: {}",
                                    game_state.get_enemy_number_in_sight()
                                );
                            }

                            if message.text.find("No target in view!").is_some() {
                                game_state.set_enemy_number_in_sight(0);
                                log_debug!(
                                    "0 enemy_number_in_sight: {}",
                                    game_state.get_enemy_number_in_sight()
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
            Instruction::Nothing => {}
            _ => {
                log_warn!("Unknown message.");
            }
        };
    }

    log_debug!("Exiting loop_bot...");
}

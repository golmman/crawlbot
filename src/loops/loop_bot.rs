extern crate websocket;

use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use serde_json::Value;

use super::super::*;
use super::internal_message::InternalMessage;

use super::super::routines::*;

use super::super::model::GameState;

pub fn run_loop_bot(
    receiver_stdin: Receiver<InternalMessage>,
    receiver_websocket: Receiver<InternalMessage>,
    sender_websocket: Sender<InternalMessage>,
) {
    let mut pause = true;
    let mut game_state = GameState::new();
    let mut message_queue: VecDeque<InternalMessage> = VecDeque::new();
    let mut routine_queue: VecDeque<InternalMessage> = VecDeque::new();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        let message_stdin = receiver_stdin
            .try_recv()
            .unwrap_or(InternalMessage::Nothing);
        let message_websocket = receiver_websocket
            .try_recv()
            .unwrap_or(InternalMessage::Nothing);

        if message_stdin.is_something() {
            message_queue.push_back(message_stdin);
        }

        if message_websocket.is_something() {
            message_queue.push_back(message_websocket);
        }

        if !pause {
            if routine_queue.is_empty() {
                log_debug!("Routine queue is empty, pausing now!");
                message_queue.push_back(InternalMessage::Pause);
            } else {
                let routine_message = routine_queue.pop_front().unwrap();
                log_debug!("Popping from routine queue: {:?}", routine_message);
                message_queue.push_back(routine_message);
            }
        }

        let message = message_queue
            .pop_front()
            .unwrap_or(InternalMessage::Nothing);

        if message.is_something() {
            let m = format!("{:?}", message);
            log_debug!("Processing {:80.80} ... {} total chars", m, m.len());
        }

        match message {
            InternalMessage::ClearRoutines => {
                routine_queue.clear();
            }
            InternalMessage::Close => {
                let _ = sender_websocket.send(InternalMessage::Close);
                break;
            }
            InternalMessage::Pause => {
                pause = true;
                game_state.set_paused(true);
            }
            InternalMessage::Unpause => {
                pause = false;
            }
            InternalMessage::Abandon => {
                push_routine(&mut routine_queue, create_routine_abandon);
            }
            InternalMessage::Idle10 => {
                push_routine(&mut routine_queue, create_routine_idle10);
            }
            InternalMessage::Idle5 => {
                push_routine(&mut routine_queue, create_routine_idle5);
            }
            InternalMessage::PickMiFi => {
                push_routine(&mut routine_queue, create_routine_pick_mifi);
            }
            InternalMessage::PickTrBe => {
                push_routine(&mut routine_queue, create_routine_pick_trbe);
            }
            InternalMessage::Start => {
                push_routine(&mut routine_queue, create_routine_start);
            }
            InternalMessage::GetStatus => {
                log_debug!(",--- STATUS ---");
                log_debug!("| message_queue.len: {:?}", message_queue.len());
                log_debug!("| routine_queue.len: {:?}", routine_queue.len());
                log_debug!("`--------------");
            }

            InternalMessage::Ping(data) => {
                let _ = sender_websocket.send(InternalMessage::Pong(data));
            }
            InternalMessage::CrawlOutput(data) => {
                let _ = sender_websocket.send(InternalMessage::CrawlOutput(data));
            }
            InternalMessage::CrawlInput(crawl_message) => {
                let crawl_input: Value = serde_json::from_str(crawl_message.as_str()).unwrap();
                let crawl_msg = &crawl_input["msg"];

                match crawl_msg.as_str().unwrap() {
                    "ping" => {
                        let _ = sender_websocket.send(InternalMessage::CrawlOutput(
                            "{\"msg\":\"pong\"}".to_string(),
                        ));
                    }
                    "test" => {}
                    _ => {}
                }
            }
            InternalMessage::Nothing => {}
            _ => {
                log_warn!("Unknown message.");
            }
        };
    }

    log_debug!("Exiting loop_bot...");
}

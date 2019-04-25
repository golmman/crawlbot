extern crate websocket;

use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use super::super::*;
use super::internal_message::InternalMessage;

use super::super::model::CrawlMessage;

use super::super::routines::push_routine;
use super::super::routines::NOTHING5;

pub fn run_loop_bot(
    receiver_stdin: Receiver<InternalMessage>,
    receiver_websocket: Receiver<InternalMessage>,
    sender_websocket: Sender<InternalMessage>,
) {
    let mut pause = true;
    let mut message_queue: VecDeque<InternalMessage> = VecDeque::new();
    let mut routine_queue: VecDeque<InternalMessage> = VecDeque::new();


    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));



        // TODO websocket messages?
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
            log_debug!("test");
            if !routine_queue.is_empty() {
                let routine_message = routine_queue.pop_front().unwrap();
                log_debug!("Popping from routine queue: {:?}", routine_message);
                message_queue.push_back(routine_message);
            }
        }

        let message = message_queue
            .pop_front()
            .unwrap_or(InternalMessage::Nothing);

        match message {
            InternalMessage::Pause => {
                log_debug!("Message: Pause");
                pause = true;
            }
            InternalMessage::Unpause => {
                log_debug!("Message: Unpause");
                pause = false;
            }
            InternalMessage::Idle => {
                log_debug!("Message: Idle");
                push_routine(&mut routine_queue, NOTHING5);
            }
            InternalMessage::GetStatus => {
                log_debug!(",--- STATUS ---");
                log_debug!("| message_queue.len: {:?}", message_queue.len());
                log_debug!("| routine_queue.len: {:?}", routine_queue.len());
                log_debug!("`--------------");
            }
            InternalMessage::Close => {
                log_debug!("Message Close");
                let _ = sender_websocket.send(InternalMessage::Close);
                break;
            }
            InternalMessage::Ping(data) => {
                log_debug!("Message Ping");
                let _ = sender_websocket.send(InternalMessage::Pong(data));
            }
            InternalMessage::Proxy(data) => {
                log_debug!("Message Proxy: {}", data);
                let _ = sender_websocket.send(InternalMessage::Proxy(data));
            }
            InternalMessage::CrawlData(crawl_message) => {
                log_debug!("Message CrawlData: {:?}", crawl_message);

                match crawl_message.msg.as_str() {
                    "ping" => {
                        sender_websocket.send(InternalMessage::CrawlData(CrawlMessage { msg: "pong".to_string() }));
                    }
                    _ => {}
                }
            }
            InternalMessage::Nothing => {}
            _ => {
                log_debug!("loop_bot message unknown: {}", message);
            }
        };
    }

    log_info!("Exiting loop_bot...");
}

extern crate websocket;

use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use super::super::*;
use super::internal_message::InternalMessage;

pub fn run_loop_bot(
    receiver_stdin: Receiver<InternalMessage>,
    receiver_websocket: Receiver<InternalMessage>,
    sender_websocket: Sender<InternalMessage>,
) {
    let mut pause = false;
    let mut message_queue: VecDeque<InternalMessage> = VecDeque::new();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        if !pause {
            log_debug!("test");
        }

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
            InternalMessage::GetStatus => {
                log_debug!(",--- STATUS ---");
                log_debug!("| message_queue.len: {:?}", message_queue.len());
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
                log_debug!("Message Proxy");
                let _ = sender_websocket.send(InternalMessage::Proxy(data));
            }
            InternalMessage::Nothing => {}
            _ => {
                log_debug!("loop_bot message unknown: {}", message);
            }
        };
    }

    log_info!("Exiting loop_bot...");
}

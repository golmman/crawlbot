extern crate websocket;

use std::sync::mpsc::Sender;

use super::super::*;
use super::internal_message::InternalMessage;

pub fn run_loop_stdin(sender_bot: Sender<InternalMessage>) {
    let mut pause = false;

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Unable to read from stdin.");
        let trimmed = input.trim();

        let message = match trimmed {
            "" => {
                if pause {
                    pause = false;
                    InternalMessage::Unpause
                } else {
                    pause = true;
                    InternalMessage::Pause
                }
            }
            "/get_status" => InternalMessage::GetStatus,
            "/close" => {
                let _ = sender_bot.send(InternalMessage::Close);
                break;
            }
            _ => InternalMessage::Proxy(trimmed.to_string()),
        };

        match sender_bot.send(message) {
            Ok(()) => (),
            Err(e) => {
                log_error!("An error occured when trying to send a message: {:?}", e);
                break;
            }
        }
    }

    log_info!("Exiting loop_stdin...");
}

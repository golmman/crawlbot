extern crate websocket;

use websocket::OwnedMessage;

use super::super::*;

pub fn run_loop_stdin(
    sender_loop_bot: std::sync::mpsc::Sender<String>,
    sender: std::sync::mpsc::Sender<websocket::OwnedMessage>,
) {
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        let message = match trimmed {
            "/close" => {
                // Close the connection
                let _ = sender.send(OwnedMessage::Close(None));
                let _ = sender_loop_bot.send("/close".to_string());
                break;
            }
            // Send a ping
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            // Otherwise, just send text
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match sender.send(message) {
            Ok(()) => (),
            Err(e) => {
                log_error!("Main Loop: {:?}", e);
                break;
            }
        }

        match sender_loop_bot.send(trimmed.to_string()) {
            Ok(()) => (),
            Err(e) => {
                log_error!("Main Loop: {:?}", e);
                break;
            }
        }
    }

    log_info!("Exiting loop_stdin...");
}

extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::Receiver;

use websocket::sender::Writer;
use websocket::{Message, OwnedMessage};

use super::super::*;

pub fn run_loop_wssend(mut ws_writer: Writer<TcpStream>, receiver: Receiver<OwnedMessage>) {
    loop {
        let message = match receiver.recv() {
            Ok(m) => m,
            Err(e) => {
                log_error!("An error occured while receiving a message: {:?}", e);
                break;
            }
        };

        if let OwnedMessage::Close(close_data_option) = message {
            log_info!("Server sent 'close'.");
            if let Some(close_data) = close_data_option {
                log_info!(
                    "Close was with status {:?} and reason {:?}.",
                    close_data.status_code,
                    close_data.reason,
                );
            }
            let _ = ws_writer.send_message(&Message::close());
            break;
        }

        // Send the message
        match ws_writer.send_message(&message) {
            Ok(()) => (),
            Err(e) => {
                log_error!("An error occured while writing a ws message: {:?}", e);
                let _ = ws_writer.send_message(&Message::close());
                break;
            }
        }
    }

    log_info!("Exiting loop_wssend...");
}

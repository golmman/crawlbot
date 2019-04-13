extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::Sender;

use websocket::OwnedMessage;
use websocket::receiver::Reader;

use super::super::*;

pub fn run_loop_wsrecv(
    mut ws_reader: Reader<TcpStream>,
    sender: Sender<OwnedMessage>,
) {
    // Receive loop
    for message_result in ws_reader.incoming_messages() {
        let message = match message_result {
            Ok(m) => m,
            Err(e) => {
                log_error!("An error occured while reading a ws message: {:?}", e);
                let _ = sender.send(OwnedMessage::Close(None));
                break;
            }
        };

        match message {
            OwnedMessage::Close(_) => {
                let _ = sender.send(OwnedMessage::Close(None));
                break;
            }
            OwnedMessage::Ping(data) => {
                match sender.send(OwnedMessage::Pong(data)) {
                    Ok(()) => (),
                    Err(e) => {
                        log_error!("An error occured while sending a pong response: {:?}", e);
                        break;
                    }
                }
            }
            // Say what we received
            _ => log_debug!("loop_wsrecv message {:?}", message),
        }
    }

    log_info!("Exiting loop_wsrecv...");
}

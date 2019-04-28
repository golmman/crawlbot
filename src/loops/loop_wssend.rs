extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::Receiver;

use websocket::sender::Writer;
use websocket::Message;

use super::super::*;
use super::internal_message::InternalMessage;

pub fn run_loop_wssend(mut ws_writer: Writer<TcpStream>, receiver: Receiver<InternalMessage>) {
    loop {
        let message = match receiver.recv() {
            Ok(m) => m,
            Err(e) => {
                log_error!("An error occured while receiving a message: {:?}", e);
                break;
            }
        };

        log_debug!("Processing {:?}", message);

        match message {
            InternalMessage::Close => {
                let _ = ws_writer.send_message(&Message::close());
                break;
            }
            InternalMessage::Pong(data) => {
                let _ = ws_writer.send_message(&Message::pong(data));
            }
            InternalMessage::CrawlOutput(data) => {
                let _ = ws_writer.send_message(&Message::text(data));
            }
            InternalMessage::CrawlInput(crawl_message) => {
                log_warn!("CrawlInput {:?}", crawl_message);
            }
            _ => { log_warn!("Unknown message."); }
        }
    }

    log_debug!("Exiting loop_wssend...");
}

extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::Receiver;

use serde_json;

use websocket::sender::Writer;
use websocket::{Message, OwnedMessage};

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

        match message {
            InternalMessage::Close => {
                log_info!("Close");
                let _ = ws_writer.send_message(&Message::close());
                break;
            }
            InternalMessage::Pong(data) => {
                log_info!("Pong");
                let _ = ws_writer.send_message(&Message::pong(data));
            }
            InternalMessage::Proxy(data) => {
                log_info!("Proxy");
                let _ = ws_writer.send_message(&Message::text(data));
            }
            InternalMessage::CrawlData(crawl_message) => {
                log_info!("CrawlData");
                let crawl_message_strinfified = serde_json::to_string(&crawl_message).unwrap();
                let _ = ws_writer.send_message(&Message::text(crawl_message_strinfified));
            }
            _ => {
                log_warn!("Unexpected");
            }
        }
    }

    log_info!("Exiting loop_wssend...");
}

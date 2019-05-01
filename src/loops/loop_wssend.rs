extern crate websocket;

use super::super::*;
use super::instruction::Instruction;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use websocket::sender::Writer;
use websocket::Message;

pub fn run_loop_wssend(mut ws_writer: Writer<TcpStream>, receiver: Receiver<Instruction>) {
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
            Instruction::Close => {
                let _ = ws_writer.send_message(&Message::close());
                break;
            }
            Instruction::Pong(data) => {
                let _ = ws_writer.send_message(&Message::pong(data));
            }
            Instruction::CrawlOutput(data) => {
                let _ = ws_writer.send_message(&Message::text(data));
            }
            Instruction::CrawlInput(crawl_message) => {
                log_warn!("CrawlInput {:?}", crawl_message);
            }
            _ => {
                log_warn!("Unknown message.");
            }
        }
    }

    log_debug!("Exiting loop_wssend...");
}

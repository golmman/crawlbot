extern crate websocket;

use crate::model::instruction::Instruction;
use crate::{log_crawl, log_debug, log_warn, LoopState};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use websocket::sender::Writer;
use websocket::Message;

pub struct WriterLoopState {
    ws_writer: Writer<TcpStream>,
    receiver: Receiver<Instruction>,
}

impl WriterLoopState {
    pub fn new(ws_writer: Writer<TcpStream>, receiver: Receiver<Instruction>) -> Self {
        WriterLoopState {
            ws_writer,
            receiver,
        }
    }
}

impl LoopState<String, String> for WriterLoopState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e)
    }

    fn run_loop(&mut self) -> Result<String, String> {
        let message = match self.receiver.recv() {
            Ok(m) => m,
            Err(_) => {
                return Err(String::from("An error occured while receiving a message."));
            }
        };

        log_debug!("Processing {:?}", message);

        match message {
            Instruction::Close => {
                let _ = self.ws_writer.send_message(&Message::close());
                return Err(String::from("Exiting writer_loop..."));
            }
            Instruction::Pong(data) => {
                let _ = self.ws_writer.send_message(&Message::pong(data));
            }
            Instruction::CrawlOutput(data) => {
                let _ = self.ws_writer.send_message(&Message::text(data));
            }
            Instruction::CrawlMessage(message) => {
                log_warn!("CrawlMessage {:?}", message);
            }
            _ => {
                log_warn!("Unknown message.");
            }
        }

        Ok(String::from(""))
    }
}

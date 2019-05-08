extern crate websocket;

use super::super::model::instruction::Instruction;
use super::super::{log_crawl, log_debug, log_error, log_warn, LoopState};
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
            Err(e) => {
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
            Instruction::CrawlInput(crawl_message) => {
                log_warn!("CrawlInput {:?}", crawl_message);
            }
            _ => {
                log_warn!("Unknown message.");
            }
        }

        Ok(String::from(""))
    }
}

pub fn run_writer_loop(mut ws_writer: Writer<TcpStream>, receiver: Receiver<Instruction>) {
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

    log_debug!("Exiting writer_loop...");
}
extern crate websocket;

use super::super::model::instruction::Instruction;
use super::super::*;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use websocket::receiver::Reader;
use websocket::OwnedMessage;

pub struct ReaderLoopState {
    ws_reader: Reader<TcpStream>,
    sender: Sender<Instruction>,
    debug_counter: u32,
}

impl ReaderLoopState {
    pub fn new(ws_reader: Reader<TcpStream>, sender: Sender<Instruction>) -> Self {
        ReaderLoopState {
            ws_reader,
            sender,
            debug_counter: 0,
        }
    }
}

impl LoopState<String, String> for ReaderLoopState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e)
    }

    fn run_loop(&mut self) -> Result<String, String> {
        let message_result = self.ws_reader.incoming_messages().next().unwrap();

        let message = match message_result {
            Ok(m) => m,
            Err(e) => {
                let _ = self.sender.send(Instruction::Close);
                return Err(String::from("An error occured while reading a ws message."));
            }
        };

        match message {
            OwnedMessage::Close(_) => {
                let _ = self.sender.send(Instruction::Close);
                return Err(String::from("Exiting loop_wsrecv..."));
            }
            OwnedMessage::Ping(data) => match self.sender.send(Instruction::Ping(data)) {
                Ok(()) => (),
                Err(e) => {
                    return Err(String::from("An error occured while sending a pong response"));
                }
            },
            OwnedMessage::Text(data) => {
                if data.len() < 10000 {
                    log_debug!("Processing {:?}", data);
                } else {
                    log_debug!(
                        "Text was too long! -> stored in debug{}.json",
                        self.debug_counter
                    );
                    let filename = format!("debug{}.json", self.debug_counter);
                    let mut file = File::create(filename).unwrap();
                    let _ = file.write_all(data.as_bytes());
                    self.debug_counter += 1;
                }

                let crawl_input: Value = serde_json::from_str(data.as_str()).unwrap();
                let crawl_msgs: &Value = &crawl_input["msgs"];

                for crawl_msg in crawl_msgs.as_array().unwrap() {
                    let _ = self.sender.send(Instruction::CrawlInput(crawl_msg.to_string()));
                }
            }
            _ => {
                log_warn!("Unknown message.");
            }
        }

        Ok(String::from(""))
    }
}

extern crate websocket;

use crate::model::cws::root::CwsRoot;
use crate::model::instruction::Instruction;
use crate::{log_crawl, log_debug, log_warn, LoopState};
use std::fs::File;
use std::io::prelude::Write;
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

    fn log_text_data(&mut self, data: &str) {
        if data.len() < 10000 {
            log_debug!("Processing {}", data);
        } else {
            log_debug!("Text was too long! -> stored in debug{}.json", self.debug_counter);
            let filename = format!("debug{}.json", self.debug_counter);
            let mut file = File::create(filename).expect("file to be created");
            let _ = file.write_all(data.as_bytes());
            self.debug_counter += 1;
        }
    }
}

impl LoopState<String, String> for ReaderLoopState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e)
    }

    fn run_loop(&mut self) -> Result<String, String> {
        let message_result = self
            .ws_reader
            .incoming_messages()
            .next()
            .expect("there to be a next message");

        let message = match message_result {
            Ok(m) => m,
            Err(_) => {
                let _ = self.sender.send(Instruction::Close);
                return Err(String::from("An error occured while reading a ws message."));
            }
        };

        match message {
            OwnedMessage::Close(_) => {
                let _ = self.sender.send(Instruction::Close);
                return Err(String::from("Exiting reader_loop..."));
            }
            OwnedMessage::Ping(data) => match self.sender.send(Instruction::Ping(data)) {
                Ok(()) => (),
                Err(_) => {
                    return Err(String::from("An error occured while sending a pong response"));
                }
            },
            OwnedMessage::Text(data) => {
                self.log_text_data(&data);

                let root: CwsRoot = serde_json::from_str(&data).unwrap();

                if let Some(msgs) = root.msgs {
                    for message in msgs {
                        let _ = self.sender.send(Instruction::CrawlMessage(message));
                    }
                }
            }
            _ => {
                log_warn!("Unknown message.");
            }
        }

        Ok(String::from(""))
    }
}

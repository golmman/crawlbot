extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::Sender;

use std::fs::File;
use std::io::prelude::*;

use serde_json::Value;

use websocket::receiver::Reader;
use websocket::OwnedMessage;

use super::super::*;
use super::internal_message::Instruction;

pub fn run_loop_wsrecv(mut ws_reader: Reader<TcpStream>, sender: Sender<Instruction>) {
    let mut debug_counter = 0;

    // Receive loop
    for message_result in ws_reader.incoming_messages() {
        let message = match message_result {
            Ok(m) => m,
            Err(e) => {
                log_error!("An error occured while reading a ws message: {:?}", e);
                let _ = sender.send(Instruction::Close);
                break;
            }
        };

        match message {
            OwnedMessage::Close(_) => {
                let _ = sender.send(Instruction::Close);
                break;
            }
            OwnedMessage::Ping(data) => match sender.send(Instruction::Ping(data)) {
                Ok(()) => (),
                Err(e) => {
                    log_error!("An error occured while sending a pong response: {:?}", e);
                    break;
                }
            },
            OwnedMessage::Text(data) => {
                if data.len() < 10000 {
                    log_info!("Text {:?}", data);
                } else {
                    log_warn!(
                        "Text was too long! -> stored in debug{}.json",
                        debug_counter
                    );
                    let filename = format!("debug{}.json", debug_counter);
                    let mut file = File::create(filename).unwrap();
                    let _ = file.write_all(data.as_bytes());
                    debug_counter += 1;
                }

                let crawl_input: Value = serde_json::from_str(data.as_str()).unwrap();
                let crawl_msgs: &Value = &crawl_input["msgs"];

                for crawl_msg in crawl_msgs.as_array().unwrap() {
                    let _ = sender.send(Instruction::CrawlInput(crawl_msg.to_string()));
                }
            }
            _ => {
                log_warn!("Unknown message.");
            }
        }
    }

    log_debug!("Exiting loop_wsrecv...");
}

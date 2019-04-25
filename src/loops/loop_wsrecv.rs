extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::Sender;

use std::fs::File;
use std::io::prelude::*;

use serde_json;

use websocket::receiver::Reader;
use websocket::OwnedMessage;

use super::super::*;
use super::internal_message::InternalMessage;

use super::super::model::CrawlBatch;

pub fn run_loop_wsrecv(mut ws_reader: Reader<TcpStream>, sender: Sender<InternalMessage>) {
    let mut debug_counter = 0;

    // Receive loop
    for message_result in ws_reader.incoming_messages() {
        let message = match message_result {
            Ok(m) => m,
            Err(e) => {
                log_error!("An error occured while reading a ws message: {:?}", e);
                let _ = sender.send(InternalMessage::Close);
                break;
            }
        };

        match message {
            OwnedMessage::Close(_) => {
                let _ = sender.send(InternalMessage::Close);
                break;
            }
            OwnedMessage::Ping(data) => match sender.send(InternalMessage::Ping(data)) {
                Ok(()) => (),
                Err(e) => {
                    log_error!("An error occured while sending a pong response: {:?}", e);
                    break;
                }
            },
            OwnedMessage::Text(data) => {
                let crawl_batch: CrawlBatch = serde_json::from_str(data.as_str()).unwrap();

                if data.len() < 10000 {
                    log_info!("Text {:?}", data);
                } else {
                    log_warn!("Text was too long! {:?}", crawl_batch);
                    let filename = format!("debug{}.json", debug_counter);
                    let mut file = File::create(filename).unwrap();
                    let _ = file.write_all(data.as_bytes());
                    debug_counter += 1;
                }

                
                for crawl_message in crawl_batch.msgs.into_iter() {
                    let _ = sender.send(InternalMessage::CrawlData(crawl_message));
                }

                // let _ = sender.send(InternalMessage::CrawlData(crawl_batch));
            },
            // Say what we received
            _ => log_debug!("loop_wsrecv message {:?}", message),
        }
    }

    log_info!("Exiting loop_wsrecv...");
}

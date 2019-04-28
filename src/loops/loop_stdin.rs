extern crate websocket;

use std::sync::mpsc::Sender;

use super::super::*;
use super::internal_message::InternalMessage;

fn print_help() {
    println!("-------------------------------------------------------------------------------");
    println!();
    println!();
    println!("CRAWLBOT");
    println!();
    println!("commands:");
    println!("/close          sends a close message to crawl, exits crawlbot");
    println!("/get_status     prints a status report");
    println!("/help           prints this help screen");
    println!("<return key>    pauses/unpauses crawlbot");
    println!();
    println!();
    println!("-------------------------------------------------------------------------------");
}

pub fn run_loop_stdin(sender_bot: Sender<InternalMessage>) {
    
    print_help();
    let mut pause = true;

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin.");
        let trimmed = input.trim();

        let message = match trimmed {
            "" => {
                if pause {
                    pause = false;
                    InternalMessage::Unpause
                } else {
                    pause = true;
                    InternalMessage::Pause
                }
            }
            "/get_status" => InternalMessage::GetStatus,
            "/idle" => InternalMessage::Idle,
            "/help" => {
                print_help();
                InternalMessage::Nothing
            }
            "/close" | "/c" => {
                let _ = sender_bot.send(InternalMessage::Close);
                break;
            }
            _ => InternalMessage::CrawlOutput(trimmed.to_string()),
        };

        match sender_bot.send(message) {
            Ok(()) => (),
            Err(e) => {
                log_error!("An error occured when trying to send a message: {:?}", e);
                break;
            }
        }
    }

    log_info!("Exiting loop_stdin...");
}

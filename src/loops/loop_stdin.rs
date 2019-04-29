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
    println!("/idle5          lets crawlbot idle for 5 tics");
    println!("/idle10         lets crawlbot idle for 10 tics");
    println!("/help           prints this help screen");
    println!("/start          starts a the game");
    println!("/u              un-pauses crawlbot");
    println!("<return key>    pauses crawlbot");
    println!();
    println!();
    println!("-------------------------------------------------------------------------------");
}

pub fn run_loop_stdin(sender_bot: Sender<InternalMessage>) {
    print_help();

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin.");
        let trimmed = input.trim();

        let message = match trimmed {
            "" => InternalMessage::Pause,
            "/abandon" => InternalMessage::Abandon,
            "/clear_routines" => InternalMessage::ClearRoutines,
            "/close" | "/c" => {
                let _ = sender_bot.send(InternalMessage::Close);
                break;
            }
            "/get_status" => InternalMessage::GetStatus,
            "/idle5" => InternalMessage::Idle5,
            "/idle10" => InternalMessage::Idle10,
            "/help" => {
                print_help();
                InternalMessage::Nothing
            }
            "/pick_mifi" => InternalMessage::PickMiFi,
            "/pick_trbe" => InternalMessage::PickTrBe,
            "/start" => InternalMessage::Start,
            "/u" => InternalMessage::Unpause,
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

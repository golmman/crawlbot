extern crate websocket;

use super::super::*;
use super::instruction::Instruction;
use std::sync::mpsc::Sender;

fn print_help() {
    println!("-------------------------------------------------------------------------------");
    println!();
    println!();
    println!("CRAWLBOT");
    println!();
    println!("commands:");
    println!("/abandon              abandons the current game");
    println!("/clear_routines       clears the routine queue");
    println!("/c or /close          sends a close message to crawl, exits crawlbot");
    println!("/s or /get_status     prints a status report");
    println!("/idle5                lets crawlbot idle for 5 tics");
    println!("/idle10               lets crawlbot idle for 10 tics");
    println!("/help                 prints this help screen");
    println!("/pick_mifi            when a new game was started: pick mifi");
    println!("/pick_trbe            when a new game was started: pick trbe");
    println!("/start                starts a the game");
    println!("/u or /unpause        un-pauses crawlbot");
    println!("<return key>          pauses crawlbot");
    println!();
    println!();
    println!("-------------------------------------------------------------------------------");
}

pub fn run_loop_stdin(sender_bot: Sender<Instruction>) {
    print_help();

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin.");
        let trimmed = input.trim();

        let message = match trimmed {
            "" => Instruction::Pause,
            "/abandon" => Instruction::Abandon,
            "/clear_routines" => Instruction::ClearRoutines,
            "/c" | "/close" => {
                let _ = sender_bot.send(Instruction::Close);
                break;
            }
            "/s" | "/get_status" => Instruction::GetStatus,
            "/idle5" => Instruction::Idle5,
            "/idle10" => Instruction::Idle10,
            "/help" => {
                print_help();
                Instruction::Nothing
            }
            "/main" => Instruction::Main,
            "/pick_mifi" => Instruction::PickMiFi,
            "/pick_trbe" => Instruction::PickTrBe,
            "/start" => Instruction::Start,
            "/u" | "/unpause" => Instruction::Unpause,
            _ => Instruction::CrawlOutput(trimmed.to_string()),
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

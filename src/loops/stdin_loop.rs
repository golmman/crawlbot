extern crate websocket;

use crate::model::instruction::Instruction;
use crate::{log_crawl, log_error, log_info, log_debug, stdin, LoopState};
use std::sync::mpsc::Sender;

fn print_help() {
    println!("-------------------------------------------------------------------------------");
    println!();
    println!();
    println!("CRAWLBOT");
    println!();
    println!("commands:");
    println!("/:keypad nuumber:     take one step in that direction");
    println!("/abandon              abandons the current game");
    println!("/clear_routines       clears the routine queue");
    println!("/exit");
    println!("or /q or /quit");
    println!("or /c or /close       sends a close message to crawl, exits crawlbot");
    println!("/explore              auto explore - equivalent to one 'o' press");
    println!("/fight                auto fight - equivalent to one 'tab' press");
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

pub struct StdinLoopState {
    history: Vec<String>,
    sender_bot: Sender<Instruction>,
}

impl StdinLoopState {
    pub fn new(sender_bot: Sender<Instruction>) -> Self {
        print_help();

        StdinLoopState {
            history: Vec::new(),
            sender_bot,
        }
    }
}

impl LoopState<String, String> for StdinLoopState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e)
    }

    fn run_loop(&mut self) -> Result<String, String> {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin.");
        let trimmed = input.trim();

        self.history.push(trimmed.to_string());

        let instruction = match trimmed {
            "" => Instruction::Pause,
            "\u{1b}[A" => { 
                log_info!("TEST!!!!!!!"); 
                Instruction::Nothing    
            },
            "/1" => Instruction::StepSouthWest,
            "/2" => Instruction::StepSouth,
            "/3" => Instruction::StepSouthEast,
            "/4" => Instruction::StepWest,
            "/5" => Instruction::StepNone,
            "/6" => Instruction::StepEast,
            "/7" => Instruction::StepNorthWest,
            "/8" => Instruction::StepNorth,
            "/9" => Instruction::StepNorthEast,
            "/abandon" => Instruction::Abandon,
            "/clear_routines" => Instruction::ClearRoutines,
            "/descend" => Instruction::Descend,
            "/exit" | "/q" | "/quit" | "/c" | "/close" => {
                let _ = self.sender_bot.send(Instruction::Close);
                return Err(String::from("Exiting stdin_loop..."));
            }
            "/explore" => Instruction::Explore,
            "/fight" => Instruction::Fight,
            "/get_status" | "/s" => Instruction::GetStatus,
            "/idle5" => Instruction::Idle5,
            "/idle10" => Instruction::Idle10,
            "/help" => {
                print_help();
                Instruction::Nothing
            }
            "/history" => {
                log_debug!("----- HISTORY -----");
                for command in self.history.iter().cloned() {
                    println!("{}", command);
                }
                log_debug!("--- END HISTORY ---");
                Instruction::Nothing
            }
            "/main" => Instruction::Main,
            "/pick_mifi" => Instruction::PickMiFi,
            "/pick_trbe" => Instruction::PickTrBe,
            "/start" => Instruction::Start,
            "/u" | "/unpause" => Instruction::Unpause,
            _ => Instruction::CrawlOutput(trimmed.to_string()),
        };

        match self.sender_bot.send(instruction) {
            Ok(()) => Ok(String::from("")),
            Err(e) => {
                log_error!("An error occured when trying to send a message: {:?}", e);
                Err(String::from("Exiting writer_loop..."))
            }
        }
    }

}

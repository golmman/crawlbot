extern crate websocket;

use crate::model::instruction::Instruction;
use crate::{log_crawl, log_debug, stdin, LoopState};
use std::sync::mpsc::Sender;

const KEYMAP_ARROW_UP: &str = "\u{1b}[A";

fn print_help() {
    println!(
        r#"
-------------------------------------------------------------------------------


CRAWLBOT

commands:
/:keypad number:     take one step in that direction
/abandon              abandons the current game
/clear_routines       clears the routine queue
/exit
or /q or /quit
or /c or /close       sends a close message to crawl, exits crawlbot
/explore              auto explore - equivalent to one 'o' press
/fight                auto fight - equivalent to one 'tab' press
/get_status or /s     prints a status report
/idle5                lets crawlbot idle for 5 tics
/idle10               lets crawlbot idle for 10 tics
/help                 prints this help screen
/history or /h        prints the stdin command history
/main                 pushes the main routine to the queue
/map or /m            prints the map
/pick_mifi            when a new game was started: pick mifi
/pick_trbe            when a new game was started: pick trbe
/start                starts a the game
/u or /unpause        un-pauses crawlbot
<return key>          pauses crawlbot


-------------------------------------------------------------------------------
    "#
    );
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

    fn match_instruction(&mut self, input: &str) -> Instruction {
        self.history.push(input.to_string());

        match input {
            "" => Instruction::Pause,
            KEYMAP_ARROW_UP => {
                self.history.pop();
                let x = self.history.pop().unwrap_or_else(|| String::from(""));
                self.match_instruction(&x)
            }
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
            "/exit" | "/q" | "/quit" | "/c" | "/close" => Instruction::Close,
            "/explore" => Instruction::Explore,
            "/fight" => Instruction::Fight,
            "/get_status" | "/s" => Instruction::GetStatus,
            "/idle5" => Instruction::Idle5,
            "/idle10" => Instruction::Idle10,
            "/help" => {
                print_help();
                Instruction::Nothing
            }
            "/history" | "/h" => {
                log_debug!("----- HISTORY -----");
                for command in self.history.iter().cloned() {
                    println!("{}", command);
                }
                log_debug!("--- END HISTORY ---");
                Instruction::Nothing
            }
            "/main" => Instruction::Main,
            "/map" | "/m" => Instruction::Map,
            "/pick_mifi" => Instruction::PickMiFi,
            "/pick_trbe" => Instruction::PickTrBe,
            "/start" => Instruction::Start,
            "/u" | "/unpause" => Instruction::Unpause,
            _ => Instruction::CrawlOutput(input.to_string()),
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
        let trimmed_input = input.trim();

        let instruction = self.match_instruction(trimmed_input);

        let _ = self.sender_bot.send(instruction.clone());

        match instruction {
            Instruction::Close => Err(String::from("Exiting stdin_loop...")),
            _ => Ok(String::from("")),
        }
    }

}

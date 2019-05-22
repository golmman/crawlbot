extern crate websocket;

use crate::model::game_state::GameState;
use crate::model::instruction::Instruction;
use crate::{log_crawl, log_debug, log_warn, LoopState};
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod game_state_updates;
mod instruction_handlers;
mod util;

#[derive(Debug)]
pub struct BotLoopState {
    receiver_stdin: Receiver<Instruction>,
    receiver_websocket: Receiver<Instruction>,
    sender_websocket: Sender<Instruction>,

    exit_loop: bool,
    game_state: GameState,
    primary_queue: VecDeque<Instruction>,
    secondary_queue: VecDeque<Instruction>,
}

impl BotLoopState {
    pub fn new(
        receiver_stdin: Receiver<Instruction>,
        receiver_websocket: Receiver<Instruction>,
        sender_websocket: Sender<Instruction>,
    ) -> Self {
        BotLoopState {
            receiver_stdin,
            receiver_websocket,
            sender_websocket,

            exit_loop: false,
            game_state: GameState::new(),
            primary_queue: VecDeque::new(),
            secondary_queue: VecDeque::new(),
        }
    }
}

impl LoopState<String, String> for BotLoopState {
    fn assess_iteration(&self, _t: String) {}
    fn assess_error(&self, e: String) {
        log_debug!("{}", e);
    }

    fn run_loop(&mut self) -> Result<String, String> {
        std::thread::sleep(std::time::Duration::from_millis(10));

        self.fill_primary_queue();

        match self.get_next_instruction() {
            Instruction::Abandon => self.abandon(),
            Instruction::ClearRoutines => self.clear_routines(),
            Instruction::Close => self.close(),
            Instruction::CrawlInput(crawl_message) => self.crawl_input(crawl_message),
            Instruction::CrawlOutput(data) => self.crawl_output(data),
            Instruction::GetStatus => self.get_status(),
            Instruction::Idle10 => self.idle10(),
            Instruction::Idle5 => self.idle5(),
            Instruction::IfThenElse(c, t, e) => self.if_then_else(c, t, e),
            Instruction::Main => self.main(),
            Instruction::Nothing => self.nothing(),
            Instruction::Pause => self.pause(),
            Instruction::PickMiFi => self.pick_mifi(),
            Instruction::PickTrBe => self.pick_trbe(),
            Instruction::Ping(data) => self.ping(data),
            Instruction::Script(evaluate) => self.script(evaluate),
            Instruction::Start => self.start(),
            Instruction::Unpause => self.unpause(),
            _ => {
                log_warn!("Unknown message.");
            }
        };

        if self.exit_loop {
            Err(String::from("Exiting loop_bot..."))
        } else {
            Ok(String::from(""))
        }
    }
}

use serde_json::Value;

use crate::model::GameState;
use std::collections::VecDeque;

pub type Routine = VecDeque<Instruction>;

#[derive(Clone, Debug)]
pub enum Instruction {
    // single instructions
    ClearRoutines,
    Close,
    GetStatus,
    Nothing,
    Pause,
    Unpause,

    // sequential instructions, which construct routines
    Abandon,
    Idle10,
    Idle5,
    Main,
    PickMiFi,
    PickTrBe,
    Start,
    IfThenElse(fn(GameState) -> bool, fn() -> Routine, fn() -> Routine),
    Script(fn(GameState) -> Routine),

    // control instructions
    CrawlInput(Value),
    CrawlOutput(String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

impl Instruction {
    pub fn is_something(&self) -> bool {
        match *self {
            Instruction::Nothing => false,
            _ => true,
        }
    }
}

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use serde_json::Value;

use super::super::model::GameState;

pub type Routine = Vec<Instruction>;

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
    Script(fn(GameState) -> fn() -> Routine),
    
    // control instructions
    CrawlInput(Value),
    CrawlOutput(String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Instruction")
    }
}

impl Instruction {
    pub fn is_something(&self) -> bool {
        match *self {
            Instruction::Nothing => false,
            _ => true,
        }
    }
}

use crate::model::cws::msg::CwsMsg;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Result;

use crate::model::game_state::GameState;
use std::collections::VecDeque;

pub type Routine = VecDeque<Instruction>;

#[derive(Clone)]
pub struct CrawlScript {
    script: fn(&GameState) -> Routine,
}

impl CrawlScript {
    pub fn new(script: fn(&GameState) -> Routine) -> Self {
        Self {
            script,
        }
    }

    pub fn evaluate(&self, game_state: &GameState) -> Routine {
        (self.script)(game_state)
    }
}

impl Debug for CrawlScript {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "CrawlScript")
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    // single instructions
    ClearRoutines,
    Close,
    GetStatus,
    Map,
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
    Script(CrawlScript),
    
    StepSouthWest,
    StepSouth,
    StepSouthEast,
    StepWest,
    StepNone,
    StepEast,
    StepNorthWest,
    StepNorth,
    StepNorthEast,
    Fight,
    Explore,
    Descend,

    // control instructions
    CrawlMessage(CwsMsg),
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

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use super::super::model::GameState;

pub type Routine = Vec<InternalMessage>;

#[derive(Clone, Debug)]
pub enum InternalMessage {
    // basic messages
    ClearRoutines,
    Close,
    GetStatus,
    Nothing,
    Pause,
    Unpause,

    // combined messages (routines)
    Abandon,
    Idle10,
    Idle5,
    PickMiFi,
    PickTrBe,
    Start,
    IfThenElse(fn(GameState) -> bool, fn() -> Routine, fn() -> Routine),

    // control messages
    CrawlInput(String),
    CrawlOutput(String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

impl Display for InternalMessage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "InternalMessage")
    }
}

impl InternalMessage {
    pub fn is_something(&self) -> bool {
        match *self {
            InternalMessage::Nothing => false,
            _ => true,
        }
    }
}

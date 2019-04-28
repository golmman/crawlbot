use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Debug)]
pub enum InternalMessage {
    Close,
    GetStatus,
    Nothing,
    Pause,
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Unpause,

    Idle5,
    Idle10,
    Start,

    CrawlInput(String),
    CrawlOutput(String),
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

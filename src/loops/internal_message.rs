use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use super::super::model::CrawlMessage;

#[derive(Clone, Debug)]
pub enum InternalMessage {
    Close,
    GetStatus,
    Nothing,
    Pause,
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Unpause,

    Idle,

    Proxy(String),
    CrawlData(CrawlMessage),
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

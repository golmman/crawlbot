use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrawlInputMsgsMessage {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrawlInputMsgs {
    pub msg: String,
    pub messages: Vec<CrawlInputMsgsMessage>,
}

use crate::model::cws::message::Message;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Root {
    pub msgs: Option<Vec<Message>>,
}

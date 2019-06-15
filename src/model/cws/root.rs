use crate::model::cws::msg::CwsMsg;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CwsRoot {
    pub msgs: Option<Vec<CwsMsg>>,
}

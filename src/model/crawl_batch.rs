use serde::{Serialize, Deserialize};

use super::CrawlMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct CrawlBatch {
    pub msgs: Vec<CrawlMessage>,
}

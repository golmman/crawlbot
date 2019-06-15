use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CwsMessage {
    pub text: Option<String>,
}

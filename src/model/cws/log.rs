use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Log {
    pub text: Option<String>,
}

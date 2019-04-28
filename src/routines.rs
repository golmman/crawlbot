use crate::loops::internal_message::InternalMessage;
use std::collections::VecDeque;

pub fn create_routine_idle5() -> Vec<InternalMessage> {
    vec![
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
    ]
}

pub fn create_routine_idle10() -> Vec<InternalMessage> {
    vec![
        InternalMessage::Idle5,
        InternalMessage::Idle5,
    ]
}

pub fn create_routine_start() -> Vec<InternalMessage> {
    vec![
        InternalMessage::CrawlOutput(String::from(
            r#"{"msg":"login","username":"crawlbot","password":"123"}"#,
        )),
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::CrawlOutput(String::from(r#"{"msg":"play","game_id":"dcss-web-trunk"}"#)),
        InternalMessage::Pause,
    ]
}

pub fn push_routine(queue: &mut VecDeque<InternalMessage>, routine: fn() -> Vec<InternalMessage>) {
    let rou = routine();
    for r in rou {
        queue.push_back(r.clone());
    }
}

use crate::loops::internal_message::InternalMessage;
use std::collections::VecDeque;

fn cr_out(s: &str) -> InternalMessage {
    InternalMessage::CrawlOutput(String::from(s))
}

pub fn create_routine_abandon() -> Vec<InternalMessage> {
    vec![
        cr_out(r#"{"msg":"key","keycode":17}"#),
        cr_out(r#"{"msg":"input","text":"yes\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
    ]
}

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
    vec![InternalMessage::Idle5, InternalMessage::Idle5]
}

pub fn create_routine_pick_mifi() -> Vec<InternalMessage> {
    vec![
        cr_out(r#"{"msg": "input","text": "b"}"#),
        cr_out(r#"{"msg": "input","text": "a"}"#),
        cr_out(r#"{"msg": "input","text": "c"}"#),
    ]
}

pub fn create_routine_pick_trbe() -> Vec<InternalMessage> {
    vec![
        cr_out(r#"{"msg": "input","text": "g"}"#),
        cr_out(r#"{"msg": "input","text": "h"}"#),
        cr_out(r#"{"msg": "input","text": "f"}"#),
    ]
}

pub fn create_routine_start() -> Vec<InternalMessage> {
    vec![
        cr_out(r#"{"msg":"login","username":"crawlbot","password":"123"}"#),
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        cr_out(r#"{"msg":"play","game_id":"dcss-web-trunk"}"#),
        InternalMessage::Pause,
    ]
}

pub fn push_routine(queue: &mut VecDeque<InternalMessage>, routine: fn() -> Vec<InternalMessage>) {
    let rou = routine();
    for r in rou {
        queue.push_back(r.clone());
    }
}

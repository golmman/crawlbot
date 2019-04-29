use crate::loops::internal_message::InternalMessage;
use crate::loops::internal_message::Routine;
use std::collections::VecDeque;

use crate::model::GameState;

fn cr_out(s: &str) -> InternalMessage {
    InternalMessage::CrawlOutput(String::from(s))
}

pub fn create_routine_ifthenelse_test() -> Routine {
    vec![
        InternalMessage::IfThenElse(GameState::get_paused, create_routine_idle5, create_routine_idle5),
    ]
}

pub fn create_routine_abandon() -> Routine {
    vec![
        cr_out(r#"{"msg":"key","keycode":17}"#),
        cr_out(r#"{"msg":"input","text":"yes\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
    ]
}

pub fn create_routine_idle5() -> Routine {
    vec![
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
        InternalMessage::Nothing,
    ]
}

pub fn create_routine_idle10() -> Routine {
    vec![InternalMessage::Idle5, InternalMessage::Idle5]
}

pub fn create_routine_pick_mifi() -> Routine {
    vec![
        cr_out(r#"{"msg": "input","text": "b"}"#),
        cr_out(r#"{"msg": "input","text": "a"}"#),
        cr_out(r#"{"msg": "input","text": "c"}"#),
    ]
}

pub fn create_routine_pick_trbe() -> Routine {
    vec![
        cr_out(r#"{"msg": "input","text": "g"}"#),
        cr_out(r#"{"msg": "input","text": "h"}"#),
        cr_out(r#"{"msg": "input","text": "f"}"#),
    ]
}

pub fn create_routine_start() -> Routine {
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

pub fn push_routine(queue: &mut VecDeque<InternalMessage>, routine: fn() -> Routine) {
    let rou = routine();
    for r in rou {
        queue.push_back(r.clone());
    }
}

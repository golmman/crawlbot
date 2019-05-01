use crate::loops::internal_message::Instruction;
use crate::loops::internal_message::Routine;
use crate::model::GameState;
use std::collections::VecDeque;

fn cr_out(s: &str) -> Instruction {
    Instruction::CrawlOutput(String::from(s))
}

pub fn create_routine_main() -> Routine {
    vec![Instruction::IfThenElse(
        |g| g.get_enemy_number_in_sight() == 0,
        create_routine_explore,
        create_routine_fight,
    )]
}

pub fn create_routine_explore() -> Routine {
    vec![
        cr_out(r#"{"msg":"input","text":"o"}"#),
        Instruction::IfThenElse(
            |g| g.get_enemy_number_in_sight() == 0,
            create_routine_explore,
            create_routine_fight,
        ),
    ]
}

pub fn create_routine_fight() -> Routine {
    vec![
        cr_out(r#"{"msg":"key","keycode":9}"#),
        Instruction::IfThenElse(
            |g| g.get_enemy_number_in_sight() == 0,
            create_routine_explore,
            create_routine_fight,
        ),
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
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
    ]
}

pub fn create_routine_idle10() -> Routine {
    vec![Instruction::Idle5, Instruction::Idle5]
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
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        cr_out(r#"{"msg":"play","game_id":"dcss-web-trunk"}"#),
        Instruction::Pause,
    ]
}

pub fn push_routine(queue: &mut VecDeque<Instruction>, routine: fn() -> Routine) {
    let rou = routine();
    for r in rou {
        queue.push_back(r.clone());
        // queue.push_front(r.clone());
    }
}

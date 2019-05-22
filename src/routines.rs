use crate::model::instruction::Instruction;
use crate::model::instruction::Routine;
use crate::model::game_state::InputMode;
use std::collections::VecDeque;

fn cr_out(s: &str) -> Instruction {
    Instruction::CrawlOutput(String::from(s))
}

pub fn supply_routine_main() -> Routine {
    VecDeque::from(vec![Instruction::Script(|game_state| {
        let mut instruction_queue: Routine = VecDeque::new();

        match game_state.get_input_mode() {
            InputMode::Choose => {
                instruction_queue.push_back(cr_out(r#"{"msg": "input","text": "D"}"#));
                instruction_queue.append(&mut supply_routine_main());
                return instruction_queue;
            }
            InputMode::Game => {}
            InputMode::More => {
                instruction_queue.push_back(cr_out(r#"{"msg":"key","keycode":27}"#));
                instruction_queue.append(&mut supply_routine_main());
                return instruction_queue;
            }
            InputMode::Unknown => {}
            InputMode::Wait => {}
        }

        if game_state.get_enemy_number_in_sight() == 0 {
            // instruction_queue.append(&mut VecDeque::from(supply_routine_explore()));
            instruction_queue.push_back(cr_out(r#"{"msg":"input","text":"o"}"#));
        } else {
            // instruction_queue.append(&mut VecDeque::from(supply_routine_fight()));
            instruction_queue.push_back(cr_out(r#"{"msg":"key","keycode":9}"#));
        }

        instruction_queue.append(&mut supply_routine_main());
        instruction_queue
    })])
}

pub fn supply_routine_abandon() -> Routine {
    VecDeque::from(vec![
        cr_out(r#"{"msg":"key","keycode":17}"#),
        cr_out(r#"{"msg":"input","text":"yes\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
        cr_out(r#"{"msg":"input","text":"\r"}"#),
    ])
}

pub fn supply_routine_idle5() -> Routine {
    VecDeque::from(vec![
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
    ])
}

pub fn supply_routine_idle10() -> Routine {
    VecDeque::from(vec![Instruction::Idle5, Instruction::Idle5])
}

pub fn supply_routine_pick_mifi() -> Routine {
    VecDeque::from(vec![
        cr_out(r#"{"msg": "input","text": "b"}"#),
        cr_out(r#"{"msg": "input","text": "a"}"#),
        cr_out(r#"{"msg": "input","text": "c"}"#),
    ])
}

pub fn supply_routine_pick_trbe() -> Routine {
    VecDeque::from(vec![
        cr_out(r#"{"msg": "input","text": "g"}"#),
        cr_out(r#"{"msg": "input","text": "h"}"#),
        cr_out(r#"{"msg": "input","text": "f"}"#),
    ])
}

pub fn supply_routine_start() -> Routine {
    VecDeque::from(vec![
        cr_out(r#"{"msg":"login","username":"crawlbot","password":"123"}"#),
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        Instruction::Nothing,
        cr_out(r#"{"msg":"play","game_id":"dcss-web-trunk"}"#),
        Instruction::Pause,
    ])
}

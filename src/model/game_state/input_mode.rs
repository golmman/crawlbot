#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    // the game wants us to wait until another input_mode is sent
    Wait = 0,

    // manually attack: {"msg": "input","text": "1"}
    // manually attack: {"msg": "input","text": "2"}
    // manually attack: {"msg": "input","text": "4"}
    // manually attack: {"msg": "input","text": "5"}
    // manually attack: {"msg": "input","text": "6"}
    // manually attack: {"msg": "input","text": "7"}
    // manually attack: {"msg": "input","text": "8"}
    // manually attack: {"msg": "input","text": "9"}
    // manually attack: {"msg": "input","text": "s"}
    Game = 1,

    // there is 'more' the game wants to show us
    // -> react with the escape-key: {"msg":"key","keycode":27}
    More = 5,

    // the game wants us to choose from several options
    // -> react with a choice, e.g. {"msg": "input","text": "D"}
    Choose = 7,

    Unknown,
}

impl InputMode {
    pub fn from_i64(i: i64) -> Self {
        match i {
            0 => InputMode::Wait,
            1 => InputMode::Game,
            5 => InputMode::More,
            7 => InputMode::Choose,
            _ => InputMode::Unknown,
        }
    }
}

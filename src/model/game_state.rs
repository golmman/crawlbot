use serde::Deserialize;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Place {
    // branches
    Dungeon(u32),
    Temple,
    Lair(u32),
    Swamp(u32),
    Shoals(u32),
    Snake(u32),
    Slime(u32),
    Orc(u32),
    Elf(u32),
    Vaults(u32),
    Crypt(u32),
    Tomb(u32),
    Depths(u32),
    Hell,
    Cocytus(u32),
    Gehenna(u32),
    Tartarus(u32),
    Dis(u32),
    Abyss(u32),
    Pandemonium(u32),
    Zot(u32),

    // portals
    Bailey,
    Bazaar,
    Desolation,
    Ice,
    Gauntlet,
    Ossuary,
    Sewer,
    Treasure,
    Volcano,
    Wizlab,
    Ziggurat(u32),
}

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

#[derive(Deserialize, Debug, Clone)]
pub struct CwsMon {
    name: Option<String>,
    threat: Option<i64>,
}

impl CwsMon {
    pub fn from_value(value: &Value) -> Self {
        let s: Self = serde_json::from_value(value.clone()).expect("value to be defined");
        s
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    // control
    paused: bool,
    idle_ticks: u32,

    // crawl knowledge
    enemies_in_sight: Vec<CwsMon>,
    enemy_number_in_sight: u32,
    explored: bool,
    input_mode: InputMode,
    place: Place,
}

#[allow(dead_code)]
impl GameState {
    pub fn new() -> Self {
        GameState {
            paused: true,
            idle_ticks: 0,

            enemies_in_sight: Vec::new(),
            enemy_number_in_sight: 0,
            input_mode: InputMode::Wait,
            explored: false,
            place: Place::Dungeon(1),
        }
    }

    //
    pub fn pause(&mut self) {
        self.set_paused(true);
        self.set_idle_ticks(0);
    }

    pub fn unpause(&mut self) {
        self.set_paused(false);
        self.set_idle_ticks(0);
    }

    // getters and setters
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_explored(&self) -> bool {
        self.explored
    }

    pub fn set_explored(&mut self, explored: bool) {
        self.explored = explored;
    }

    pub fn get_place(&self) -> Place {
        self.place
    }

    pub fn set_place(&mut self, place: Place) {
        self.place = place;
    }

    pub fn get_idle_ticks(&self) -> u32 {
        self.idle_ticks
    }

    pub fn set_idle_ticks(&mut self, n: u32) {
        self.idle_ticks = n;
    }

    pub fn inc_idle_ticks(&mut self) {
        self.idle_ticks += 1;
    }

    //
    pub fn get_enemies_in_sight(&self) -> &Vec<CwsMon> {
        &self.enemies_in_sight
    }

    pub fn add_enemy_in_sight(&mut self, enemy: &CwsMon) -> &Vec<CwsMon> {
        self.enemies_in_sight.push(enemy.clone());
        &self.enemies_in_sight
    }

    pub fn clear_enemies_in_sight(&mut self) -> &Vec<CwsMon> {
        self.enemies_in_sight.clear();
        &self.enemies_in_sight
    }

    pub fn get_enemy_number_in_sight(&self) -> u32 {
        self.enemy_number_in_sight
    }

    pub fn set_enemy_number_in_sight(&mut self, n: u32) {
        self.enemy_number_in_sight = n;
    }

    pub fn dec_enemy_number_in_sight(&mut self) {
        self.enemy_number_in_sight -= 1;
    }

    pub fn inc_enemy_number_in_sight(&mut self) {
        self.enemy_number_in_sight += 1;
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn set_input_mode(&mut self, i: InputMode) {
        self.input_mode = i;
    }
}

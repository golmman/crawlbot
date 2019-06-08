use std::collections::HashMap;
use crate::model::cws::cell::Cell;
use crate::model::cws::monster::Monster;
use crate::model::game_state::input_mode::InputMode;
use crate::model::game_state::place::Place;

pub mod input_mode;
pub mod cell_cache;
pub mod place;

#[derive(Debug, Clone)]
pub struct GameState {
    // control
    paused: bool,
    idle_ticks: u32,

    // crawl knowledge
    cell_cache: HashMap<i64, Cell>,
    monsters_in_sight: Vec<Monster>,
    monster_number_in_sight: u32,
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

            cell_cache: HashMap::new(),
            monsters_in_sight: Vec::new(),
            monster_number_in_sight: 0,
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
    pub fn get_monsters_in_sight(&self) -> &Vec<Monster> {
        &self.monsters_in_sight
    }

    pub fn add_monster_in_sight(&mut self, monster: Monster) -> &Vec<Monster> {
        self.monsters_in_sight.push(monster.clone());
        &self.monsters_in_sight
    }

    pub fn clear_monsters_in_sight(&mut self) -> &Vec<Monster> {
        self.monsters_in_sight.clear();
        &self.monsters_in_sight
    }

    pub fn get_monster_number_in_sight(&self) -> u32 {
        self.monster_number_in_sight
    }

    pub fn set_monster_number_in_sight(&mut self, n: u32) {
        self.monster_number_in_sight = n;
    }

    pub fn dec_monster_number_in_sight(&mut self) {
        self.monster_number_in_sight -= 1;
    }

    pub fn inc_monster_number_in_sight(&mut self) {
        self.monster_number_in_sight += 1;
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn set_input_mode(&mut self, i: InputMode) {
        self.input_mode = i;
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Wait = 0,
    Game = 1,
    More = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    // control
    paused: bool,
    idle_ticks: u32,

    // crawl knowledge
    enemy_number_in_sight: u32,
    input_mode: InputMode,
    explored: bool,
    place: Place,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            paused: true,
            idle_ticks: 0,

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
    pub fn is_paused(self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_explored(self) -> bool {
        self.explored
    }

    pub fn set_explored(&mut self, explored: bool) {
        self.explored = explored;
    }

    pub fn get_place(self) -> Place {
        self.place
    }

    pub fn set_place(&mut self, place: Place) {
        self.place = place;
    }

    pub fn get_idle_ticks(self) -> u32 {
        self.idle_ticks
    }

    pub fn set_idle_ticks(&mut self, n: u32) {
        self.idle_ticks = n;
    }

    pub fn inc_idle_ticks(&mut self) {
        self.idle_ticks += 1;
    }

    //
    pub fn get_enemy_number_in_sight(self) -> u32 {
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

    pub fn get_input_mode(self) -> InputMode {
        self.input_mode
    }

    pub fn set_input_mode(&mut self, i: InputMode) {
        self.input_mode = i;
    }
}

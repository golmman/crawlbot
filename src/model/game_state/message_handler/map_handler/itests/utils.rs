use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_tile_index(x: i64, y: i64) -> usize {
    (x + y * GameState::MAP_WIDTH) as usize
}

pub fn prepare_cws_msg(file_name: &str) -> CwsMsg {
    let map_message_map_json = read_file(file_name);
    serde_json::from_str(&map_message_map_json).unwrap()
}

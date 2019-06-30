use crate::model::cws::root::CwsRoot;
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

    let root: CwsRoot = serde_json::from_str(&map_message_map_json).unwrap();

    let mut cws_msg_result = None;

    if let Some(msgs) = root.msgs {
        for message in msgs {
            if let Some(message_type) = &message.msg {
                if let "map" = message_type.as_str() {
                    cws_msg_result =  Some(message);
                    break;
                }
            }
        }
    }

    cws_msg_result.unwrap()
}

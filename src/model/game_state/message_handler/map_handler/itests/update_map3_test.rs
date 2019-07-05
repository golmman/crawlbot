use crate::model::game_state::monster::Monster;
use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

const GNOLL_ID1: i64 = 1;
const GNOLL_ID2: i64 = 2;
const GNOLL_ID3: i64 = 3;

fn assert_map0(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!(3, game_state.map.monsters_visible.len());

    assert_eq!(
        &Monster {
            id: GNOLL_ID1,
            name: String::from("gnoll"),
            threat: 2,
            tile_index: get_tile_index(35, 48) as i64,
        },
        game_state.map.monsters_visible.get(&GNOLL_ID1).unwrap()
    );
    assert_eq!(
        &Monster {
            id: GNOLL_ID2,
            name: String::from("gnoll"),
            threat: 2,
            tile_index: get_tile_index(33, 49) as i64,
        },
        game_state.map.monsters_visible.get(&GNOLL_ID2).unwrap()
    );
    assert_eq!(
        &Monster {
            id: GNOLL_ID3,
            name: String::from("gnoll"),
            threat: 2,
            tile_index: get_tile_index(33, 50) as i64,
        },
        game_state.map.monsters_visible.get(&GNOLL_ID3).unwrap()
    );
}

#[test]
fn update_map3() {
    let mut game_state = GameState::new();

    let map_message_map0: CwsMsg = prepare_cws_msg("tests/examples/maps3/root0.json");

    assert_map0(&mut game_state, map_message_map0);
}

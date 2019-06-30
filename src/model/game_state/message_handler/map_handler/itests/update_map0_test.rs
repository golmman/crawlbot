use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

const RAT_ID: i64 = 2;

fn assert_map0(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    // assert cell updates
    assert_eq!("@", game_state.map.tiles[get_tile_index(57, 53)].glyph);
    assert_eq!("r", game_state.map.tiles[get_tile_index(63, 50)].glyph);
    assert_eq!((57, 53), game_state.map.focus);

    // assert monster updates
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(63, 50) as i64,
        game_state.map.monsters_visible.get(&RAT_ID).unwrap().tile_index
    );
}

fn assert_map1(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    // assert cell updates
    assert_eq!("@", game_state.map.tiles[get_tile_index(57, 53)].glyph);
    assert_eq!("r", game_state.map.tiles[get_tile_index(62, 51)].glyph);
    assert_eq!((57, 53), game_state.map.focus);

    // assert monster updates
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(62, 51) as i64,
        game_state.map.monsters_visible.get(&RAT_ID).unwrap().tile_index
    );
}

fn assert_map2(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    // assert cell updates
    assert_eq!("@", game_state.map.tiles[get_tile_index(58, 52)].glyph);
    assert_eq!("r", game_state.map.tiles[get_tile_index(61, 52)].glyph);
    assert_eq!((57, 53), game_state.map.focus);

    // assert monster updates
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(61, 52) as i64,
        game_state.map.monsters_visible.get(&RAT_ID).unwrap().tile_index
    );
}

#[test]
fn update_map0() {
    let mut game_state = GameState::new();

    let map_message_map0: CwsMsg = prepare_cws_msg("tests/examples/maps0/root0.json");
    let map_message_map1: CwsMsg = prepare_cws_msg("tests/examples/maps0/root1.json");
    let map_message_map2: CwsMsg = prepare_cws_msg("tests/examples/maps0/root2.json");

    assert_map0(&mut game_state, map_message_map0);
    assert_map1(&mut game_state, map_message_map1);
    assert_map2(&mut game_state, map_message_map2);
}


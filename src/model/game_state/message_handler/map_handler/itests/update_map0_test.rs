use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

#[test]
fn update_map0() {
    let map_message_map0_json = read_file("tests/examples/maps0/map0.json");
    let map_message_map1_json = read_file("tests/examples/maps0/map1.json");
    let map_message_map2_json = read_file("tests/examples/maps0/map2.json");
    let map_message_map0: CwsMsg = serde_json::from_str(&map_message_map0_json).unwrap();
    let map_message_map1: CwsMsg = serde_json::from_str(&map_message_map1_json).unwrap();
    let map_message_map2: CwsMsg = serde_json::from_str(&map_message_map2_json).unwrap();

    let mut game_state = GameState::new();

    let rat_id = 2;

    /*
     * Update with map0 -> full map update
     */
    game_state.update_map(map_message_map0);

    // assert cell updates
    assert_eq!("@", game_state.map.tiles[get_tile_index(57, 53)].glyph);
    assert_eq!("r", game_state.map.tiles[get_tile_index(63, 50)].glyph);
    assert_eq!((57, 53), game_state.map.focus);

    // assert monster updates
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(63, 50) as i64,
        game_state.map.monsters_visible.get(&rat_id).unwrap().tile_index
    );

    /*
     * Update with map1 -> partial map update
     */
    game_state.update_map(map_message_map1);

    // assert cell updates
    assert_eq!("@", game_state.map.tiles[get_tile_index(57, 53)].glyph);
    assert_eq!("r", game_state.map.tiles[get_tile_index(62, 51)].glyph);
    assert_eq!((57, 53), game_state.map.focus);

    // assert monster updates
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(62, 51) as i64,
        game_state.map.monsters_visible.get(&rat_id).unwrap().tile_index
    );

    /*
     * Update with map2 -> partial map update
     */
    game_state.update_map(map_message_map2);

    // assert cell updates
    assert_eq!("@", game_state.map.tiles[get_tile_index(58, 52)].glyph);
    assert_eq!("r", game_state.map.tiles[get_tile_index(61, 52)].glyph);
    assert_eq!((57, 53), game_state.map.focus);

    // assert monster updates
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(61, 52) as i64,
        game_state.map.monsters_visible.get(&rat_id).unwrap().tile_index
    );
}


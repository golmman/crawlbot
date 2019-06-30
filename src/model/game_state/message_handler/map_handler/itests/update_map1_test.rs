use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

// TODO: prepare, execute, expect
#[allow(clippy::cognitive_complexity)]
#[test]
fn update_map1() {
    let map_message_map0_json = read_file("tests/examples/maps1/map0.json");
    let map_message_map1_json = read_file("tests/examples/maps1/map1.json");
    let map_message_map2_json = read_file("tests/examples/maps1/map2.json");
    let map_message_map3_json = read_file("tests/examples/maps1/map3.json");
    let map_message_map4_json = read_file("tests/examples/maps1/map4.json");
    let map_message_map5_json = read_file("tests/examples/maps1/map5.json");

    let map_message_map0: CwsMsg = serde_json::from_str(&map_message_map0_json).unwrap();
    let map_message_map1: CwsMsg = serde_json::from_str(&map_message_map1_json).unwrap();
    let map_message_map2: CwsMsg = serde_json::from_str(&map_message_map2_json).unwrap();
    let map_message_map3: CwsMsg = serde_json::from_str(&map_message_map3_json).unwrap();
    let map_message_map4: CwsMsg = serde_json::from_str(&map_message_map4_json).unwrap();
    let map_message_map5: CwsMsg = serde_json::from_str(&map_message_map5_json).unwrap();

    let mut game_state = GameState::new();

    let lizard_id = 1;
    let goblin_id = 2;

    game_state.update_map(map_message_map0);
    assert_eq!((20, 18), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(20, 18)].glyph);
    assert_eq!("l", game_state.map.tiles[get_tile_index(17, 18)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(17, 18) as i64,
        game_state.map.monsters_visible.get(&lizard_id).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(18, 16) as i64,
        game_state.map.monsters_visible.get(&goblin_id).unwrap().tile_index
    );

    game_state.update_map(map_message_map1);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 17)].glyph);
    assert_eq!("l", game_state.map.tiles[get_tile_index(18, 17)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(18, 17) as i64,
        game_state.map.monsters_visible.get(&lizard_id).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(18, 16) as i64,
        game_state.map.monsters_visible.get(&goblin_id).unwrap().tile_index
    );

    game_state.update_map(map_message_map2);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 17)].glyph);
    assert_eq!("l", game_state.map.tiles[get_tile_index(18, 17)].glyph);
    assert_eq!(".", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(18, 17) as i64,
        game_state.map.monsters_visible.get(&lizard_id).unwrap().tile_index
    );

    game_state.update_map(map_message_map3);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 17)].glyph);
    assert_eq!("†", game_state.map.tiles[get_tile_index(18, 17)].glyph);
    assert_eq!(0, game_state.map.monsters_visible.len());

    game_state.update_map(map_message_map4);
    assert_eq!("@", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(0, game_state.map.monsters_visible.len());

    game_state.update_map(map_message_map5);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 15)].glyph);
    assert_eq!(0, game_state.map.monsters_visible.len());
}

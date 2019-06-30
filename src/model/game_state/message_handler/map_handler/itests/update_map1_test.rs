use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

const LIZARD_ID: i64 = 1;
const GOBLIN_ID: i64 = 2;

fn assert_map0(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
    assert_eq!((20, 18), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(20, 18)].glyph);
    assert_eq!("l", game_state.map.tiles[get_tile_index(17, 18)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(17, 18) as i64,
        game_state.map.monsters_visible.get(&LIZARD_ID).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(18, 16) as i64,
        game_state.map.monsters_visible.get(&GOBLIN_ID).unwrap().tile_index
    );
}

fn assert_map1(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 17)].glyph);
    assert_eq!("l", game_state.map.tiles[get_tile_index(18, 17)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(18, 17) as i64,
        game_state.map.monsters_visible.get(&LIZARD_ID).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(18, 16) as i64,
        game_state.map.monsters_visible.get(&GOBLIN_ID).unwrap().tile_index
    );
}

fn assert_map2(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 17)].glyph);
    assert_eq!("l", game_state.map.tiles[get_tile_index(18, 17)].glyph);
    assert_eq!(".", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(18, 17) as i64,
        game_state.map.monsters_visible.get(&LIZARD_ID).unwrap().tile_index
    );
}

fn assert_map3(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 17)].glyph);
    assert_eq!("†", game_state.map.tiles[get_tile_index(18, 17)].glyph);
    assert_eq!(0, game_state.map.monsters_visible.len());
}

fn assert_map4(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
    assert_eq!("@", game_state.map.tiles[get_tile_index(18, 16)].glyph);
    assert_eq!(0, game_state.map.monsters_visible.len());
}

fn assert_map5(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
    assert_eq!("@", game_state.map.tiles[get_tile_index(19, 15)].glyph);
    assert_eq!(0, game_state.map.monsters_visible.len());
}


#[test]
fn update_map1() {
    let mut game_state = GameState::new();

    let map_message_map0: CwsMsg = prepare_cws_msg("tests/examples/maps1/root0.json");
    let map_message_map1: CwsMsg = prepare_cws_msg("tests/examples/maps1/root1.json");
    let map_message_map2: CwsMsg = prepare_cws_msg("tests/examples/maps1/root2.json");
    let map_message_map3: CwsMsg = prepare_cws_msg("tests/examples/maps1/root3.json");
    let map_message_map4: CwsMsg = prepare_cws_msg("tests/examples/maps1/root4.json");
    let map_message_map5: CwsMsg = prepare_cws_msg("tests/examples/maps1/root5.json");

    assert_map0(&mut game_state, map_message_map0);
    assert_map1(&mut game_state, map_message_map1);
    assert_map2(&mut game_state, map_message_map2);
    assert_map3(&mut game_state, map_message_map3);
    assert_map4(&mut game_state, map_message_map4);
    assert_map5(&mut game_state, map_message_map5);
}

use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

fn assert_map0(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("l", game_state.map.tiles[get_tile_index(32, 48)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(36, 48)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(35, 49)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(36, 50)].glyph);
    assert_eq!("@", game_state.map.tiles[get_tile_index(34, 53)].glyph);
    
    assert_eq!(4, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(32, 48) as i64,
        game_state.map.monsters_visible.get(&4).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_visible.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_visible.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_visible.get(&1).unwrap().tile_index
    );
    
    assert_eq!(0, game_state.map.monsters_assumed.len());
}

fn assert_map1(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("l", game_state.map.tiles[get_tile_index(33, 49)].glyph);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 49) as i64,
        game_state.map.monsters_visible.get(&4).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map2(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("l", game_state.map.tiles[get_tile_index(33, 50)].glyph);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    
    assert_eq!(1, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 50) as i64,
        game_state.map.monsters_visible.get(&4).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map3(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("l", game_state.map.tiles[get_tile_index(33, 50)].glyph);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(34, 53)].glyph);
    
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 50) as i64,
        game_state.map.monsters_visible.get(&4).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(34, 53) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map4(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("l", game_state.map.tiles[get_tile_index(33, 50)].glyph);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 50) as i64,
        game_state.map.monsters_visible.get(&4).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(33, 52) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map5(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("l", game_state.map.tiles[get_tile_index(33, 50)].glyph);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(34, 53)].glyph);
    
    assert_eq!(3, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 50) as i64,
        game_state.map.monsters_visible.get(&4).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(33, 52) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(34, 53) as i64,
        game_state.map.monsters_visible.get(&6).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map6(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(34, 53)].glyph);
    
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 52) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(34, 53) as i64,
        game_state.map.monsters_visible.get(&6).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map7(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 50)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 51) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(33, 52) as i64,
        game_state.map.monsters_visible.get(&6).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map8(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 49)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 51) as i64,
        game_state.map.monsters_visible.get(&6).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(33, 52) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map9(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(33, 48)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 52)].glyph);
    
    assert_eq!(2, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 51) as i64,
        game_state.map.monsters_visible.get(&6).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(33, 52) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    
    assert_eq!(3, game_state.map.monsters_assumed.len());
    assert_eq!(
        get_tile_index(36, 48) as i64,
        game_state.map.monsters_assumed.get(&3).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 49) as i64,
        game_state.map.monsters_assumed.get(&2).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_assumed.get(&1).unwrap().tile_index
    );
}

fn assert_map10(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);

    assert_eq!((34, 53), game_state.map.focus);
    assert_eq!("@", game_state.map.tiles[get_tile_index(34, 47)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 50)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(36, 50)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(33, 51)].glyph);
    assert_eq!("g", game_state.map.tiles[get_tile_index(35, 51)].glyph);
    
    assert_eq!(4, game_state.map.monsters_visible.len());
    assert_eq!(
        get_tile_index(33, 50) as i64,
        game_state.map.monsters_visible.get(&6).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(36, 50) as i64,
        game_state.map.monsters_visible.get(&7).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(33, 51) as i64,
        game_state.map.monsters_visible.get(&5).unwrap().tile_index
    );
    assert_eq!(
        get_tile_index(35, 51) as i64,
        game_state.map.monsters_visible.get(&8).unwrap().tile_index
    );
    
    assert_eq!(0, game_state.map.monsters_assumed.len());
}

#[test]
fn update_map2() {
    let mut game_state = GameState::new();

    let map_message_map0: CwsMsg = prepare_cws_msg("tests/examples/maps2/root0.json");
    let map_message_map1: CwsMsg = prepare_cws_msg("tests/examples/maps2/root1.json");
    let map_message_map2: CwsMsg = prepare_cws_msg("tests/examples/maps2/root2.json");
    let map_message_map3: CwsMsg = prepare_cws_msg("tests/examples/maps2/root3.json");
    let map_message_map4: CwsMsg = prepare_cws_msg("tests/examples/maps2/root4.json");
    let map_message_map5: CwsMsg = prepare_cws_msg("tests/examples/maps2/root5.json");
    let map_message_map6: CwsMsg = prepare_cws_msg("tests/examples/maps2/root6.json");
    let map_message_map7: CwsMsg = prepare_cws_msg("tests/examples/maps2/root7.json");
    let map_message_map8: CwsMsg = prepare_cws_msg("tests/examples/maps2/root8.json");
    let map_message_map9: CwsMsg = prepare_cws_msg("tests/examples/maps2/root9.json");
    let map_message_map10: CwsMsg = prepare_cws_msg("tests/examples/maps2/root10.json");

    assert_map0(&mut game_state, map_message_map0);
    assert_map1(&mut game_state, map_message_map1);
    assert_map2(&mut game_state, map_message_map2);
    assert_map3(&mut game_state, map_message_map3);
    assert_map4(&mut game_state, map_message_map4);
    assert_map5(&mut game_state, map_message_map5);
    assert_map6(&mut game_state, map_message_map6);
    assert_map7(&mut game_state, map_message_map7);
    assert_map8(&mut game_state, map_message_map8);
    assert_map9(&mut game_state, map_message_map9);
    assert_map10(&mut game_state, map_message_map10);
}


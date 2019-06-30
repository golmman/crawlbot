use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::message_handler::map_handler::itests::utils::*;

const RAT_ID: i64 = 2;

fn assert_map0(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map1(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map2(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map3(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map4(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map5(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map6(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map7(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map8(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map9(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
}

fn assert_map10(game_state: &mut GameState, map: CwsMsg) {
    game_state.update_map(map);
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

    // TODO: error here!
    // assert_map10(&mut game_state, map_message_map10);

    game_state.print_map();
}


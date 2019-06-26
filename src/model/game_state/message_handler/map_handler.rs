use crate::model::cws::cell::CwsCell;
use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::monster::Monster;
use crate::util::json_option::JsonOption;

impl GameState {
    pub fn update_map(&mut self, map_message: CwsMsg) {
        if let Some(cells) = map_message.cells {
            if cells.len() == Self::MAP_CELLS_COUNT as usize {
                self.update_map_tiles_full(cells);
            } else {
                self.update_map_tiles_partial(cells);
            }
        }
    }

    fn update_map_tiles_full(&mut self, cells: Vec<CwsCell>) {
        self.map.monsters_visible.clear();

        for (tile_index, cell) in cells.into_iter().enumerate() {
            if let Some(glyph) = cell.g {
                if glyph == "@" {
                    self.update_map_focus(tile_index as i64);
                }

                self.map.tiles[tile_index].glyph = glyph;
            }

            if let JsonOption::Some(mon) = cell.mon {
                self.map
                    .monsters_visible
                    .insert(mon.id.unwrap(), Monster::from(tile_index as i64, &mon));
            }
        }
    }

    fn update_map_tiles_partial(&mut self, cells: Vec<CwsCell>) {
        let mut tile_index = 0;
        let mut monster_vanish_ids: Vec<i64> = Vec::new();
        let mut monster_update_ids: Vec<i64> = Vec::new();

        for cell in cells {
            if let Some((cell_x, cell_y)) = cell.get_location() {
                let map_x = cell_x + self.map.focus.0;
                let map_y = cell_y + self.map.focus.1;
                tile_index = map_x + map_y * Self::MAP_WIDTH;
            } else {
                tile_index += 1;
            }

            if let Some(glyph) = cell.g {
                match glyph.as_str() {
                    "@" => {}
                    "†" | "." => { // TODO: any other tile-glyph below the corpse also qulifies...
                        for monster in self.map.monsters_visible.values() {
                            if tile_index == monster.tile_index {
                                monster_vanish_ids.push(monster.id);
                                break;
                            }
                        }
                    }
                    _ => {}
                }

                self.map.tiles[tile_index as usize].glyph = glyph;
            }

            if let JsonOption::Some(mon) = cell.mon {
                let monster_option = self.map.monsters_visible.get_mut(&mon.id.unwrap());
                if let Some(monster) = monster_option {
                    monster_update_ids.push(monster.id);
                    monster.update(tile_index, &mon);
                } else {
                    self.map
                        .monsters_visible
                        .insert(mon.id.unwrap(), Monster::from(tile_index as i64, &mon));
                }
            }
        }

        self.update_monster_deaths(&monster_update_ids, &monster_vanish_ids);
    }

    fn update_monster_deaths(&mut self, monster_update_ids: &[i64], monster_vanish_ids: &[i64]) {
        for vanished_id in monster_vanish_ids {
            let mut vanished_not_updated = true;
            for updated_id in monster_update_ids {
                if vanished_id == updated_id {
                    vanished_not_updated = false;
                }
            }

            if vanished_not_updated {
                self.map.monsters_visible.remove(&vanished_id);
            }
        }
    }

    fn update_map_focus(&mut self, index: i64) {
        self.map.focus = (index % Self::MAP_WIDTH, index / Self::MAP_WIDTH);
    }

    pub fn print_map(&self) {
        let mut x = 0;
        let mut y = 0;

        println!("  |0         1         2         3         4         5         6         7         ");
        println!("  |01234567890123456789012345678901234567890123456789012345678901234567890123456789");
        println!("--|--------------------------------------------------------------------------------");

        for tile in &self.map.tiles {
            if x == 0 {
                print!("{:02}|", y);
            }

            print!("{}", tile.glyph);
            x += 1;

            if x >= Self::MAP_WIDTH {
                println!("|");
                x = 0;
                y += 1;
            }
        }
        println!("  |--------------------------------------------------------------------------------|");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;


    fn read_file(file_name: &str) -> String {
        let mut file = File::open(file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[test]
    fn update_monster_deaths() {
        let mut game_state = GameState::new();
        game_state.map.monsters_visible.insert(
            1,
            Monster {
                id: 1,
                name: String::from("monster1"),
                threat: 0,
                tile_index: 0,
            },
        );
        game_state.map.monsters_visible.insert(
            2,
            Monster {
                id: 2,
                name: String::from("monster2"),
                threat: 0,
                tile_index: 0,
            },
        );
        game_state.map.monsters_visible.insert(
            3,
            Monster {
                id: 3,
                name: String::from("monster3"),
                threat: 0,
                tile_index: 0,
            },
        );
        game_state.map.monsters_visible.insert(
            4,
            Monster {
                id: 4,
                name: String::from("monster4"),
                threat: 0,
                tile_index: 0,
            },
        );

        let monster_vanish_ids = vec![1, 2, 3];
        let monster_update_ids = vec![2, 4];

        game_state.update_monster_deaths(&monster_update_ids, &monster_vanish_ids);

        let mv = &game_state.map.monsters_visible;
        assert_eq!(2, mv.len());
        assert!(mv.contains_key(&2));
        assert!(mv.contains_key(&4));
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn update_map2() {
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

    #[test]
    fn update_map1() {
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

    fn get_tile_index(x: i64, y: i64) -> usize {
        (x + y * GameState::MAP_WIDTH) as usize
    }
}

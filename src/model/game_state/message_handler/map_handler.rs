use crate::util::color_print::yellow;
use crate::util::color_print::cyan;
use crate::util::color_print::red;
use crate::util::color_print::blue;
use crate::model::cws::mon::CwsMon;
use crate::model::cws::cell::CwsCell;
use crate::model::cws::msg::CwsMsg;
use crate::model::game_state::GameState;
use crate::model::game_state::monster::Monster;
use crate::util::json_option::JsonOption;
use crate::{log_crawl, log_error};

#[cfg(test)]
mod itests;

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
            self.map.tiles[tile_index].update(&cell);
            
            if let Some(glyph) = cell.g {
                if glyph == "@" {
                    self.update_map_focus(tile_index as i64);
                }
            }

            if let JsonOption::Some(mon) = cell.mon {
                if let Some(id) = mon.id {
                    self.map
                        .monsters_visible
                        .insert(id, Monster::from(tile_index as i64, &mon));
                }
            }
        }
    }

    fn update_map_tiles_partial(&mut self, cells: Vec<CwsCell>) {
        let mut tile_index = 0;
        let mut monster_cells: Vec<(i64, JsonOption<CwsMon>)> = Vec::new();

        for cell in cells {
            if let Some((cell_x, cell_y)) = cell.get_location() {
                let map_x = cell_x + self.map.focus.0;
                let map_y = cell_y + self.map.focus.1;
                tile_index = map_x + map_y * Self::MAP_WIDTH;
            } else {
                tile_index += 1;
            }

            if let Some(glyph) = cell.g {
                self.map.tiles[tile_index as usize].glyph = glyph;
            }

            if cell.mon.is_defined() {
                monster_cells.push((tile_index, cell.mon));
            }
        }

        self.remove_visible_monsters_by_cellinfo(&monster_cells);
        self.upsert_visible_monsters_by_cellinfo(&monster_cells);
    }

    // TODO: reduce complexity
    fn remove_visible_monsters_by_cellinfo(&mut self, monster_cells: &[(i64, JsonOption<CwsMon>)]) {
        for (tile_index, mon) in monster_cells {
            if mon.is_null() {
                // find mon id in monsters_visible map
                let mut mon_id_option: Option<i64> = None;
                for monster_visible in self.map.monsters_visible.values() {
                    if monster_visible.tile_index == *tile_index {
                        mon_id_option = Some(monster_visible.id);
                    }
                }

                if let Some(mon_id) = mon_id_option {
                    let mut is_remove = true;

                    for (_, mon_option) in monster_cells {
                        if let JsonOption::Some(mon2) = mon_option {
                            if let Some(mon2_id) = mon2.id {
                                if mon2_id == mon_id {
                                    is_remove = false;
                                    break;
                                }
                            }
                        }
                    }

                    if is_remove {
                        // remove
                        self.map.monsters_visible.remove(&mon_id);
                    }
                } else {
                    log_error!("(tile_index, mon): ({:?}, {:?})", tile_index, mon);
                    log_error!("monsters_visible: {:?}", self.map.monsters_visible);
                    panic!("Tried to remove a monster which was not yet created.");
                }
            }
        }
    }

    fn upsert_visible_monsters_by_cellinfo(&mut self, monster_cells: &[(i64, JsonOption<CwsMon>)]) {
        for (tile_index, mon_option) in monster_cells {
            if let JsonOption::Some(mon) = mon_option {
                let mon_id = mon.id.unwrap();
                if let Some(monster_visible) = self.map.monsters_visible.get_mut(&mon_id) {
                    // update
                    monster_visible.update(*tile_index, &mon);
                } else {
                    // create
                    self.map
                        .monsters_visible
                        .insert(mon_id, Monster::from(*tile_index, &mon));
                }
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

            if tile.color == 8 {
                print!("{}", tile.glyph);
            } else if tile.color == 7 {
                print!("{}", red(&tile.glyph));
            } else if tile.color == 6 {
                print!("{}", yellow(&tile.glyph));
            } else {
                print!("{}", blue(&tile.glyph));
            }
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

    fn generate_game_state_with_monsters_visible() -> GameState {
        let mut game_state = GameState::new();
        
        game_state.map.monsters_visible.insert(
            1,
            Monster {
                id: 1,
                name: String::from("monster1"),
                threat: 0,
                tile_index: 10,
            },
        );
        game_state.map.monsters_visible.insert(
            2,
            Monster {
                id: 2,
                name: String::from("monster2"),
                threat: 0,
                tile_index: 20,
            },
        );
        game_state.map.monsters_visible.insert(
            3,
            Monster {
                id: 3,
                name: String::from("monster3"),
                threat: 0,
                tile_index: 30,
            },
        );
        game_state.map.monsters_visible.insert(
            4,
            Monster {
                id: 4,
                name: String::from("monster4"),
                threat: 0,
                tile_index: 40,
            },
        );

        game_state
    }

    fn generate_monster_cells() -> Vec<(i64, JsonOption<CwsMon>)> {
        let monster_cells: Vec<(i64, JsonOption<CwsMon>)> = vec![
            // monster1 moves, and should not be removed
            (10, JsonOption::Null),
            (
                20,
                JsonOption::Some(CwsMon {
                    id: Some(1),
                    name: Some(String::from("Stephen")),
                    threat: None,
                }),
            ),
            // monster2 vanishes and should be remove
            (20, JsonOption::Null),

            // monster3 vanishes and should be removed
            (30, JsonOption::Null),

            // monster5 is create
            (
                50,
                JsonOption::Some(CwsMon {
                    id: Some(5),
                    name: Some(String::from("monster5")),
                    threat: Some(2),
                }),
            ),
        ];

        monster_cells
    }

    #[test]
    fn remove_visible_monsters_by_cellinfo() {
        let mut game_state = generate_game_state_with_monsters_visible();

        let monster_cells_tmp = generate_monster_cells();

        game_state.remove_visible_monsters_by_cellinfo(&monster_cells_tmp);

        let mv = &game_state.map.monsters_visible;
        assert_eq!(2, mv.len());
        assert!(mv.contains_key(&1));
        assert!(mv.contains_key(&4));
    }

    #[test]
    fn upsert_visible_monsters_by_cellinfo() {
        let mut game_state = generate_game_state_with_monsters_visible();

        let monster_cells = generate_monster_cells();

        game_state.upsert_visible_monsters_by_cellinfo(&monster_cells);

        let mv = &game_state.map.monsters_visible;

        assert_eq!(5, mv.len());
        assert_eq!(
            Some(&Monster {
                id: 1,
                name: "Stephen".to_string(),
                threat: 0,
                tile_index: 20
            }),
            mv.get(&1)
        );
        assert_eq!(
            Some(&Monster {
                id: 5,
                name: "monster5".to_string(),
                threat: 2,
                tile_index: 50
            }),
            mv.get(&5)
        );
    }
}

use crate::model::cws::cell::Cell;
use crate::model::cws::message::Message;
use crate::model::game_state::GameState;

impl GameState {
    pub fn update_map(&mut self, map_message: Message) {
        if let Some(cells) = map_message.cells {
            if cells.len() == Self::MAP_CELLS_COUNT as usize {
                self.update_map_tiles_full(cells);
            } else {
                self.update_map_tiles_partial(cells);
            }
        }
    }

    fn update_map_tiles_full(&mut self, cells: Vec<Cell>) {
        for (index, cell) in cells.into_iter().enumerate() {
            if let Some(glyph) = cell.g {
                if glyph == "@" {
                    self.update_map_focus(index as i64);
                }

                self.map.tiles[index].glyph = glyph;
            }
        }
    }

    fn update_map_tiles_partial(&mut self, cells: Vec<Cell>) {
        let mut index = 0;
        let mut new_map_focus_index: Option<i64> = None;

        for cell in cells {
            if let Some((cell_x, cell_y)) = cell.get_location() {
                let map_x = cell_x + self.map.focus.0;
                let map_y = cell_y + self.map.focus.1;
                index = map_x + map_y * Self::MAP_WIDTH;
            } else {
                index += 1;
            }

            if let Some(glyph) = cell.g {
                if glyph == "@" {
                    new_map_focus_index = Some(index);
                }
                self.map.tiles[index as usize].glyph = glyph;
            }
        }

        if let Some(i) = new_map_focus_index {
            self.update_map_focus(i);
        }
    }

    fn update_map_tile_glyph(&mut self, index: i64, cell: Cell) {
        if let Some(glyph) = cell.g {
            if glyph == "@" {
                self.update_map_focus(index);
            }

            self.map.tiles[index as usize].glyph = glyph;
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
    use crate::model::cws::message::Message;
    use std::fs::File;
    use std::io::Read;
    use super::*;

    fn read_file(file_name: &str) -> String {
        let mut file = File::open(file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[test]
    fn update_map() {
        let map_message_map0_json = read_file("tests/examples/map/map0.json");
        let map_message_map1_json = read_file("tests/examples/map/map1.json");
        let map_message_map2_json = read_file("tests/examples/map/map2.json");

        let map_message_map0: Message = serde_json::from_str(&map_message_map0_json).unwrap();
        let map_message_map1: Message = serde_json::from_str(&map_message_map1_json).unwrap();
        let map_message_map2: Message = serde_json::from_str(&map_message_map2_json).unwrap();

        let mut game_state = GameState::new();

        let mut expected_map_focus_index: usize;
        let mut expected_rat_index: usize;

        game_state.update_map(map_message_map0);
        // game_state.print_map();
        expected_map_focus_index = (57 + 53 * GameState::MAP_WIDTH) as usize;
        expected_rat_index = (63 + 50 * GameState::MAP_WIDTH) as usize;
        assert_eq!("@", game_state.map.tiles[expected_map_focus_index].glyph);
        assert_eq!("r", game_state.map.tiles[expected_rat_index].glyph);
        assert_eq!((57, 53), game_state.map.focus);

        game_state.update_map(map_message_map1);
        // game_state.print_map();
        expected_map_focus_index = (57 + 53 * GameState::MAP_WIDTH) as usize;
        expected_rat_index = (62 + 51 * GameState::MAP_WIDTH) as usize;
        assert_eq!("@", game_state.map.tiles[expected_map_focus_index].glyph);
        assert_eq!("r", game_state.map.tiles[expected_rat_index].glyph);
        assert_eq!((57, 53), game_state.map.focus);

        game_state.update_map(map_message_map2);
        // game_state.print_map();
        expected_map_focus_index = (58 + 52 * GameState::MAP_WIDTH) as usize;
        expected_rat_index = (61 + 52 * GameState::MAP_WIDTH) as usize;
        assert_eq!("@", game_state.map.tiles[expected_map_focus_index].glyph);
        assert_eq!("r", game_state.map.tiles[expected_rat_index].glyph);
        assert_eq!((58, 52), game_state.map.focus);
    }
}

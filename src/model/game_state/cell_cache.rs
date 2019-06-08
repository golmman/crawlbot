use crate::model::cws::cell::Cell;
use crate::model::cws::util::Upgradable;
use crate::model::game_state::GameState;

impl GameState {
    pub fn update_cell_cache(&mut self, check_cell: Cell) {
        let mut check_cell_backup = check_cell.clone();
        // https://doc.rust-lang.org/reference/patterns.html#struct-patterns
        if let Some(mon) = check_cell.mon {
            if let Some(id) = mon.id {
                if let Some(hashed_cell) = self.cell_cache.get_mut(&id) {
                    hashed_cell.upgrade2(&check_cell_backup);
                }
            }
        }
    }
}

use crate::model::cws::cell::CwsCell;
use crate::model::cws::util::Upgradable;
use crate::model::game_state::GameState;

impl GameState {
    pub fn update_cell_cache(&mut self, check_cell: CwsCell) {
        let check_cell_backup = check_cell.clone();
        
        if let Some(mon) = check_cell.mon {
            if let Some(id) = mon.id {
                if let Some(hashed_cell) = self.cell_cache.get_mut(&id) {
                    // update monster cells
                    hashed_cell.upgrade2(&check_cell_backup);
                } else {
                    // create monster cells
                    self.cell_cache.insert(id, check_cell_backup);
                }
            }
        }

        // remove dead monster cells
    }
}

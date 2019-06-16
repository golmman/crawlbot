use crate::model::cws::mon::CwsMon;

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: i64,
    pub name: String,
    pub threat: i64,
    pub tile_index: i64,
}

impl Monster {
    pub fn from(tile_index: i64, mon: &CwsMon) -> Self {
        Self {
            id: mon.id.unwrap(),
            name: mon.name.clone().unwrap(),
            threat: mon.threat.unwrap(),
            tile_index,
        }
    }

    pub fn update(&mut self, tile_index: i64, mon: &CwsMon) {
        if let Some(id) = &mon.id {
            self.id = *id;
        }

        if let Some(name) = &mon.name {
            self.name = name.clone();
        }

        if let Some(threat) = &mon.threat {
            self.threat = *threat;
        }

        self.tile_index = tile_index;
    }
}

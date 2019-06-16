use crate::model::cws::mon::CwsMon;

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: i64,
    pub name: String,
    pub threat: i64,
}

impl Monster {
    pub fn from_cws_mon(mon: &CwsMon) -> Self {
        Self {
            id: mon.id.unwrap(),
            name: mon.name.clone().unwrap(),
            threat: mon.threat.unwrap(),
        }
    }
}

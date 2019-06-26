use crate::model::cws::mon::CwsMon;
use crate::model::cws::util::Upgradable;
use crate::model::cws::util::upgrade_primitive;
use crate::model::cws::util::upgrade_struct;
use crate::util::json_option::JsonOption;
use serde::Deserialize;

#[serde(default)]
#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
pub struct CwsCell {
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub g: Option<String>,
    pub mon: Option<CwsMon>,
    pub mon2: JsonOption<CwsMon>,
}

impl CwsCell {
    pub fn get_location(&self) -> Option<(i64, i64)> {
        if let Some(cell_x) = self.x {
            if let Some(cell_y) = self.y {
                return Some((cell_x, cell_y));
            }
        }

        None
    }
}

impl Upgradable<CwsCell> for CwsCell {
    // TODO: remove?
    fn upgrade(self, other: CwsCell) -> CwsCell {
        CwsCell {
            x: upgrade_primitive(self.x, other.x),
            y: upgrade_primitive(self.y, other.y),
            g: upgrade_primitive(self.g, other.g),
            mon: upgrade_struct(self.mon, other.mon),
            mon2: JsonOption::Undefined,
        }
    }

    fn upgrade2(&mut self, other: &CwsCell) {
        self.x = upgrade_primitive(self.x, other.x);
        self.y = upgrade_primitive(self.y, other.y);
        self.g = upgrade_primitive(self.g.clone(), other.g.clone());
        self.mon = upgrade_struct(self.mon.clone(), other.mon.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_cell() {
        let json = String::from(
            r#"
            {
                "x": -4,
                "y": 10,
                "mon": {
                    "id": 1,
                    "name": "Crazy Yiuf",
                    "threat": 2
                }
            }
            "#,
        );

        let cell: CwsCell = serde_json::from_str(&json).unwrap();

        let mon = cell.mon.unwrap();
        assert_eq!(cell.x.unwrap(), -4);
        assert_eq!(cell.y.unwrap(), 10);
        assert_eq!(mon.id.unwrap(), 1);
        assert_eq!(mon.name.unwrap(), "Crazy Yiuf");
        assert_eq!(mon.threat.unwrap(), 2);
    }
}

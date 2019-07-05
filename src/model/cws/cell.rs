use crate::model::cws::mon::CwsMon;
use crate::util::json_option::JsonOption;
use serde::Deserialize;

#[serde(default)]
#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
pub struct CwsCell {
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub g: Option<String>,
    pub col: Option<i64>,
    pub mon: JsonOption<CwsMon>,
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

        if let JsonOption::Some(mon) = cell.mon {
            assert_eq!(cell.x.unwrap(), -4);
            assert_eq!(cell.y.unwrap(), 10);
            assert_eq!(mon.id.unwrap(), 1);
            assert_eq!(mon.name.unwrap(), "Crazy Yiuf");
            assert_eq!(mon.threat.unwrap(), 2);
        } else {
            panic!();
        }
    }
}

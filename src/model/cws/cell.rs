use crate::model::cws::monster::Monster;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Cell {
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub mon: Option<Monster>,
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
            "#
        );

        let cell: Cell = serde_json::from_str(&json).unwrap();

        let mon = cell.mon.unwrap();
        assert_eq!(cell.x.unwrap(), -4);
        assert_eq!(cell.y.unwrap(), 10);
        assert_eq!(mon.id.unwrap(), 1);
        assert_eq!(mon.name.unwrap(), "Crazy Yiuf");
        assert_eq!(mon.threat.unwrap(), 2);
    }
}

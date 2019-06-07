use crate::model::cws::cell::Cell;
use crate::model::cws::log::Log;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub msg: Option<String>,

    pub cells: Option<Vec<Cell>>,
    pub messages: Option<Vec<Log>>,
    pub mode: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cws::monster::Monster;

    #[test]
    fn deserialize_message() {
       let json = String::from(
            r#"
            {
                "msg": "map",
                "cells": [
                    {
                        "x": 1,
                        "test": 1,
                        "mon": {
                            "name": "Crazy Yiuf"
                        }
                    },
                    {
                        "x": 1,
                        "mon": {
                            "name": "Crazy Yiuf",
                            "nonsense": "123"
                        }
                    }
                ]
            }
            "#,
        );

        let message: Message = serde_json::from_str(&json).unwrap();

        assert_message(message);
    }

    fn assert_message(message: Message) {
        if let Some(msg) = message.msg {
            assert_eq!(msg, "map");
        } else {
            panic!();
        }

        if let Some(cells) = message.cells {
            for cell in cells {
                assert_cell(cell);
            }
        } else {
            panic!();
        }
    }

    fn assert_cell(cell: Cell) {
        if let Some(mon) = cell.mon {
            assert_mon(mon);
        } else {
            panic!();
        }
    }

    fn assert_mon(mon: Monster) {
        if let Some(name) = mon.name {
            assert_eq!(name, "Crazy Yiuf")
        } else {
            panic!();
        }
    }
}

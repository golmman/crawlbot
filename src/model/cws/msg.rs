use crate::model::cws::cell::CwsCell;
use crate::model::cws::message::CwsMessage;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CwsMsg {
    pub msg: Option<String>,

    pub cells: Option<Vec<CwsCell>>,
    pub messages: Option<Vec<CwsMessage>>,
    pub mode: Option<i64>,
}

#[cfg(test)]
mod tests {
    use crate::model::cws::mon::CwsMon;
    use crate::util::json_option::JsonOption;
    use super::*;

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

        let message: CwsMsg = serde_json::from_str(&json).unwrap();

        assert_message(message);
    }

    fn assert_message(message: CwsMsg) {
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

    fn assert_cell(cell: CwsCell) {
        if let JsonOption::Some(mon) = cell.mon {
            assert_mon(mon);
        } else {
            panic!();
        }
    }

    fn assert_mon(mon: CwsMon) {
        if let Some(name) = mon.name {
            assert_eq!(name, "Crazy Yiuf")
        } else {
            panic!();
        }
    }
}

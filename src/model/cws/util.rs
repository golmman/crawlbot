pub trait Upgradable<T> {
    fn upgrade(self, other: T) -> T;
    fn upgrade2(&mut self, other: &T);
}

pub fn upgrade_primitive<T>(self_field: Option<T>, other_field: Option<T>) -> Option<T> {
    if other_field.is_some() {
        other_field
    } else {
        self_field
    }
}

pub fn upgrade_struct<T: Upgradable<T>>(self_option: Option<T>, other_option: Option<T>) -> Option<T> {
    if let Some(other) = other_option {
        if let Some(self1) = self_option {
            Some(self1.upgrade(other))
        } else {
            Some(other)
        }
    } else {
        self_option
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cws::monster::Monster;
    use super::*;
    use crate::model::cws::cell::Cell;

    #[test]
    fn upgrade2_from_empty() {
        let json_before = String::from(
            r#"
            {}
            "#,
        );

        let json_increment = String::from(
            r#"
            {
                "x": -4,
                "y": 10,
                "mon": {
                    "id": 1,
                    "threat": 2
                }
            }
            "#,
        );

        let mut cell_before: Cell = serde_json::from_str(&json_before).unwrap();
        let cell_increment: Cell = serde_json::from_str(&json_increment).unwrap();

        cell_before.upgrade2(&cell_increment);
        assert_eq!(
            cell_before,
            Cell {
                x: Some(-4),
                y: Some(10),
                mon: Some(Monster {
                    id: Some(1),
                    name: None,
                    threat: Some(2),
                })
            }
        );
    }

    #[test]
    fn upgrade_from_empty() {
        let json_before = String::from(
            r#"
            {}
            "#,
        );

        let json_increment = String::from(
            r#"
            {
                "x": -4,
                "y": 10,
                "mon": {
                    "id": 1,
                    "threat": 2
                }
            }
            "#,
        );

        let cell_before: Cell = serde_json::from_str(&json_before).unwrap();
        let cell_increment: Cell = serde_json::from_str(&json_increment).unwrap();

        let cell_upgraded = cell_before.upgrade(cell_increment);
        assert_eq!(
            cell_upgraded,
            Cell {
                x: Some(-4),
                y: Some(10),
                mon: Some(Monster {
                    id: Some(1),
                    name: None,
                    threat: Some(2),
                })
            }
        );
    }

    #[test]
    fn upgrade_only_changes() {
        let json_before = String::from(
            r#"
            {
                "x": -1,
                "y": 2,
                "mon": {
                    "id": 1,
                    "name": "before",
                    "threat": 2
                }
            }
            "#,
        );

        let json_increment = String::from(
            r#"
            {
                "x": -11,
                "y": 22,
                "mon": {
                    "id": 10,
                    "name": "upgraded",
                    "threat": 20
                }
            }
            "#,
        );

        let cell_before: Cell = serde_json::from_str(&json_before).unwrap();
        let cell_increment: Cell = serde_json::from_str(&json_increment).unwrap();

        let cell_upgraded = cell_before.upgrade(cell_increment);
        assert_eq!(
            cell_upgraded,
            Cell {
                x: Some(-11),
                y: Some(22),
                mon: Some(Monster {
                    id: Some(10),
                    name: Some(String::from("upgraded")),
                    threat: Some(20),
                })
            }
        );
    }

    #[test]
    fn upgrade_only_new_fields() {
        let json_before = String::from(
            r#"
            {
                "x": -1,
                "mon": {
                    "id": 1,
                    "threat": 2
                }
            }
            "#,
        );

        let json_increment = String::from(
            r#"
            {
                "y": 22,
                "mon": {
                    "name": "upgraded"
                }
            }
            "#,
        );

        let cell_before: Cell = serde_json::from_str(&json_before).unwrap();
        let cell_increment: Cell = serde_json::from_str(&json_increment).unwrap();

        let cell_upgraded = cell_before.upgrade(cell_increment);
        assert_eq!(
            cell_upgraded,
            Cell {
                x: Some(-1),
                y: Some(22),
                mon: Some(Monster {
                    id: Some(1),
                    name: Some(String::from("upgraded")),
                    threat: Some(2),
                })
            }
        );
    }

    #[test]
    fn upgrade_never_decrement() {
        let json_before = String::from(
            r#"
            {
                "x": -1,
                "y": 2,
                "mon": {
                    "id": 1,
                    "name": "before",
                    "threat": 2
                }
            }
            "#,
        );

        let json_increment = String::from(
            r#"
            {}
            "#,
        );

        let cell_before: Cell = serde_json::from_str(&json_before).unwrap();
        let cell_increment: Cell = serde_json::from_str(&json_increment).unwrap();

        let cell_upgraded = cell_before.upgrade(cell_increment);
        assert_eq!(
            cell_upgraded,
            Cell {
                x: Some(-1),
                y: Some(2),
                mon: Some(Monster {
                    id: Some(1),
                    name: Some(String::from("before")),
                    threat: Some(2),
                })
            }
        );
    }
}

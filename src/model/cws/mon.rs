use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CwsMon {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub threat: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_monster() {
        let json = String::from(
            r#"
            {
                "id": 1,
                "name": "Crazy Yiuf",
                "threat": 2
            }
            "#,
        );

        let monster: CwsMon = serde_json::from_str(&json).unwrap();

        assert_eq!(monster.id.unwrap(), 1);
        assert_eq!(monster.name.unwrap(), "Crazy Yiuf");
        assert_eq!(monster.threat.unwrap(), 2);
    }
}

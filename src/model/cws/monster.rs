use serde::Deserialize;
use crate::model::cws::util::upgrade_primitive;
use crate::model::cws::util::Upgradable;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Monster {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub threat: Option<i64>,
}

impl Upgradable<Monster> for Monster {
    fn upgrade(self, other: Monster) -> Monster {
        Monster {
            id: upgrade_primitive(self.id, other.id),
            name: upgrade_primitive(self.name, other.name),
            threat: upgrade_primitive(self.threat, other.threat),
        }
    }
    fn upgrade2(&mut self, other: &Monster) {
        self.id = upgrade_primitive(self.id, other.id);
        self.name = upgrade_primitive(self.name.clone(), other.name.clone());
        self.threat = upgrade_primitive(self.threat, other.threat);
    }
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

        let monster: Monster = serde_json::from_str(&json).unwrap();

        assert_eq!(monster.id.unwrap(), 1);
        assert_eq!(monster.name.unwrap(), "Crazy Yiuf");
        assert_eq!(monster.threat.unwrap(), 2);
    }
}

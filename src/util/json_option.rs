// see https://github.com/serde-rs/serde/issues/1042

use serde::Deserializer;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonOption<T> {
    Undefined,
    Null,
    Some(T),
}

impl<T> JsonOption<T> {
    pub fn is_defined(&self) -> bool {
        match self {
            JsonOption::Undefined => false,
            _ => true,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            JsonOption::Null => true,
            _ => false,
        }
    }
}

impl<T> Default for JsonOption<T> {
    fn default() -> Self {
        JsonOption::Undefined
    }
}

impl<T> From<Option<T>> for JsonOption<T> {
    fn from(opt: Option<T>) -> JsonOption<T> {
        match opt {
            Some(v) => JsonOption::Some(v),
            None => JsonOption::Null,
        }
    }
}

impl<'de, T> Deserialize<'de> for JsonOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    struct Inner {
        num: Option<i64>,
        text: Option<String>,
        text2: Option<String>,
    }

    #[serde(default)]
    #[derive(Deserialize, Debug, Default, Clone, PartialEq)]
    struct Outer {
        null: JsonOption<Inner>,
        some: JsonOption<Inner>,
        undefined: JsonOption<Inner>,
    }

    #[test]
    fn deserialize_json_option() {
        let outer_deserialized: Outer = serde_json::from_str(
            r#"
            {
                "a": -4,
                "b": "cde",
                "mon": { "x": 1 },
                "null": null,
                "some": {
                    "num": 12,
                    "text": "test123"
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(
            Outer {
                null: JsonOption::Null,
                some: JsonOption::Some(Inner {
                    num: Some(12),
                    text: Some(String::from("test123")),
                    text2: None,
                }),
                undefined: JsonOption::Undefined,
            },
            outer_deserialized
        );
    }
}

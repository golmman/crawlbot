use std::marker::PhantomData;
use serde::Deserializer;
use serde::Deserialize;
use serde_json::Value;
use serde::de::Visitor;

#[derive(Deserialize, Debug, PartialEq)]
struct Incomplete {
    may_be_null: Value,
    may_be_there: Option<String>,
    must_be_there: String,
}

#[test]
fn serde_json_incomplete_deserialization1() {
    let incomplete: Incomplete = serde_json::from_str(
        r#"{
            "may_be_null": "one",
            "may_be_there": "two",
            "must_be_there": "three"
        }"#,
    )
    .unwrap();

    assert_eq!(
        Incomplete {
            may_be_null: Value::String(String::from("one")),
            may_be_there: Some(String::from("two")),
            must_be_there: String::from("three"),
        },
        incomplete
    );
}

#[test]
fn serde_json_incomplete_deserialization2() {
    let incomplete: Incomplete = serde_json::from_str(
        r#"{
            "may_be_null": null,
            "must_be_there": "three"
        }"#,
    )
    .unwrap();

    assert_eq!(
        Incomplete {
            may_be_null: Value::Null,
            may_be_there: None,
            must_be_there: String::from("three"),
        },
        incomplete
    );

    assert!(incomplete.may_be_null.is_null());
}

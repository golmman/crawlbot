use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Msgs {
    msgs: Vec<Msg>,
}

#[test]
fn summation() {
    assert_eq!(1 + 2, 3);
}

#[test]
fn if_let_is_useful_for_deconstruction() {
    let mut actual = 0;
    let expected = 5;

    let optional = Some(expected);
    let none: Option<i32> = None;

    if let Some(value) = optional {
        actual = value;
    }

    if let Some(value) = none {
        assert_eq!(value, value + 1);
    }

    assert_eq!(actual, expected);
}

#[test]
fn if_still_works_for_optionals() {
    let mut actual = 0;
    let expected = 5;

    let optional = Some(expected);

    if Some(5) == optional {
        actual = 5;
    }

    assert_eq!(actual, expected);
}

#[test]
fn try_results() {
    let x;
    let mut actual = "";
    let expected = "some error message";

    let result_err: Result<i32, &str> = Err(expected);

    if let Err(error) = result_err {
        actual = error;
    }

    if let Ok(value) = result_err {
        x = format!("{}", 10);
        actual = x.as_str();
    }

    assert_eq!(actual, expected);
}

#[test]
fn json_serialize() {
    let person = Person {
        name: "dirk".to_string(),
        age: 100,
    };

    let json = serde_json::to_string(&person).unwrap();

    assert_eq!(json, "{\"name\":\"dirk\",\"age\":100}");
}

#[test]
fn json_deserialize() {
    let object: Person = serde_json::from_str("{\"name\":\"dirk\",\"age\":100}").unwrap();

    assert_eq!(object.age, 100);
    assert_eq!(object.name, "dirk");
}

#[test]
fn json_deserialize2() {
    let object: Msgs = serde_json::from_str(
        "{\"msgs\":[{\"msg\": \"lobby_clear\"},{\"msg\": \"lobby_complete\"}]}",
    )
    .unwrap();

    println!("{:?}", object);
}

#[test]
fn json_deserialize3() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "subobject": {
                "test": 12,
                "aaa": "dasdas"
            }
        }"#;

    let v: Value = serde_json::from_str(data).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("dasdasd {}", v["subobject"]["aaa"]);
    println!("dasdasd {}", v["subobject"]["hjjj"]);
}

// #[derive(Debug)]
// enum TestEnum {
//     Test2 = vec![1, 2]
// }

#[test]
fn emptying_queue() {
    let mut queue1: VecDeque<_> = vec![1, 2].into_iter().collect();
    let queue2: VecDeque<_> = vec![3, 4].into_iter().collect();
    let queue3 = vec![3, 4];
    let queue4 = &[3, 4];
    
    // println!("{:?}", TestEnum::Test1);
}

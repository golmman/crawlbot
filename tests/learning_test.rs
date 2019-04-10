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

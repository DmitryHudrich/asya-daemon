use macros::Stringify;

#[allow(dead_code)]
#[derive(Stringify)]
enum TestEnum {
    First,
    Second(i32, usize),
    Third { name: String, value: Option<i32> },
}

#[test]
fn simple_test() {
    println!("{}", TestEnum::stringify_one());
    assert_eq!(
        TestEnum::stringify_one(),
        "enum TestEnum {
    First,
    Second(i32, usize),
    Third { name: String, value: Option<i32> },
}
"
    );
}

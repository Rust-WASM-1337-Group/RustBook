use adder::add_two;

mod common;

#[test]
fn it_adds_two_integration_test() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}

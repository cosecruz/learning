use adder::add_manual;

mod common;

#[test]
fn test_adder() {
    common::setup();
    let result = add_manual(1, 2);
    assert_eq!(3, result);
}

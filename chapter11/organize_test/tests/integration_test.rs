use organize_test::add_two;

mod common; // to use that from any of integration test files as a module

#[test]
fn it_adds_two()
{
    common::setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}
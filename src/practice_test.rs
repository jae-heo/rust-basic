#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
#[should_panic]
fn another() {
    panic!("Make this test fail");
}

#[test]
fn test_assert() {
    assert!(always_true())
}

fn always_true() -> bool {
    true
}
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


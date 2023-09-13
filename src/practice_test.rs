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
    assert!(always_true());
    assert!(!always_false());
}

fn always_true() -> bool {
    true
}

fn always_false() -> bool {
    false
}

#[test]
fn test_with_error_message(){
    let test_string = String::from("my name is jay");

    assert!(test_string.contains("kim"), "the test_string does not contain the word 'kim'. \n the string was : {test_string}")
}
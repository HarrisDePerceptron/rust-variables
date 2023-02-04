use variables;

#[test]
fn use_print_and_return_value(){
    let data = 10;
    assert_eq!(variables::print_and_return_value(data), 10);
}
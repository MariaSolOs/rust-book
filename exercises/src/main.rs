mod collections;

fn main() {
    collections_demo();
}

fn collections_demo() {
    assert_eq!(0.0, collections::median(&vec![1, 0, -1]));
    assert_eq!(1.5, collections::median(&vec![1, 2, 0, 3]));

    assert_eq!(vec![1], collections::mode(&vec![1, 1, 0]));
    assert_eq!(vec![0, 1], collections::mode(&vec![1, 1, 0, 0]));

    let mut word = String::from("first");
    collections::convert_to_pig_latin(&mut word);
    assert_eq!(String::from("irst-fay"), word);
    word = String::from("apple");
    collections::convert_to_pig_latin(&mut word);
    assert_eq!(String::from("apple-hay"), word);

    collections::company_app();
}

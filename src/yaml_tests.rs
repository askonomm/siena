use std::collections::HashMap;
use crate::yaml;

#[test]
fn yaml_generic_test() {
    let seed = "title: Hello, World\nsubtitle: Bye, World";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("subtitle"), String::from("Bye, World"))
    ]);

    assert_eq!(yaml::parse(seed), expected);
}
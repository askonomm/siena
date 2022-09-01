use std::collections::HashMap;
use crate::parsers::front_matter;

#[test]
fn front_matter_generic_test() {
    let seed = "---\ntitle: Hello, World\n---\n\nBye, world.";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("html"), String::from("<p>Bye, world.</p>\n"))
    ]);

    assert_eq!(front_matter::parse(seed), expected);
}

#[test]
fn front_matter_test_no_html() {
    let seed = "---\ntitle: Hello, World\n---";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("html"), String::from(""))
    ]);

    assert_eq!(front_matter::parse(seed), expected);
}

#[test]
fn front_matter_test_no_yaml() {
    let seed = "Hi there world.";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("html"), String::from("<p>Hi there world.</p>\n"))
    ]);

    assert_eq!(front_matter::parse(seed), expected);
}
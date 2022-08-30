use std::collections::HashMap;
use crate::parsers::front_matter_parser;

#[test]
fn frontmatter_test() {
    let seed = "---\ntitle: Hello, world\n---\n\nBye, world.";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("html"), String::from("<p>Bye, world.</p>"))
    ]);

    assert_eq!(front_matter_parser::parse(seed), expected);
}
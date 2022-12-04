use std::collections::HashMap;
use crate::frontmatter;

#[test]
fn frontmatter_generic_test() {
    let seed = "---\ntitle: Hello, World\n---\n\nHi there.";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("content"), String::from("<p>Hi there.</p>\n"))
    ]);

    assert_eq!(frontmatter::parse(seed), expected);
}
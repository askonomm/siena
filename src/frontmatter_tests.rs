use crate::frontmatter;
use std::collections::HashMap;

#[test]
fn frontmatter_generic_test() {
    let seed = "---\ntitle: Hello, World\n---\n\nHi there.";
    let expected: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("content"), String::from("<p>Hi there.</p>\n")),
        (String::from("content_raw"), String::from("Hi there.")),
    ]);

    assert_eq!(frontmatter::parse(seed), expected);
}

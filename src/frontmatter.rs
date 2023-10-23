use crate::siena::RecordData;
use comrak::ComrakOptions;
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrontMatterError {
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub fn parse(contents: &str) -> Result<HashMap<String, RecordData>, FrontMatterError> {
    let re = Regex::new(r"(?is)(?:---)\n(.*?)\n(?:---)")?;
    let yaml_captures = re.captures(contents);

    // Captures not found, return empty HashMap
    if yaml_captures.is_none() {
        return Ok(HashMap::new());
    }

    let yaml_matches = yaml_captures.unwrap();

    // Captures found, but no YAML, return empty HashMap
    if yaml_matches.len() < 2 {
        return Ok(HashMap::new());
    }

    let yaml_match = yaml_matches.get(1).unwrap();
    let yaml_bit = &contents[yaml_match.start()..yaml_match.end()];
    let mut data: HashMap<String, RecordData> = serde_yaml::from_str(yaml_bit)?;

    // Insert Markdown
    let doc = re.replace(contents, "").into_owned().trim().to_owned();
    let md = comrak::markdown_to_html(&doc, &ComrakOptions::default());

    data.insert("content".to_string(), RecordData::Str(md));
    data.insert("content_raw".to_string(), RecordData::Str(doc.to_string()));

    Ok(data)
}

#[test]
fn parse_test() -> Result<(), FrontMatterError> {
    let seed = "---\ntitle: Hello, World\n---\n\nHi there.";
    let expected: HashMap<String, RecordData> = HashMap::from([
        (
            String::from("title"),
            RecordData::Str(String::from("Hello, World")),
        ),
        (
            String::from("content"),
            RecordData::Str(String::from("<p>Hi there.</p>\n")),
        ),
        (
            String::from("content_raw"),
            RecordData::Str(String::from("Hi there.")),
        ),
    ]);

    assert_eq!(parse(seed)?, expected);

    Ok(())
}

use std::collections::HashMap;
use regex::Regex;
use comrak::{markdown_to_html, ComrakOptions};

// Parses the YAML block from the Front Matter `contents` into a key-value
// hashmap.
fn parse_yaml(contents: &str) -> Option<HashMap<String, String>> {
    let meta_block_regex = Regex::new(r"(?s)\-\-\-\n(.*?)\n\-\-\-").unwrap();

    if meta_block_regex.is_match(contents) {
        let meta_block = meta_block_regex.captures(contents);

        if meta_block.is_some() {
            let meta_block_contents = &meta_block.unwrap().get(1).unwrap().as_str();
            let meta: Result<HashMap<String, String>, serde_yaml::Error> = serde_yaml::from_str(meta_block_contents);

            if meta.is_ok() {
                return Some(meta.unwrap());
            }
        }
    }

    return None;
}

// Parses the markdown block from the Front Matter `contents` into a HTML string.
fn parse_markdown(contents: &str) -> String {
    let meta_block_regex = Regex::new(r"(?s)\-\-\-.*?\-\-\-").unwrap();
    let markdown_block = meta_block_regex.replace(contents, "");

    return markdown_to_html(markdown_block.trim(), &ComrakOptions::default())
}

// Given Front Matter `contents`, returns a key-value hashmap where the Markdown content 
// is returned as "entry" key, and parsed into HTML. The YAML block is parsed as-is, with 
// keys and values kept in place.
pub fn parse(contents: &str) -> HashMap<String, String> {
    let meta = parse_yaml(contents);
    let html = parse_markdown(contents);
    let mut result = HashMap::new();

    if meta.is_some() {
        for (key, value) in meta.unwrap().into_iter() {
            result.insert(key, value);
        }
    }

    result.insert(String::from("html"), html);
    
    return result;
}
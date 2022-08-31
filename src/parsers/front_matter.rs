use std::collections::HashMap;
use regex::Regex;
use comrak::{markdown_to_html, ComrakOptions};

// Parses the YAML block from the Markdown file into a key-value
// hashmap. Note that this supports YAML-like structures, not actual YAML, 
// and so things like indents to create hierarchies are not supported.
fn parse_yaml(contents: &str) -> Option<HashMap<String, String>> {
    let meta_block_regex = Regex::new(r"(?s)\-\-\-(.*?)\-\-\-").unwrap();

    if meta_block_regex.is_match(contents) {
        let meta_block = meta_block_regex.captures(contents);

        if meta_block.is_some() {
            let meta_block_contents = &meta_block.unwrap().get(0);
            let lines = meta_block_contents.unwrap().as_str().lines();
            let mut meta = HashMap::new();

            for line in lines {
                let parts: Vec<&str> = line.split(r":").collect();

                if parts.len() > 1 {
                    let key = parts[0];
                    let value = parts[1..].join(":");

                    meta.insert(String::from(key.trim()), String::from(value.trim()));
                }
            }

            return Some(meta);
        }
    }

    return None;
}

// Parses the markdown block from the Markdown file into HTML.
fn parse_markdown(contents: &str) -> String {
    let meta_block_regex = Regex::new(r"(?s)\-\-\-.*?\-\-\-").unwrap();
    let markdown_block = meta_block_regex.replace(contents, "");

    return markdown_to_html(markdown_block.trim(), &ComrakOptions::default())
}


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
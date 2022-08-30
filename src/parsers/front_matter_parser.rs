use std::collections::HashMap;
use regex::Regex;
use comrak::{markdown_to_html, ComrakOptions};

fn parse_meta(contents: &str) -> Option<HashMap<String, String>> {
    let meta_block_regex = Regex::new(r"\-\-\-(.*?)\-\-\-").unwrap();

    if meta_block_regex.is_match(contents) {
        let meta_block = meta_block_regex.find(contents);

        if meta_block.is_some() {
            let lines = meta_block.unwrap().as_str().lines();
            let mut meta = HashMap::new();

            for line in lines {
                let parts: Vec<&str> = line.split(r"\:").collect();

                if parts.len() > 2 {
                    let key = parts[0];
                    let value = parts[1..].join(":");

                    meta.insert(String::from(key), value);
                }
            }

            return Some(meta);
        }
    }

    return None;
}

fn parse_markdown(contents: &str) -> String {
    let meta_block_regex = Regex::new(r"\-\-\-(.*?)\-\-\-").unwrap();
    let markdown_block = meta_block_regex.replace(contents, "");

    return markdown_to_html(markdown_block.trim(), &ComrakOptions::default())
}

pub fn parse(contents: &str) -> HashMap<String, String> {
    let meta = parse_meta(contents);
    let html = parse_markdown(contents);
    let mut result = HashMap::new();

    if meta.is_some() {
        //result.
    }

    result.insert(String::from("html"), html);
    
    return result;
}
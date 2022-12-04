use std::collections::HashMap;
use comrak::ComrakOptions;
use regex::Regex;

fn parse_yaml(contents: &str) -> HashMap<String, String> {
	let parts = contents.split("\n");
	let mut data = HashMap::new();

	for part in parts {
		if part.contains(":") {
			let bits: Vec<&str> = part.split(":").collect();
			let key = bits.first().unwrap().trim();
			let value = bits.last().unwrap().trim();

			data.insert(key.to_string(), value.to_string());
		}
	}

	return data;
}

fn parse_markdown(contents: &str) -> String {
	return comrak::markdown_to_html(contents, &ComrakOptions::default());
}

pub fn parse(contents: &str) -> HashMap<String, String> {
	let re = Regex::new(r"(?is)---(.*?)---").unwrap();
	let yaml_match = re.find(contents);

	if yaml_match.is_none() {
		return HashMap::new();
	}

	let yaml_bit = &contents[yaml_match.unwrap().start()..yaml_match.unwrap().end()];
	let mut data = parse_yaml(yaml_bit);

	let md = parse_markdown(re.replace(contents, "").as_ref().trim());

	data.insert("content".to_string(), md);
	data.insert("content_raw".to_string(), re.replace(contents, "").as_ref().trim().to_string());
	
    return data;
}
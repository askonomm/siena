use std::{collections::HashMap, fs, cmp::Ordering};
use regex::Regex;
use crate::parsers::{front_matter, yaml};

#[derive(Debug, Default)]
pub enum RecordParser {
    #[default] FrontMatter,
    Yaml,
}

#[derive(Debug)]
pub enum RecordSortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Default)]
pub struct Siena {
    directory: String,
    parser: RecordParser,
    records: Vec<HashMap<String, String>>,
}

// 
impl Siena {
    pub fn set_directory(mut self, directory: &str) -> Self {
        self.directory = String::from(directory.trim_end_matches(&['/']));

        return self;
    }

    pub fn set_parser(mut self, parser: RecordParser) -> Self {
        self.parser = parser;

        return self;
    }

    pub fn from_collection(mut self, name: &str) -> Self {
        let dir = fs::read_dir(format!("{}{}{}", self.directory, "/", name));

        if dir.is_ok() {
            for file in dir.unwrap() {
                // Skip iteration when parser does not match file extension
                let file_path = file.as_ref().unwrap().path();
                let file_path_str = file_path.to_str().clone().unwrap();

                match self.parser {
                    RecordParser::FrontMatter => {
                        if !file_path_str.ends_with(".md") && !file_path.ends_with(".markdown") {
                            continue;
                        }
                    }
                    RecordParser::Yaml => {
                        if !file_path_str.ends_with(".yaml") && !file_path.ends_with(".yml") {
                            continue;
                        }
                    }
                }

                // If we made it this far, continue with parsing
                if file.as_ref().is_ok(){
                    let contents = fs::read_to_string(file.as_ref().unwrap().path());

                    if contents.is_ok() {
                        match self.parser {
                            RecordParser::FrontMatter => {
                                self.records.push(front_matter::parse(&contents.unwrap()))
                            }
                            RecordParser::Yaml => {
                                self.records.push(yaml::parse(&contents.unwrap()))
                            }
                        }
                    }
                }
            }
        }
        
        return self;
    }

    pub fn when_equals(mut self, key: &str, equals_value: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record.contains_key(key) && record[key] == equals_value {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    pub fn when_not_equals(mut self, key: &str, equals_value: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record.contains_key(key) && record[key] != equals_value || !record.contains_key(key) {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    pub fn when_has(mut self, key: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();

        for record in &self.records {
            if record.contains_key(key) {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    pub fn when_matches(mut self, key: &str, pattern: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        let re = Regex::new(pattern).unwrap();

        for record in &self.records {
            if record.contains_key(key) && re.is_match(record[key].as_str()) {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    fn sort_compare(a: &HashMap<String, String>, b: &HashMap<String, String>, by: &str, order: &RecordSortOrder) -> Ordering {
        if a.get(by).is_some() && b.get(by).is_some() {
            return match order {
                RecordSortOrder::Asc => a.get(by).unwrap().cmp(b.get(by).unwrap()),
                RecordSortOrder::Desc => b.get(by).unwrap().cmp(a.get(by).unwrap())
            }
        }

        return match order {
            RecordSortOrder::Asc => Ordering::Greater,
            RecordSortOrder::Desc => Ordering::Less
        }
    }

    pub fn sort(mut self, key: &str, order: RecordSortOrder) -> Self {
        self.records.sort_by(|a, b| Self::sort_compare(a, b, key, &order));

        return self;
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.records.truncate(limit);

        return self;
    }

    pub fn offset(mut self, offset: usize) -> Self {
        if self.records.len() >= offset {
            self.records.drain(0..offset);
        } else {
            self.records = Vec::new();
        }

        return self;
    }

    pub fn get_all(self) -> Vec<HashMap<String, String>> {
        return self.records;
    }

    pub fn get_first(self) -> Option<HashMap<String, String>> {
        if self.records.first().is_some() {
            let first_item = self.records.first().unwrap().clone();

            return Some(first_item);
        }

        return None;
    }

    pub fn get_last(self) -> Option<HashMap<String, String>> {
        if self.records.last().is_some() {
            let last_item = self.records.last().unwrap().clone();

            return Some(last_item);
        }

        return None;
    }
}
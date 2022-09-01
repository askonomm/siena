use std::{collections::{HashMap}, fs};
use regex::Regex;

use crate::parsers::front_matter;

#[derive(Debug, Default)]
pub enum RecordParser {
    #[default] FrontMatter,
    Yaml,
}

#[derive(Debug, Default)]
pub struct Siena {
    directory: String,
    parser: RecordParser,
    records: Vec<HashMap<String, String>>,
}

// 
impl Siena {
    // set dir
    // TODO: remove ending forward slash
    pub fn set_directory(mut self, directory: &str) -> Self {
        self.directory = String::from(directory);

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
                if file.is_ok() {
                    let contents = fs::read_to_string(file.unwrap().path());

                    if contents.is_ok() {
                        match self.parser {
                            RecordParser::FrontMatter => {
                                self.records.push(front_matter::parse(&contents.unwrap()))
                            }
                            _ => {
                                self.records.push(front_matter::parse(&contents.unwrap()))
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
            if record[key] == equals_value {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    pub fn when_not_equals(mut self, key: &str, equals_value: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record[key] != equals_value {
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
            if re.is_match(record[key].as_str()) {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.records.truncate(limit);

        return self;
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.records.drain(0..offset);

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
use std::{collections::{HashMap}, fs};

use crate::parsers;

#[derive(Debug, Default)]
pub enum RecordParser {
    #[default] FrontMatter,
}

#[derive(Debug, Default)]
pub struct Siena {
    directory: String,
    parser: RecordParser,
    records: Vec<HashMap<String, String>>,
}

impl Siena {
    pub fn set_directory(mut self, directory: &str) -> Self {
        self.directory = String::from(directory);

        return self;
    }

    pub fn set_parser(mut self, parser: RecordParser) -> Self {
        self.parser = parser;

        return self;
    }

    pub fn collection(mut self, path: &str) -> Self {
        let dir = fs::read_dir(path);

        if dir.is_ok() {
            for file in dir.unwrap() {
                if file.is_ok() {
                    let contents = fs::read_to_string(file.unwrap().path());

                    if contents.is_ok() {
                        match self.parser {
                            RecordParser::FrontMatter => {
                                self.records.push(parsers::front_matter_parser::parse(&contents.unwrap()))
                            }
                        }
                    }
                }
            }
        }
        
        return self;
    }

    pub fn r#where(mut self, key: &str, value: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record[key] == value {
                records.push(record.clone());
            }
        }

        self.records = records;

        return self;
    }

    pub fn where_not(mut self, key: &str, value: &str) -> Self {
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record[key] != value {
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
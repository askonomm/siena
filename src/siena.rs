use std::{collections::HashMap, cmp::Ordering};
use regex::Regex;
use crate::providers::local::LocalProvider;

#[derive(Debug, Default, Clone, Copy)]
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
pub enum Store {
    #[default] None,
    Local {
        directory: String
    }
}

#[derive(Debug, Default)]
pub struct Siena {
    store: Store,
    parser: RecordParser,
    records: Vec<HashMap<String, String>>,
}

// 
impl Siena {
    pub fn set_store(mut self, store: Store) -> Self {
        self.store = store;

        return self;
    }

    pub fn set_parser(mut self, parser: RecordParser) -> Self {
        self.parser = parser;

        return self;
    }

    pub fn from_collection(mut self, name: &str) -> Self {
        match self.store {
            Store::Local { ref directory } => {
                let mut provider = LocalProvider {
                    directory, 
                    parser: self.parser,
                };

                self.records = provider.get_collection(name);
            }

            Store::None => ()
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
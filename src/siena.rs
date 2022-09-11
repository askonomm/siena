use std::{collections::HashMap, cmp::Ordering};
use regex::Regex;
use crate::providers::local::LocalProvider;

#[derive(Debug)]
pub enum RecordSortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Default, Clone)]
pub enum Store {
    #[default] None,
    
    Local {
        directory: String
    }
}

pub trait StoreProvider {
    fn retrieve(&self, name: &str) -> Vec<HashMap<String, String>>;
    fn set(&self, records: Vec<HashMap<String, String>>, data: Vec<(&str, &str)>) -> Vec<HashMap<String, String>>;
}

#[derive(Debug, Default)]
pub struct Siena {
    store: Store,
    records: Vec<HashMap<String, String>>,
}

// 
impl Siena {
    pub fn set_store(mut self, store: Store) -> Self {
        self.store = store;

        return self;
    }

    pub fn from_collection(&self, name: &str) -> Siena {
        let store = self.store.clone();
        let mut records = Vec::new();

        match self.store {
            Store::Local { ref directory } => {
                let provider = LocalProvider {
                    directory,
                };

                records = provider.retrieve(name);
            }

            Store::None => ()
        }

        return Siena {
            store,
            records,
        }
    }

    pub fn when_equals(&self, key: &str, equals_value: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record.contains_key(key) && record[key] == equals_value {
                records.push(record.clone());
            }
        }

        return Siena {
            store,
            records,
        };
    }

    pub fn when_not_equals(&self, key: &str, equals_value: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        for record in &self.records {
            if record.contains_key(key) && record[key] != equals_value || !record.contains_key(key) {
                records.push(record.clone());
            }
        }

        return Siena {
            store,
            records,
        };
    }

    pub fn when_has(&self, key: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<HashMap<String, String>> = Vec::new();

        for record in &self.records {
            if record.contains_key(key) {
                records.push(record.clone());
            }
        }

        return Siena {
            store,
            records,
        };
    }

    pub fn when_matches(&self, key: &str, pattern: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        let re = Regex::new(pattern).unwrap();

        for record in &self.records {
            if record.contains_key(key) && re.is_match(record[key].as_str()) {
                records.push(record.clone());
            }
        }

        return Siena {
            store,
            records,
        };
    }

    pub fn sort(&self, key: &str, order: RecordSortOrder) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        records.sort_by(|a, b| {
            if a.get(key).is_some() && b.get(key).is_some() {
                return match order {
                    RecordSortOrder::Asc => a.get(key).unwrap().cmp(b.get(key).unwrap()),
                    RecordSortOrder::Desc => b.get(key).unwrap().cmp(a.get(key).unwrap())
                }
            }
    
            return match order {
                RecordSortOrder::Asc => Ordering::Greater,
                RecordSortOrder::Desc => Ordering::Less
            }
        });

        return Siena {
            store,
            records,
        };
    }

    pub fn limit(&self, limit: usize) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        records.truncate(limit);

        return Siena {
            store,
            records,
        };
    }

    pub fn offset(&self, offset: usize) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        if records.len() >= offset {
            records.drain(0..offset);
        } else {
            records = Vec::new();
        }

        return Siena {
            store,
            records,
        };
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

    pub fn set(&self, data: Vec<(&str, &str)>) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        match self.store {
            Store::Local { ref directory } => {
                let provider = LocalProvider {
                    directory, 
                };

                records = provider.set(records, data);
            }

            Store::None => ()
        }

        return Siena {
            store,
            records,
        }
    }

    pub fn create(&self, collection: &str, id: &str) -> Siena {
        let store = self.store.clone();
        let mut records = Vec::new();
        let mut record: HashMap<String, String> = HashMap::new();

        record.insert("_id".to_string(), id.to_string());
        record.insert("_collection".to_string(), collection.to_string());
        record.insert("_file_name".to_string(), format!("{}.yml", id));

        records.push(record);

        return Siena {
            store,
            records,
        }
    }
}
use std::{collections::HashMap, cmp::Ordering};
use regex::Regex;
use crate::providers::local::LocalProvider;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Record {
    pub id: String,
    pub collection: String,
    pub file_name: String,
    pub data: HashMap<String, String>,
}

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
    fn retrieve(&self, name: &str) -> Vec<Record>;
    fn set(&self, records: Vec<Record>, data: Vec<(&str, &str)>) -> Vec<Record>;
}

#[derive(Debug, Default)]
pub struct Siena {
    store: Store,
    records: Vec<Record>,
}

impl Siena {
    // Sets the type of store being used which will change the provider
    // being used to fetch, create and manipulate records.
    pub fn set_store(mut self, store: Store) -> Self {
        self.store = store;

        return self;
    }

    // Fetch records from a collection with a given `name`. A collection
    // is a directory in your Store, and a record is a YAML file in that
    // directory.
    pub fn collection(&self, name: &str) -> Siena {
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

        Siena {
            store,
            records,
        }
    }

    // Filter records based on value equality for a key.
    pub fn when_is(&self, key: &str, equals_value: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<Record> = Vec::new();
        
        for record in &self.records {
            if record.data.contains_key(key) && record.data[key] == equals_value {
                records.push(record.clone());
            }
        }

        Siena {
            store,
            records,
        }
    }

    // Filter records based on value inequality for a key.
    pub fn when_isnt(&self, key: &str, equals_value: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<Record> = Vec::new();
        
        for record in &self.records {
            if record.data.contains_key(key) && record.data[key] != equals_value || !record.data.contains_key(key) {
                records.push(record.clone());
            }
        }

        Siena {
            store,
            records,
        }
    }

    // Filter records based on key presence.
    pub fn when_has(&self, key: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<Record> = Vec::new();

        for record in &self.records {
            if record.data.contains_key(key) {
                records.push(record.clone());
            }
        }

        Siena {
            store,
            records,
        }
    }

    // Filter records based on key lack of presence.
    pub fn when_hasnt(&self, key: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<Record> = Vec::new();

        for record in &self.records {
            if !record.data.contains_key(key) {
                records.push(record.clone());
            }
        }

        Siena {
            store,
            records,
        }
    }

    // Filter records based on value matching a regex pattern for a key.
    pub fn when_matches(&self, key: &str, pattern: &str) -> Siena {
        let store = self.store.clone();
        let mut records: Vec<Record> = Vec::new();
        let re = Regex::new(pattern).unwrap();

        for record in &self.records {
            if record.data.contains_key(key) && re.is_match(record.data[key].as_str()) {
                records.push(record.clone());
            }
        }

        Siena {
            store,
            records,
        }
    }

    // Sort records by a value for a key in `RecordSortOrder`.
    pub fn sort(&self, key: &str, order: RecordSortOrder) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        records.sort_by(|a, b| {
            if a.data.get(key).is_some() && b.data.get(key).is_some() {
                return match order {
                    RecordSortOrder::Asc => a.data.get(key).unwrap().cmp(b.data.get(key).unwrap()),
                    RecordSortOrder::Desc => b.data.get(key).unwrap().cmp(a.data.get(key).unwrap())
                }
            }
    
            match order {
                RecordSortOrder::Asc => Ordering::Greater,
                RecordSortOrder::Desc => Ordering::Less
            }
        });

        Siena {
            store,
            records,
        }
    }

    // Limit records.
    pub fn limit(&self, limit: usize) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        records.truncate(limit);

        Siena {
            store,
            records,
        }
    }

    // Offset records.
    pub fn offset(&self, offset: usize) -> Siena {
        let store = self.store.clone();
        let mut records = self.records.clone();

        if records.len() >= offset {
            records.drain(0..offset);
        } else {
            records = Vec::new();
        }

        Siena {
            store,
            records,
        }
    }

    // Paginate records.
    pub fn paginate(&self, page: usize, limit: usize) -> Siena {
        self.offset((page - 1) * limit).limit(limit)
    }

    // Get all records.
    pub fn get_all(self) -> Vec<Record> {
        self.records
    }

    // Get first record.
    pub fn get_first(self) -> Option<Record> {
        if self.records.first().is_some() {
            let first_item = self.records.first().unwrap().clone();

            return Some(first_item);
        }

        None
    }

    // Get last record.
    pub fn get_last(self) -> Option<Record> {
        if self.records.last().is_some() {
            let last_item = self.records.last().unwrap().clone();

            return Some(last_item);
        }

        None
    }

    // Set a Vector of tuples (key, value) in all records queried,
    // and persist them on file.
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

        Siena {
            store,
            records,
        }
    }

    // Create a new record in a `collection` with the given `id`.
    //
    // Note: this alone does not persist the newly created record, and in
    // order to do so you must also set some data via the `set` method.
    //
    // Also note: if the record with such an `id` in given `collection` already
    // exists, it will be overwritten.
    pub fn create(&self, collection: &str, id: &str) -> Siena {
        let store = self.store.clone();
        let mut records = Vec::new();

        records.push(Record {
            id: id.to_string(),
            collection: collection.to_string(),
            file_name: format!("{}.yml", id),
            data: HashMap::new()
        });

        Siena {
            store,
            records,
        }
    }
}

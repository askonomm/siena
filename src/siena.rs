use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Custom(fn(String, String) -> Ordering),
}

pub trait StoreProvider {
    fn retrieve(&self, name: &str) -> Vec<Record>;
    fn set(&self, records: Vec<Record>, data: Vec<(&str, &str)>) -> Vec<Record>;
    fn delete(&self, records: Vec<Record>);
}

impl Debug for dyn StoreProvider {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StoreProvider")
    }
}

#[derive(Debug)]
pub struct Siena {
    pub(crate) records: Vec<Record>,
    pub(crate) provider: Box<dyn StoreProvider>,
}

impl Siena {
    // Sets the type of store being used which will change the provider
    // being used to fetch, create and manipulate records.
    pub fn set_provider(mut self, provider: impl StoreProvider + 'static) -> Self {
        self.provider = Box::new(provider);

        self
    }

    // Fetch records from a collection with a given `name`. A collection
    // is a directory in your Store, and a record is a YAML file in that
    // directory.
    pub fn collection(mut self, name: &str) -> Siena {
        self.records = self.provider.retrieve(name);

        self
    }

    // Filter records based on value equality for a key.
    pub fn when_is(mut self, key: &str, equals_value: &str) -> Siena {
        self.records = self
            .records
            .into_iter()
            .filter(|r| {
                if key == "id" && r.id == equals_value {
                    return true;
                }

                if r.data.contains_key(key) && r.data[key] == equals_value {
                    return true;
                }

                return false;
            })
            .collect();

        self
    }

    // Filter records based on value inequality for a key.
    pub fn when_isnt(mut self, key: &str, equals_value: &str) -> Siena {
        self.records = self
            .records
            .into_iter()
            .filter(|r| {
                if key == "id" && r.id != equals_value {
                    return true;
                }

                let contains_k = r.data.contains_key(key);

                if (contains_k && r.data[key] != equals_value) || !contains_k {
                    return true;
                }

                return false;
            })
            .collect();

        self
    }

    // Filter records based on key presence.
    pub fn when_has(mut self, key: &str) -> Siena {
        self.records = self
            .records
            .into_iter()
            .filter(|r| r.data.contains_key(key))
            .collect();

        self
    }

    // Filter records based on key lack of presence.
    pub fn when_hasnt(mut self, key: &str) -> Siena {
        self.records = self
            .records
            .into_iter()
            .filter(|r| !r.data.contains_key(key))
            .collect();

        self
    }

    // Filter records based on value matching a regex pattern for a key.
    pub fn when_matches(mut self, key: &str, pattern: &str) -> Siena {
        let re = Regex::new(pattern).unwrap();

        self.records = self
            .records
            .into_iter()
            .filter(|r| {
                if key == "id" && re.is_match(r.id.as_str()) {
                    return true;
                }

                if r.data.contains_key(key) && re.is_match(r.data[key].as_str()) {
                    return true;
                }

                return false;
            })
            .collect();

        self
    }

    // Sort records by a value for a key in `RecordSortOrder`.
    pub fn sort(mut self, key: &str, order: RecordSortOrder) -> Siena {
        self.records.sort_by(|a, b| {
            if key == "id" {
                return match order {
                    RecordSortOrder::Asc => a.id.cmp(&b.id),
                    RecordSortOrder::Desc => b.id.cmp(&a.id),
                    RecordSortOrder::Custom(f) => f(a.clone().id, b.clone().id),
                };
            }

            if a.data.get(key).is_some() && b.data.get(key).is_some() {
                let a_key = a.data.get(key).unwrap();
                let b_key = b.data.get(key).unwrap();

                return match order {
                    RecordSortOrder::Asc => a_key.cmp(b_key),
                    RecordSortOrder::Desc => b_key.cmp(a_key),
                    RecordSortOrder::Custom(f) => f(a_key.clone(), b_key.clone()),
                };
            }

            match order {
                RecordSortOrder::Asc => Ordering::Greater,
                RecordSortOrder::Desc => Ordering::Less,
                _ => Ordering::Less,
            }
        });

        self
    }

    // Limit records.
    pub fn limit(mut self, limit: usize) -> Siena {
        self.records.truncate(limit);

        self
    }

    // Offset records.
    pub fn offset(mut self, offset: usize) -> Siena {
        if self.records.len() >= offset {
            self.records.drain(0..offset);
        } else {
            self.records = Vec::new();
        }

        self
    }

    // Paginate records.
    pub fn paginate(self, page: usize, limit: usize) -> Siena {
        self.offset((page - 1) * limit).limit(limit)
    }

    // Get all records.
    pub fn get_all(self) -> Vec<Record> {
        self.records
    }

    // Get first record.
    pub fn get_first(self) -> Option<Record> {
        if self.records.first().is_some() {
            return Some(self.records.first().unwrap().clone());
        }

        None
    }

    // Get last record.
    pub fn get_last(self) -> Option<Record> {
        if self.records.last().is_some() {
            return Some(self.records.last().unwrap().clone());
        }

        None
    }

    // Set a Vector of tuples (key, value) in all records queried,
    // and persist them on file.
    pub fn set(self, data: Vec<(&str, &str)>) {
        self.provider.set(self.records.clone(), data);
    }

    // Delete all records queried from disk.
    pub fn delete(self) {
        self.provider.delete(self.records.clone());
    }

    // Create a new record in a `collection` with the given `id`.
    //
    // Note: this alone does not persist the newly created record, and in
    // order to do so you must also set some data via the `set` method.
    //
    // Also note: if the record with such an `id` in given `collection` already
    // exists, it will be overwritten.
    pub fn create(mut self, collection: &str, id: &str) -> Siena {
        self.records.push(Record {
            id: id.to_string(),
            collection: collection.to_string(),
            file_name: format!("{}.yml", id),
            data: HashMap::new(),
        });

        self
    }
}

pub fn siena(provider: impl StoreProvider + 'static) -> Siena {
    Siena {
        records: vec![],
        provider: Box::new(provider),
    }
}

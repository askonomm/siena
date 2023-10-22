use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RecordData {
    Str(String),
    Num(usize),
    Bool(bool),
    Map(HashMap<String, RecordData>),
    Vec(Vec<RecordData>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub collection: String,
    pub file_name: String,
    pub data: HashMap<String, RecordData>,
}

#[derive(Debug)]
pub enum RecordSortOrder {
    Asc,
    Desc,
    CustomStr(fn(String, String) -> Ordering),
    CustomNum(fn(usize, usize) -> Ordering),
}

pub trait StoreProvider {
    fn retrieve(&self, name: &str) -> Vec<Record>;
    fn set(&self, records: Vec<Record>, data: Vec<(&str, &RecordData)>) -> Vec<Record>;
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

                if !r.data.contains_key(key) {
                    return false;
                }

                return match r.data.get(key).unwrap() {
                    RecordData::Str(val) => val == equals_value,
                    _ => false,
                };
            })
            .collect();

        self
    }

    // Filter records based on value inequality for a key.
    pub fn when_is_not(mut self, key: &str, equals_value: &str) -> Siena {
        self.records = self
            .records
            .into_iter()
            .filter(|r| {
                if key == "id" && r.id != equals_value {
                    return true;
                }

                if !r.data.contains_key(key) {
                    return false;
                }

                return match r.data.get(key).unwrap() {
                    RecordData::Str(val) => val != equals_value,
                    _ => false,
                };
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
    pub fn when_has_not(mut self, key: &str) -> Siena {
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

                if !r.data.contains_key(key) {
                    return false;
                }

                return match r.data.get(key).unwrap() {
                    RecordData::Str(val) => re.is_match(val.as_str()),
                    _ => false,
                };
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
                    RecordSortOrder::CustomStr(f) => f(a.clone().id, b.clone().id),
                    _ => Ordering::Equal,
                };
            }

            if a.data.get(key).is_none() || b.data.get(key).is_none() {
                return match order {
                    RecordSortOrder::Asc => Ordering::Greater,
                    RecordSortOrder::Desc => Ordering::Less,
                    _ => Ordering::Equal,
                };
            }
            return match (a.data.get(key).unwrap(), b.data.get(key).unwrap()) {
                (RecordData::Str(a), RecordData::Str(b)) => {
                    return match order {
                        RecordSortOrder::Asc => a.cmp(b),
                        RecordSortOrder::Desc => b.cmp(a),
                        RecordSortOrder::CustomStr(f) => f(a.clone(), b.clone()),
                        _ => Ordering::Equal,
                    };
                }
                (RecordData::Num(a), RecordData::Num(b)) => {
                    return match order {
                        RecordSortOrder::Asc => a.cmp(b),
                        RecordSortOrder::Desc => b.cmp(a),
                        RecordSortOrder::CustomNum(f) => f(a.clone(), b.clone()),
                        _ => Ordering::Equal,
                    };
                }
                _ => Ordering::Equal,
            };
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
    pub fn set(self, data: Vec<(&str, &RecordData)>) {
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

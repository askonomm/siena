use std::{fs};
use crate::{siena::{StoreProvider}, yaml};
use crate::siena::Record;

pub struct LocalProvider<'a> {
    pub directory: &'a str,
}

impl StoreProvider for LocalProvider<'_> {
    fn retrieve(&self, name: &str) -> Vec<Record> {
        let mut records = Vec::new();
        let dir = fs::read_dir(format!("{}{}{}", self.directory, "/", name));

        if dir.is_err() {
            return records;
        }
        
        for file in dir.unwrap() {
            // Skip iteration when parser does not match file extension
            let file_path = file.as_ref().unwrap().path();
            let file_path_str = file_path.to_str().clone().unwrap();

            if !file_path_str.ends_with(".yaml") && !file_path_str.ends_with(".yml") {
                continue;
            }

            // If we made it this far, continue with parsing
            if file.as_ref().is_ok(){
                let contents = fs::read_to_string(file.as_ref().unwrap().path());

                if contents.is_ok() {
                    let file_name = file.as_ref().unwrap().file_name();
                    let id = file_name.to_str().unwrap()
                        .replace(".yml", "")
                        .replace(".yaml", "")
                        .replace(".md", "")
                        .replace(".markdown", "");

                    records.push(Record {
                        id,
                        collection: name.to_string(),
                        file_name: file_name.to_str().unwrap().to_string(),
                        data: yaml::parse(&contents.unwrap())
                    });
                }
            }
        }

        return records;
    }

    fn set(&self, records: Vec<Record>, data: Vec<(&str, &str)>) -> Vec<Record> {
        let mut updated_records: Vec<Record> = Vec::new();

        for mut record in records {
            let directory = format!("{}/{}", self.directory, record.collection);

            // Create dir if it doesnt exist
            fs::create_dir_all(&directory).unwrap();

            // Write to file
            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(format!("{}/{}", directory, record.file_name))
                .unwrap();

            for data_item in data.clone() {
                record.data.insert(data_item.0.to_string(), data_item.1.to_string());
            }

            updated_records.push(record.clone());

            serde_yaml::to_writer(file, &record.data).unwrap();
        }

        return updated_records;
    }
}
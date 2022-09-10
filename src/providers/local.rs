use std::{fs, collections::HashMap};
use crate::{siena::{StoreProvider}, yaml};

pub struct LocalProvider<'a> {
    pub directory: &'a str,
}

impl StoreProvider for LocalProvider<'_> {
    fn retrieve(&self, name: &str) -> Vec<HashMap<String, String>> {
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
                    let mut record: HashMap<String, String> = HashMap::new();
                    let file_name = file.as_ref().unwrap().file_name();
                    let file_name_str = file_name.to_str().unwrap()
                        .replace(".yml", "")
                        .replace(".yaml", "")
                        .replace(".md", "")
                        .replace(".markdown", "");

                    record.insert("_id".to_string(), file_name_str);
                    record.insert("_collection".to_string(), name.to_string());
                    record.insert("_file_name".to_string(), file_name.to_str().unwrap().to_string());
                    record.extend(yaml::parse(&contents.unwrap()));
                    records.push(record);
                }
            }
        }

        return records;
    }

    fn update(&self, records: Vec<HashMap<String, String>>, key: &str, value: &str) {
        for mut record in records {
            let collection = record.get("_collection").unwrap();
            let file_name = record.get("_file_name").unwrap();
            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(format!("{}{}{}{}{}", self.directory, "/", collection, "/", file_name))
                .unwrap();

            record.insert(key.to_string(), value.to_string());
            record.remove("_id");
            record.remove("_file_name");
            record.remove("_collection");

            serde_yaml::to_writer(file, &record).unwrap();
        }
    }
}
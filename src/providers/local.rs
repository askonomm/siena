use std::collections::HashMap;
use std::fs;
use crate::{siena::{StoreProvider, Record}, yaml, utils::string_ends_with_any, frontmatter};

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

            if !string_ends_with_any(file_path_str, Vec::from(["yml", "yaml", "md", "markdown"])) {
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
                    
                    let mut data = HashMap::new();

                    if string_ends_with_any(file_path_str, Vec::from(["yml", "yaml"])) {
                        data = yaml::parse(&contents.as_ref().unwrap());
                    }

                    if string_ends_with_any(file_path_str, Vec::from(["md", "markdown"])) {
                        data = frontmatter::parse(&contents.as_ref().unwrap());
                    }

                    records.push(Record {
                        id,
                        collection: name.to_string(),
                        file_name: file_name.to_str().unwrap().to_string(),
                        data,
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

            // yaml 
            if string_ends_with_any(record.file_name.as_ref(), Vec::from(["yml", "yaml"])) {
                serde_yaml::to_writer(file, &record.data).unwrap();
            }

            // frontmatter
            if string_ends_with_any(record.file_name.as_ref(), Vec::from(["md", "markdown"])) {
                let meta = record.data.clone();
                let markdown = record.data.get("content_raw").unwrap();
                let frontmatter = serde_frontmatter::serialize(meta, markdown).unwrap();

                fs::write(format!("{}/{}", directory, record.file_name), frontmatter).unwrap();
            }
        }

        return updated_records;
    }
}
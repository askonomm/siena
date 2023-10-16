use crate::{
    frontmatter,
    siena::{Record, StoreProvider},
    utils::str_ends_with_any,
    yaml,
};
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct LocalProvider {
    pub directory: String,
}

impl StoreProvider for LocalProvider {
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
            let allowed_exts = Vec::from(["yml", "yaml", "md", "markdown"]);

            if !str_ends_with_any(file_path_str, allowed_exts) {
                continue;
            }

            // If we made it this far, continue with parsing
            if file.as_ref().is_ok() {
                let contents = fs::read_to_string(file.as_ref().unwrap().path());

                if contents.is_ok() {
                    let file_name = file.as_ref().unwrap().file_name();
                    let id = file_name
                        .to_str()
                        .unwrap()
                        .replace(".yml", "")
                        .replace(".yaml", "")
                        .replace(".md", "")
                        .replace(".markdown", "");

                    let mut data = HashMap::new();

                    if str_ends_with_any(file_path_str, Vec::from(["yml", "yaml"])) {
                        data = yaml::parse(&contents.as_ref().unwrap());
                    }

                    if str_ends_with_any(file_path_str, Vec::from(["md", "markdown"])) {
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
            fs::create_dir_all(&directory).expect(&format!("Could not create {}", directory));

            // Write to file
            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(format!("{}/{}", directory, record.file_name))
                .expect(&format!(
                    "Could not write to file {}/{}",
                    directory, record.file_name
                ));

            for data_item in data.clone() {
                record
                    .data
                    .insert(data_item.0.to_string(), data_item.1.to_string());
            }

            updated_records.push(record.clone());

            // yaml
            if str_ends_with_any(record.file_name.as_ref(), Vec::from(["yml", "yaml"])) {
                serde_yaml::to_writer(file, &record.data).expect(&format!(
                    "Could not write to file {}/{}",
                    directory, record.file_name
                ));
            }

            // frontmatter
            if str_ends_with_any(record.file_name.as_ref(), Vec::from(["md", "markdown"])) {
                let meta = record.data.clone();
                let default_markdown = String::from("");
                let markdown = record.data.get("content_raw").unwrap_or(&default_markdown);
                let frontmatter =
                    serde_frontmatter::serialize(meta, markdown).unwrap_or(String::from(""));
                let file_path = format!("{}/{}", directory, record.file_name);

                fs::write(&file_path, frontmatter)
                    .expect(&format!("Could not write to {}", file_path));
            }
        }

        return updated_records;
    }

    fn delete(&self, records: Vec<Record>) {
        for record in records {
            let directory = format!("{}/{}", self.directory, record.collection);
            let file = format!("{}/{}", directory, record.file_name);

            fs::remove_file(file.clone()).expect(&format!("Cannot delete file: {}", file));
        }
    }
}

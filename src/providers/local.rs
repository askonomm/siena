use thiserror::Error;

use crate::{
    frontmatter,
    siena::{Record, RecordData, StoreProvider},
    utils::str_ends_with_any,
};
use std::fs;
use std::{collections::HashMap, fs::DirEntry};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

fn parse_file(file: &DirEntry, collection: &str) -> Result<Record, ParseError> {
    let contents = fs::read_to_string(file.path())?;
    let file_name = file.file_name();
    let mut data = HashMap::new();
    let id = file_name
        .to_str()
        .unwrap()
        .replace(".yml", "")
        .replace(".yaml", "")
        .replace(".md", "")
        .replace(".markdown", "");

    if str_ends_with_any(file.path().to_str().unwrap(), Vec::from(["yml", "yaml"])) {
        if let Ok(yaml) = serde_yaml::from_str(&contents) {
            data = yaml;
        }
    }

    if str_ends_with_any(file.path().to_str().unwrap(), Vec::from(["md", "markdown"])) {
        if let Ok(fm) = frontmatter::parse(&contents) {
            data = fm;
        }
    }

    Ok(Record {
        id,
        collection: collection.to_string(),
        file_name: file_name.to_str().unwrap().to_string(),
        data,
    })
}

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
                if let Ok(record) = parse_file(&file.unwrap(), &name) {
                    records.push(record);
                }
            }
        }

        return records;
    }

    fn set(&self, records: Vec<Record>, data: Vec<(&str, &RecordData)>) -> Vec<Record> {
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
                    .insert(data_item.0.to_string(), data_item.1.clone());
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

                match record.data.get("content_raw").unwrap() {
                    RecordData::Str(md) => {
                        let fm = serde_frontmatter::serialize(meta, md).unwrap_or(String::from(""));
                        let file_path = format!("{}/{}", directory, record.file_name);

                        fs::write(&file_path, fm)
                            .expect(&format!("Could not write to {}", file_path));
                    }
                    _ => (),
                }
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

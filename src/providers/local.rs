use std::{fs, collections::HashMap};
use crate::{siena::{RecordParser, StoreProvider}, parsers::{front_matter, yaml}};

pub struct LocalProvider<'a> {
    pub directory: &'a str,
    pub parser: RecordParser,
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

            match self.parser {
                RecordParser::FrontMatter => {
                    if !file_path_str.ends_with(".md") && !file_path_str.ends_with(".markdown") {
                        continue;
                    }
                }
                RecordParser::Yaml => {
                    if !file_path_str.ends_with(".yaml") && !file_path_str.ends_with(".yml") {
                        continue;
                    }
                }
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
                   
                    match self.parser {
                        RecordParser::FrontMatter => {
                            record.extend(front_matter::parse(&contents.unwrap()));
                        }
                        RecordParser::Yaml => {
                            record.extend(yaml::parse(&contents.unwrap()));
                        }
                    }

                    records.push(record);
                }
            }
        }

        return records;
    }
}
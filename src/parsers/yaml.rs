use std::collections::HashMap;

pub fn parse(contents: &str) -> HashMap<String, String> {
    let meta: Result<HashMap<String, String>, serde_yaml::Error> = serde_yaml::from_str(contents);

    if meta.is_ok() {
        return meta.unwrap();
    }

    return HashMap::new();
}
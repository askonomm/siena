use std::collections::HashMap;

pub fn parse(contents: &str) -> HashMap<String, String> {
    let data: Result<HashMap<String, String>, serde_yaml::Error> = serde_yaml::from_str(contents);

    if data.is_ok() {
        return data.unwrap();
    }

    return HashMap::new();
}

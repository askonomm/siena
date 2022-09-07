use std::{collections::HashMap, env};
use crate::siena::{Siena, RecordSortOrder, Store};

#[test]
fn sort_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });
        
    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Bye, World")),
        (String::from("date"), String::from("2022-01-01")),
        (String::from("html"), String::from("<p>Bye world.</p>\n"))
    ]);

    let expected_data_item_2: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("date"), String::from("2020-01-01")),
        (String::from("html"), String::from("<p>Hi world.</p>\n"))
    ]);

    let expected_data_item_3: HashMap<String, String> = HashMap::from([
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
        (String::from("html"), String::from(""))
    ]);
    
    let expected = Vec::from([
        expected_data_item, 
        expected_data_item_2,
        expected_data_item_3,
    ]);

    let result = store
        .from_collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn when_equals_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });

    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("date"), String::from("2020-01-01")),
        (String::from("html"), String::from("<p>Hi world.</p>\n"))
    ]);

    let expected = Vec::from([
        expected_data_item, 
    ]);

    let result = store
        .from_collection("demo")
        .when_equals("title", "Hello, World")
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn when_not_equals_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });

    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Bye, World")),
        (String::from("date"), String::from("2022-01-01")),
        (String::from("html"), String::from("<p>Bye world.</p>\n"))
    ]);

    let expected_data_item_2: HashMap<String, String> = HashMap::from([
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
        (String::from("html"), String::from(""))
    ]);

    let expected = Vec::from([
        expected_data_item, 
        expected_data_item_2,
    ]);

    let result = store
        .from_collection("demo")
        .when_not_equals("title", "Hello, World")
        .sort("date", RecordSortOrder::Desc)
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn when_has_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });

    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
        (String::from("html"), String::from(""))
    ]);

    let expected = Vec::from([
        expected_data_item
    ]);

    let result = store
        .from_collection("demo")
        .when_has("special-item")
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn when_matches_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });

    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Bye, World")),
        (String::from("date"), String::from("2022-01-01")),
        (String::from("html"), String::from("<p>Bye world.</p>\n"))
    ]);

    let expected = Vec::from([
        expected_data_item
    ]);

    let result = store
        .from_collection("demo")
        .when_matches("title", r"Bye")
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn limit_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });
    
    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Bye, World")),
        (String::from("date"), String::from("2022-01-01")),
        (String::from("html"), String::from("<p>Bye world.</p>\n"))
    ]);

    let expected = Vec::from([
        expected_data_item
    ]);

    let result = store
        .from_collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .limit(1)
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn offset_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });
    
    let expected_data_item: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("date"), String::from("2020-01-01")),
        (String::from("html"), String::from("<p>Hi world.</p>\n"))
    ]);

    let expected = Vec::from([
        expected_data_item
    ]);

    let result = store
        .from_collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .offset(1)
        .limit(1)
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn offset_out_of_bounds_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });
    
    let result = store
        .from_collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .offset(5)
        .get_all();

    assert_eq!(result, Vec::new());
}
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
        (String::from("title"), String::from("Bye, world")),
        (String::from("_id"), String::from("test")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("test.yaml")),
        (String::from("date"), String::from("2022-09-10")),
    ]);
        
    let expected_data_item_2: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, world")),
        (String::from("_id"), String::from("test2")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("test2.yml")),
        (String::from("date"), String::from("2022-09-09")),
    ]);

    let expected_data_item_3: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Bye, World")),
        (String::from("_id"), String::from("2")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("2.yml")),
        (String::from("date"), String::from("2022-01-01")),
    ]);

    let expected_data_item_4: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("_id"), String::from("1")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("1.yml")),
        (String::from("date"), String::from("2020-01-01")),
    ]);

    let expected_data_item_5: HashMap<String, String> = HashMap::from([
        (String::from("_id"), String::from("3")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("3.yml")),
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
    ]);
    
    let expected = Vec::from([
        expected_data_item, 
        expected_data_item_2,
        expected_data_item_3,
        expected_data_item_4,
        expected_data_item_5,
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
        (String::from("_id"), String::from("1")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("1.yml")),
        (String::from("title"), String::from("Hello, World")),
        (String::from("date"), String::from("2020-01-01")),
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

    let expected: HashMap<String, String> = HashMap::from([
        (String::from("_id"), String::from("1")),
        (String::from("_file_name"), String::from("1.yml")),
        (String::from("_collection"), String::from("demo")),
        (String::from("title"), String::from("Hello, World")),
        (String::from("date"), String::from("2020-01-01")),
    ]);

    let result = store
        .from_collection("demo")
        .when_not_equals("date", "2022-01-01")
        .when_not_equals("date", "1992-09-17")
        .when_not_equals("date", "2022-09-09")
        .when_not_equals("date", "2022-09-10")
        .sort("date", RecordSortOrder::Desc)
        .get_first()
        .unwrap();

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
        (String::from("_id"), String::from("3")),
        (String::from("_file_name"), String::from("3.yml")),
        (String::from("_collection"), String::from("demo")),
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
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
        (String::from("_id"), String::from("3")),
        (String::from("_file_name"), String::from("3.yml")),
        (String::from("_collection"), String::from("demo")),
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
    ]);

    let expected = Vec::from([
        expected_data_item
    ]);

    let result = store
        .from_collection("demo")
        .when_matches("date", r"1992")
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
        (String::from("_id"), String::from("test")),
        (String::from("_file_name"), String::from("test.yaml")),
        (String::from("_collection"), String::from("demo")),
        (String::from("title"), String::from("Bye, world")),
        (String::from("date"), String::from("2022-09-10")),
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
        (String::from("_id"), String::from("test2")),
        (String::from("_file_name"), String::from("test2.yml")),
        (String::from("_collection"), String::from("demo")),
        (String::from("title"), String::from("Hello, world")),
        (String::from("date"), String::from("2022-09-09")),
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

#[test]
fn update_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });
    
    store
        .from_collection("demo")
        .when_equals("date", "1992-09-17")
        .set(Vec::from([("special-item", "false")]));

    let result = store
        .from_collection("demo")
        .when_equals("date", "1992-09-17")
        .get_first()
        .unwrap();

    let expected = HashMap::from([
        (String::from("_id"), String::from("3")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("3.yml")),
        (String::from("special-item"), String::from("false")),
        (String::from("date"), String::from("1992-09-17")),
    ]);

    assert_eq!(result, expected);

    store
        .from_collection("demo")
        .when_equals("date", "1992-09-17")
        .set(Vec::from([("special-item", "true")]));

    let result_again = store
        .from_collection("demo")
        .when_equals("date", "1992-09-17")
        .get_first()
        .unwrap();

    let expected_again = HashMap::from([
        (String::from("_id"), String::from("3")),
        (String::from("_collection"), String::from("demo")),
        (String::from("_file_name"), String::from("3.yml")),
        (String::from("special-item"), String::from("true")),
        (String::from("date"), String::from("1992-09-17")),
    ]);

    assert!(result.eq(&expected) && result_again.eq(&expected_again))
}

#[test]
fn create_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default()
        .set_store(Store::Local {
            directory: format!("{}{}", root_dir.display().to_string().as_str(), "/test_data")
        });

    store
        .create("demo2", "test3")
        .set(Vec::from([("title", "Title goes here")]));

    let result = store
        .from_collection("demo2")
        .get_first()
        .unwrap();

    let expected = HashMap::from([
        (String::from("_id"), String::from("test3")),
        (String::from("_collection"), String::from("demo2")),
        (String::from("_file_name"), String::from("test3.yml")),
        (String::from("title"), String::from("Title goes here"))
    ]);

    assert_eq!(result, expected);
}
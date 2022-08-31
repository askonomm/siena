use std::{collections::HashMap, env};
use crate::siena::Siena;

#[test]
fn get_collection_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default().set_directory(&format!("{}{}", root_dir.display().to_string().as_str(), "/test_data"));

    let expected_data_item_1: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, World")),
        (String::from("date"), String::from("2020-01-01")),
        (String::from("html"), String::from("<p>Hi world.</p>\n"))
    ]);

    let expected_data_item_2: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Bye, World")),
        (String::from("date"), String::from("2022-01-01")),
        (String::from("html"), String::from("<p>Bye world.</p>\n"))
    ]);
    
    let expected = Vec::from([
        expected_data_item_1, 
        expected_data_item_2,
    ]);

    let result = store
        .from_collection("demo")
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn get_collection_where_test() {
    let root_dir = env::current_dir().unwrap();

    let store = Siena::default().set_directory(&format!("{}{}", root_dir.display().to_string().as_str(), "/test_data"));

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
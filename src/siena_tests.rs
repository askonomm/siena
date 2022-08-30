use std::collections::HashMap;
use crate::siena::Siena;

#[test]
fn get_collection_test() {
    let store = Siena::default().set_directory("./test_data");

    let data_item_1: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Hello, world")),
        (String::from("date"), String::from("2020-01-01"))
    ]);

    let data_item_2: HashMap<String, String> = HashMap::from([
        (String::from("title"), String::from("Goodbye, world")),
        (String::from("date"), String::from("2022-01-01"))
    ]);
    
    let data = Vec::from([data_item_1, data_item_2]);

    assert_eq!(store.collection("demo").get_all(), data);
}

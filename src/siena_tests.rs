use crate::providers::local::LocalProvider;
use crate::siena::{siena, Record, RecordData, RecordSortOrder};
use std::{collections::HashMap, env};

fn record_1() -> Record {
    Record {
        id: String::from("test"),
        collection: String::from("demo"),
        file_name: String::from("test.yaml"),
        data: HashMap::from([
            (
                String::from("title"),
                RecordData::Str(String::from("Bye, world")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("2022-09-10")),
            ),
        ]),
    }
}

fn record_2() -> Record {
    Record {
        id: String::from("test2"),
        collection: String::from("demo"),
        file_name: String::from("test2.yml"),
        data: HashMap::from([
            (
                String::from("title"),
                RecordData::Str(String::from("Hello, world")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("2022-09-09")),
            ),
        ]),
    }
}

fn record_3() -> Record {
    Record {
        id: String::from("2"),
        collection: String::from("demo"),
        file_name: String::from("2.yml"),
        data: HashMap::from([
            (
                String::from("title"),
                RecordData::Str(String::from("Bye, World")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("2022-01-01")),
            ),
        ]),
    }
}

fn record_4() -> Record {
    Record {
        id: String::from("1"),
        collection: String::from("demo"),
        file_name: String::from("1.yml"),
        data: HashMap::from([
            (
                String::from("title"),
                RecordData::Str(String::from("Hello, World")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("2020-01-01")),
            ),
        ]),
    }
}

fn record_5() -> Record {
    Record {
        id: String::from("3"),
        collection: String::from("demo"),
        file_name: String::from("3.yml"),
        data: HashMap::from([
            (
                String::from("special-item"),
                RecordData::Str(String::from("true")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("1992-09-17")),
            ),
        ]),
    }
}

fn record_6() -> Record {
    Record {
        id: String::from("markdown"),
        collection: String::from("demo"),
        file_name: String::from("markdown.md"),
        data: HashMap::from([
            (
                String::from("title"),
                RecordData::Str(String::from("Hello, Markdown")),
            ),
            (
                String::from("slug"),
                RecordData::Str(String::from("hello-markdown")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("2023-10-20")),
            ),
            (
                String::from("status"),
                RecordData::Str(String::from("published")),
            ),
            (
                String::from("content"),
                RecordData::Str(String::from("<p>:)</p>\n<hr />\n<p>:)</p>\n")),
            ),
            (
                String::from("content_raw"),
                RecordData::Str(String::from(":)\n\n---\n\n:)")),
            ),
        ]),
    }
}

// #[test]
// fn sort_test() {
//     let root_dir = env::current_dir().unwrap();
//     let local_dir = format!(
//         "{}{}",
//         root_dir.display().to_string().as_str(),
//         "/test_data"
//     );
//     let provider = LocalProvider {
//         directory: local_dir,
//     };
//     let store = siena(provider);

//     let expected = Vec::from([record_1(), record_2(), record_3(), record_4(), record_5()]);

//     let result = store
//         .collection("demo")
//         .sort("date", RecordSortOrder::Desc)
//         .get_all();

//     assert_eq!(result, expected);
// }

#[test]
fn when_is_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store
        .collection("demo")
        .when_is("title", "Hello, World")
        .get_all();

    assert_eq!(result, Vec::from([record_4()]));
}

#[test]
fn when_is_md_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store.collection("demo").when_is("id", "markdown").get_all();

    assert_eq!(result, Vec::from([record_6()]));
}

#[test]
fn when_is_not_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store
        .collection("demo")
        .when_is_not("date", "2022-01-01")
        .when_is_not("date", "1992-09-17")
        .when_is_not("date", "2022-09-09")
        .when_is_not("date", "2022-09-10")
        .when_is_not("date", "2023-10-20")
        .sort("date", RecordSortOrder::Desc)
        .get_first()
        .unwrap();

    assert_eq!(result, record_4());
}

#[test]
fn when_has_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store.collection("demo").when_has("special-item").get_all();

    assert_eq!(result, Vec::from([record_5()]));
}

#[test]
fn when_has_not_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let expected = Vec::from([record_6(), record_1(), record_2(), record_3(), record_4()]);

    let result = store
        .collection("demo")
        .when_has_not("special-item")
        .sort("date", RecordSortOrder::Desc)
        .get_all();

    assert_eq!(result, expected);
}

#[test]
fn when_matches_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store
        .collection("demo")
        .when_matches("date", r"1992")
        .get_all();

    assert_eq!(result, Vec::from([record_5()]));
}

#[test]
fn limit_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store
        .collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .limit(1)
        .get_all();

    assert_eq!(result, Vec::from([record_6()]));
}

#[test]
fn offset_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store
        .collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .offset(2)
        .limit(1)
        .get_all();

    assert_eq!(result, Vec::from([record_2()]));
}

#[test]
fn offset_out_of_bounds_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };
    let store = siena(provider);

    let result = store
        .collection("demo")
        .sort("date", RecordSortOrder::Desc)
        .offset(6)
        .get_all();

    assert_eq!(result, Vec::new());
}

#[test]
fn update_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };

    siena(provider.clone())
        .collection("demo")
        .when_is("date", "1992-09-17")
        .set(Vec::from([(
            "special-item",
            &RecordData::Str("false".to_string()),
        )]));

    let result = siena(provider.clone())
        .collection("demo")
        .when_is("date", "1992-09-17")
        .get_first()
        .unwrap();

    let expected = Record {
        id: String::from("3"),
        collection: String::from("demo"),
        file_name: String::from("3.yml"),
        data: HashMap::from([
            (
                String::from("special-item"),
                RecordData::Str(String::from("false")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("1992-09-17")),
            ),
        ]),
    };

    assert_eq!(result, expected);

    siena(provider.clone())
        .collection("demo")
        .when_is("date", "1992-09-17")
        .set(Vec::from([(
            "special-item",
            &RecordData::Str("true".to_string()),
        )]));

    let result_again = siena(provider.clone())
        .collection("demo")
        .when_is("date", "1992-09-17")
        .get_first()
        .unwrap();

    let expected_again = Record {
        id: String::from("3"),
        collection: String::from("demo"),
        file_name: String::from("3.yml"),
        data: HashMap::from([
            (
                String::from("special-item"),
                RecordData::Str(String::from("true")),
            ),
            (
                String::from("date"),
                RecordData::Str(String::from("1992-09-17")),
            ),
        ]),
    };

    assert!(result.eq(&expected) && result_again.eq(&expected_again))
}

#[test]
fn create_test() {
    let root_dir = env::current_dir().unwrap();
    let local_dir = format!(
        "{}{}",
        root_dir.display().to_string().as_str(),
        "/test_data"
    );
    let provider = LocalProvider {
        directory: local_dir,
    };

    siena(provider.clone())
        .create("demo2", "test3")
        .set(Vec::from([(
            "title",
            &RecordData::Str("Title goes here".to_string()),
        )]));

    let result = siena(provider.clone())
        .collection("demo2")
        .get_first()
        .unwrap();

    let expected = Record {
        id: String::from("test3"),
        collection: String::from("demo2"),
        file_name: String::from("test3.yml"),
        data: HashMap::from([(
            String::from("title"),
            RecordData::Str(String::from("Title goes here")),
        )]),
    };

    siena(provider.clone())
        .collection("demo2")
        .when_is("id", "test3")
        .delete();

    assert_eq!(result, expected);
}

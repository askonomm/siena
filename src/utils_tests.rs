use crate::utils::str_ends_with_any;

#[test]
fn string_ends_with_any_test() {
    assert_eq!(str_ends_with_any("o.yml", Vec::from(["yml"])), true);

    assert_eq!(str_ends_with_any("o.yml", Vec::from(["md", "yml"])), true);

    assert_eq!(str_ends_with_any("o.yml2", Vec::from(["md", "yml"])), false);
}

pub fn str_ends_with_any(s: &str, suffixes: Vec<&str>) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}

#[test]
fn string_ends_with_any_test() {
    assert_eq!(str_ends_with_any("o.yml", Vec::from(["yml"])), true);
    assert_eq!(str_ends_with_any("o.yml", Vec::from(["md", "yml"])), true);
    assert_eq!(str_ends_with_any("o.yml2", Vec::from(["md", "yml"])), false);
    assert_eq!(str_ends_with_any("o.yml2", Vec::from(["md"])), false);
    assert_eq!(str_ends_with_any("o.yml2", Vec::from(["yml"])), false);
    assert_eq!(str_ends_with_any("o.yml2", Vec::from(["yml2"])), true);
}

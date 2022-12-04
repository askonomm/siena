pub fn string_ends_with_any(s: &str, suffixes: Vec<&str>) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}
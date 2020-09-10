#[test]
fn lowercase() {
    let s: &'static str = const_str::to_lowercase!("AsD");
    assert_eq!(s, "asd");
}

#[test]
fn uppercase() {
    let s: &'static str = const_str::to_uppercase!("aSd");
    assert_eq!(s, "ASD");
}

#[test]
fn replace() {
    let s: &'static str = const_str::replace!("a_d", "_", "-");
    assert_eq!(s, "a-d");
}

#[cfg(feature = "regex")]
#[test]
fn regex() {
    use regex::Regex;
    let re = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
    assert!(Regex::new(re).is_ok());
}

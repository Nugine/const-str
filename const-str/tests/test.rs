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

#[cfg(feature = "verify-regex")]
#[test]
fn regex() {
    use regex::Regex;
    let re: &'static str = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
    assert!(Regex::new(re).is_ok());

    const_str::regex_assert_match!(r"^\d{4}-\d{2}-\d{2}$", "2014-01-01");
}

#[cfg(feature = "verify-http")]
#[test]
fn http() {
    use http::header::HeaderName;
    let name: &'static str = const_str::verified_header_name!("content-md5");
    assert_eq!(HeaderName::from_static(name).as_str(), "content-md5");
}

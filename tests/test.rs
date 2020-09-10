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

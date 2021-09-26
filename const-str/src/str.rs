pub const fn equal(lhs: &str, rhs: &str) -> bool {
    crate::bytes::equal(lhs.as_bytes(), rhs.as_bytes())
}

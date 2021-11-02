pub const fn equal(lhs: &str, rhs: &str) -> bool {
    crate::bytes::equal(lhs.as_bytes(), rhs.as_bytes())
}

// https://github.com/rust-lang/rust/issues/89259
#[allow(unsafe_code, clippy::transmute_int_to_char)]
pub const unsafe fn char_from_u32(x: u32) -> char {
    core::mem::transmute(x)
}

pub const fn contains(haystack: &str, needle: &str) -> bool {
    crate::bytes::contains(haystack.as_bytes(), needle.as_bytes())
}

pub const fn starts_with(haystack: &str, needle: &str) -> bool {
    crate::bytes::starts_with(haystack.as_bytes(), needle.as_bytes())
}

pub const fn ends_with(haystack: &str, needle: &str) -> bool {
    crate::bytes::ends_with(haystack.as_bytes(), needle.as_bytes())
}

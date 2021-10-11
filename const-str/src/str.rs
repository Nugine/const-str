pub const fn equal(lhs: &str, rhs: &str) -> bool {
    crate::bytes::equal(lhs.as_bytes(), rhs.as_bytes())
}

// https://github.com/rust-lang/rust/issues/89259
#[allow(unsafe_code, clippy::transmute_int_to_char)]
pub const unsafe fn char_from_u32(x: u32) -> char {
    core::mem::transmute(x)
}

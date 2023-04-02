#![allow(unsafe_code)]

use core::cmp::Ordering;

use crate::slice::advance;

pub const fn equal(lhs: &str, rhs: &str) -> bool {
    crate::bytes::equal(lhs.as_bytes(), rhs.as_bytes())
}

pub const fn compare(lhs: &str, rhs: &str) -> Ordering {
    crate::bytes::compare(lhs.as_bytes(), rhs.as_bytes())
}

pub const unsafe fn char_from_u32(x: u32) -> char {
    #[cfg(not(feature = "unstable"))]
    #[allow(clippy::transmute_int_to_char)]
    {
        core::mem::transmute(x)
    }
    #[cfg(feature = "unstable")] // feature(const_char_from_u32_unchecked)
    {
        core::char::from_u32_unchecked(x)
    }
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

pub const fn strip_prefix<'s>(s: &'s str, prefix: &str) -> Option<&'s str> {
    match crate::bytes::strip_prefix(s.as_bytes(), prefix.as_bytes()) {
        Some(remain) => Some(unsafe { core::str::from_utf8_unchecked(remain) }),
        None => None,
    }
}

pub const fn strip_suffix<'s>(s: &'s str, suffix: &str) -> Option<&'s str> {
    match crate::bytes::strip_suffix(s.as_bytes(), suffix.as_bytes()) {
        Some(remain) => Some(unsafe { core::str::from_utf8_unchecked(remain) }),
        None => None,
    }
}

pub const fn next_match<'h>(haystack: &'h str, needle: &str) -> Option<(usize, &'h str)> {
    assert!(!needle.is_empty());

    let lhs = haystack.as_bytes();
    let rhs = needle.as_bytes();

    let mut i = 0;
    while i + rhs.len() <= lhs.len() {
        let mut j = 0;
        while j < rhs.len() {
            if lhs[i + j] != rhs[j] {
                break;
            }
            j += 1;
        }
        if j == rhs.len() {
            let remain = advance(lhs, i + rhs.len());
            let remain = unsafe { core::str::from_utf8_unchecked(remain) };
            return Some((i, remain));
        }

        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_match() {
        assert_eq!(next_match("abc", "ab"), Some((0, "c")));
        assert_eq!(next_match("abc", "bc"), Some((1, "")));
        assert_eq!(next_match("abc", "c"), Some((2, "")));
        assert_eq!(next_match("abc", "d"), None);
    }
}

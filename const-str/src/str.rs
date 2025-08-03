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
    #[allow(unnecessary_transmutes)]
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

/// Returns true if the byte is an ASCII whitespace character.
/// ASCII whitespace: space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), carriage return (0x0D).
const fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
}

/// Trims ASCII whitespace from both ends of a string slice.
pub const fn trim_ascii<'s>(s: &'s str) -> &'s str {
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    // Find start
    let mut start = 0;
    while start < len && is_ascii_whitespace(bytes[start]) {
        start += 1;
    }
    
    // Find end
    let mut end = len;
    while end > start && is_ascii_whitespace(bytes[end - 1]) {
        end -= 1;
    }
    
    let trimmed_bytes = crate::slice::subslice(bytes, start..end);
    unsafe { core::str::from_utf8_unchecked(trimmed_bytes) }
}

/// Trims ASCII whitespace from the start of a string slice.
pub const fn trim_ascii_start<'s>(s: &'s str) -> &'s str {
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    // Find start
    let mut start = 0;
    while start < len && is_ascii_whitespace(bytes[start]) {
        start += 1;
    }
    
    let trimmed_bytes = crate::slice::advance(bytes, start);
    unsafe { core::str::from_utf8_unchecked(trimmed_bytes) }
}

/// Trims ASCII whitespace from the end of a string slice.
pub const fn trim_ascii_end<'s>(s: &'s str) -> &'s str {
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    // Find end
    let mut end = len;
    while end > 0 && is_ascii_whitespace(bytes[end - 1]) {
        end -= 1;
    }
    
    let trimmed_bytes = crate::slice::subslice(bytes, 0..end);
    unsafe { core::str::from_utf8_unchecked(trimmed_bytes) }
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

    #[test]
    fn test_trim_ascii() {
        assert_eq!(trim_ascii("  hello world  "), "hello world");
        assert_eq!(trim_ascii("\t\n  hello\tworld\n  \r"), "hello\tworld");
        assert_eq!(trim_ascii("   "), "");
        assert_eq!(trim_ascii("hello"), "hello");
        assert_eq!(trim_ascii(""), "");
    }

    #[test]
    fn test_trim_ascii_start() {
        assert_eq!(trim_ascii_start("  hello world  "), "hello world  ");
        assert_eq!(trim_ascii_start("\t\n  hello\tworld"), "hello\tworld");
        assert_eq!(trim_ascii_start("hello"), "hello");
        assert_eq!(trim_ascii_start(""), "");
        assert_eq!(trim_ascii_start("   "), "");
    }

    #[test]
    fn test_trim_ascii_end() {
        assert_eq!(trim_ascii_end("  hello world  "), "  hello world");
        assert_eq!(trim_ascii_end("hello\tworld\n  \r"), "hello\tworld");
        assert_eq!(trim_ascii_end("hello"), "hello");
        assert_eq!(trim_ascii_end(""), "");
        assert_eq!(trim_ascii_end("   "), "");
    }
}

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

pub const fn next_match_char_slice<'h>(haystack: &'h str, chars: &[char]) -> Option<(usize, &'h str)> {
    let mut input_bytes = haystack.as_bytes();
    let mut byte_offset = 0;

    while !input_bytes.is_empty() {
        if let Some((ch, char_byte_len)) = crate::utf8::next_char(input_bytes) {
            // Check if this character matches any in the slice
            let mut i = 0;
            while i < chars.len() {
                if ch == chars[i] {
                    let remain = advance(haystack.as_bytes(), byte_offset + char_byte_len);
                    let remain = unsafe { core::str::from_utf8_unchecked(remain) };
                    return Some((byte_offset, remain));
                }
                i += 1;
            }

            input_bytes = advance(input_bytes, char_byte_len);
            byte_offset += char_byte_len;
        } else {
            // Invalid UTF-8 or end of string
            break;
        }
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

    #[test]
    fn test_next_match_char_slice() {
        assert_eq!(next_match_char_slice("abc", &['a']), Some((0, "bc")));
        assert_eq!(next_match_char_slice("abc", &['b']), Some((1, "c")));
        assert_eq!(next_match_char_slice("abc", &['c']), Some((2, "")));
        assert_eq!(next_match_char_slice("abc", &['d']), None);
        assert_eq!(next_match_char_slice("abc", &['a', 'c']), Some((0, "bc")));
        assert_eq!(next_match_char_slice("abc", &['b', 'c']), Some((1, "c")));
        assert_eq!(next_match_char_slice("hello,world;test", &[',', ';']), Some((5, "world;test")));
        assert_eq!(next_match_char_slice("hello,world;test", &[';', ',']), Some((5, "world;test")));
        assert_eq!(next_match_char_slice("无字符", &['字']), Some((3, "符")));
    }
}

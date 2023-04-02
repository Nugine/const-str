use crate::slice::subslice;

use core::cmp::Ordering;

pub const fn clone<const N: usize>(bytes: &[u8]) -> [u8; N] {
    assert!(bytes.len() == N);
    let mut buf = [0; N];
    let mut i = 0;
    while i < bytes.len() {
        buf[i] = bytes[i];
        i += 1;
    }
    buf
}

pub const fn equal(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub const fn compare(lhs: &[u8], rhs: &[u8]) -> Ordering {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let min_len = if lhs_len < rhs_len { lhs_len } else { rhs_len };

    let mut i = 0;
    while i < min_len {
        if lhs[i] < rhs[i] {
            return Ordering::Less;
        }
        if lhs[i] > rhs[i] {
            return Ordering::Greater;
        }
        i += 1;
    }

    if lhs_len < rhs_len {
        Ordering::Less
    } else if lhs_len > rhs_len {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub const fn merge<const N: usize>(mut buf: [u8; N], bytes: &[u8]) -> [u8; N] {
    assert!(N <= bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        buf[i] = bytes[i];
        i += 1;
    }
    buf
}

pub const fn reversed<const N: usize>(mut arr: [u8; N]) -> [u8; N] {
    let mut i = 0;
    while i * 2 < N {
        let a = arr[i];
        let b = arr[N - 1 - i];
        arr[i] = b;
        arr[N - 1 - i] = a;
        i += 1;
    }
    arr
}

pub const fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    let haystack_len = haystack.len();
    let needle_len = needle.len();

    let mut i = 0;
    while i < haystack_len {
        let mut j = 0;
        while j < needle_len && i + j < haystack_len {
            if haystack[i + j] != needle[j] {
                break;
            }
            j += 1;
        }
        if j == needle_len {
            return true;
        }
        i += 1;
    }

    false
}

pub const fn starts_with(haystack: &[u8], needle: &[u8]) -> bool {
    let haystack_len = haystack.len();
    let needle_len = needle.len();

    if needle_len > haystack_len {
        return false;
    }

    let mut i = 0;
    while i < needle_len {
        if haystack[i] != needle[i] {
            break;
        }
        i += 1
    }

    i == needle_len
}

pub const fn ends_with(haystack: &[u8], needle: &[u8]) -> bool {
    let haystack_len = haystack.len();
    let needle_len = needle.len();

    if needle_len > haystack_len {
        return false;
    }

    let mut i = 0;
    while i < needle_len {
        if haystack[haystack_len - needle_len + i] != needle[i] {
            break;
        }
        i += 1
    }

    i == needle_len
}

pub const fn strip_prefix<'s>(s: &'s [u8], prefix: &[u8]) -> Option<&'s [u8]> {
    if starts_with(s, prefix) {
        Some(subslice(s, prefix.len()..s.len()))
    } else {
        None
    }
}

pub const fn strip_suffix<'s>(s: &'s [u8], suffix: &[u8]) -> Option<&'s [u8]> {
    if ends_with(s, suffix) {
        Some(subslice(s, 0..s.len() - suffix.len()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reversed() {
        let arr = [0, 1];
        assert_eq!(reversed(arr), [1, 0]);

        let arr = [0, 1, 2];
        assert_eq!(reversed(arr), [2, 1, 0]);
    }

    #[test]
    fn test_contains() {
        macro_rules! test_contains {
            (true, $haystack: expr, $needle: expr) => {
                assert!(contains($haystack.as_ref(), $needle.as_ref()));
            };
            (false, $haystack: expr, $needle: expr) => {
                assert!(!contains($haystack.as_ref(), $needle.as_ref()));
            };
        }

        let buf = b"abcdefgh";
        test_contains!(true, buf, b"");
        test_contains!(true, buf, b"a");
        test_contains!(true, buf, b"ef");
        test_contains!(false, buf, b"xyz");

        test_contains!(true, "asd", "");
        test_contains!(true, "asd", "a");
        test_contains!(true, "asdf", "sd");
        test_contains!(false, "", "a");
        test_contains!(false, "asd", "abcd");

        test_contains!(true, "唐可可", "可");
        test_contains!(true, "Liyuu", "i");
        test_contains!(false, "Liyuu", "我");
    }

    #[test]
    fn test_starts_with() {
        assert!(starts_with(b"", b""));
        assert!(starts_with(b"a", b""));
        assert!(starts_with(b"a", b"a"));
        assert!(!starts_with(b"", b"a"));
        assert!(!starts_with(b"ba", b"a"));
    }

    #[test]
    fn test_ends_with() {
        assert!(ends_with(b"", b""));
        assert!(ends_with(b"a", b""));
        assert!(ends_with(b"a", b"a"));
        assert!(!ends_with(b"", b"a"));
        assert!(!ends_with(b"ab", b"a"));
    }
}

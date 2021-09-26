use crate::utf8::CharEncodeUtf8;

pub struct Contains<'a, P>(pub &'a str, pub P);

impl<'a, 'b> Contains<'a, &'b str> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let needle = self.1.as_bytes();
        bytes_contains(haystack, needle)
    }
}

impl<'a> Contains<'a, char> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let ch = CharEncodeUtf8::new(self.1);
        let needle = ch.as_bytes();
        bytes_contains(haystack, needle)
    }
}

const fn bytes_contains(haystack: &[u8], needle: &[u8]) -> bool {
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

#[test]
fn test_bytes_contains() {
    let buf = b"abcdefgh";
    assert!(bytes_contains(buf, b""));
    assert!(bytes_contains(buf, b"a"));
    assert!(bytes_contains(buf, b"ef"));
    assert!(!bytes_contains(buf, b"xyz"));
}

#[test]
fn test_contains() {
    macro_rules! test_contains {
        (true, $haystack: expr, $needle: expr) => {
            assert!(Contains($haystack, $needle).const_eval());
        };
        (false, $haystack: expr, $needle: expr) => {
            assert!(!Contains($haystack, $needle).const_eval());
        };
    }

    test_contains!(true, "asd", "");
    test_contains!(true, "asd", "a");
    test_contains!(true, "asdf", "sd");
    test_contains!(false, "", "a");
    test_contains!(false, "asd", "abcd");

    test_contains!(true, "唐可可", '可');
    test_contains!(true, "Liyuu", 'i');
    test_contains!(false, "Liyuu", '我');
}

/// Returns [`true`] if the given pattern matches a sub-slice of this string slice.
///
/// Returns [`false`] if it does not.
///
/// The pattern type must be one of
///
/// + [`&str`]
/// + [`char`]
///
/// # Examples
///
/// ```
/// const BANANAS: &str = "bananas";
/// const A: bool = const_str::contains!(BANANAS, "nana");
/// const B: bool = const_str::contains!(BANANAS, "apples");
/// const C: bool = const_str::contains!(BANANAS, 'c');
/// assert_eq!([A, B, C], [true, false, false]);
/// ```
///
#[macro_export]
macro_rules! contains {
    ($haystack: expr, $pattern: expr) => {{
        $crate::__ctfe::Contains($haystack, $pattern).const_eval()
    }};
}

pub struct StartsWith<'a, P>(pub &'a str, pub P);

impl<'a, 'b> StartsWith<'a, &'b str> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let needle = self.1.as_bytes();
        bytes_starts_with(haystack, needle)
    }
}

impl<'a> StartsWith<'a, char> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let ch = CharEncodeUtf8::new(self.1);
        let needle = ch.as_bytes();
        bytes_starts_with(haystack, needle)
    }
}

const fn bytes_starts_with(haystack: &[u8], needle: &[u8]) -> bool {
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

#[test]
fn test_bytes_starts_with() {
    assert!(bytes_starts_with(b"", b""));
    assert!(bytes_starts_with(b"a", b""));
    assert!(bytes_starts_with(b"a", b"a"));
    assert!(!bytes_starts_with(b"", b"a"));
    assert!(!bytes_starts_with(b"ba", b"a"));
}

/// Returns [`true`] if the given pattern matches a prefix of this string slice.
///
/// Returns [`false`] if it does not.
///
/// The pattern type must be one of
///
/// + [`&str`]
/// + [`char`]
///
/// # Examples
///
/// ```
/// const BANANAS: &str = "bananas";
/// const A: bool = const_str::starts_with!(BANANAS, "bana");
/// const B: bool = const_str::starts_with!(BANANAS, "nana");
/// const C: bool = const_str::starts_with!(BANANAS, 'b');
/// assert_eq!([A, B, C], [true, false, true]);
/// ```
///
#[macro_export]
macro_rules! starts_with {
    ($haystack: expr, $pattern: expr) => {{
        $crate::__ctfe::StartsWith($haystack, $pattern).const_eval()
    }};
}

pub struct EndsWith<'a, P>(pub &'a str, pub P);

impl<'a, 'b> EndsWith<'a, &'b str> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let needle = self.1.as_bytes();
        bytes_ends_with(haystack, needle)
    }
}

impl<'a> EndsWith<'a, char> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let ch = CharEncodeUtf8::new(self.1);
        let needle = ch.as_bytes();
        bytes_ends_with(haystack, needle)
    }
}

const fn bytes_ends_with(haystack: &[u8], needle: &[u8]) -> bool {
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

#[test]
fn test_bytes_ends_with() {
    assert!(bytes_ends_with(b"", b""));
    assert!(bytes_ends_with(b"a", b""));
    assert!(bytes_ends_with(b"a", b"a"));
    assert!(!bytes_ends_with(b"", b"a"));
    assert!(!bytes_ends_with(b"ab", b"a"));
}

/// Returns [`true`] if the given pattern matches a suffix of this string slice.
///
/// Returns [`false`] if it does not.
///
/// The pattern type must be one of
///
/// + [`&str`]
/// + [`char`]
///
/// # Examples
///
/// ```
/// const BANANAS: &str = "bananas";
/// const A: bool = const_str::ends_with!(BANANAS, "anas");
/// const B: bool = const_str::ends_with!(BANANAS, "nana");
/// const C: bool = const_str::ends_with!(BANANAS, 's');
/// assert_eq!([A, B, C], [true, false, true]);
/// ```
///
#[macro_export]
macro_rules! ends_with {
    ($haystack: expr, $pattern: expr) => {{
        $crate::__ctfe::EndsWith($haystack, $pattern).const_eval()
    }};
}

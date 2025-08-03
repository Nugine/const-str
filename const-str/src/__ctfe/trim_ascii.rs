#![allow(unsafe_code)]

use crate::__ctfe::StrBuf;

/// Returns true if the byte is an ASCII whitespace character.
/// ASCII whitespace: space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), carriage return (0x0D).
const fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
}

/// Finds the start index after trimming ASCII whitespace from the beginning.
const fn find_trim_start(bytes: &[u8]) -> usize {
    let mut start = 0;
    while start < bytes.len() && is_ascii_whitespace(bytes[start]) {
        start += 1;
    }
    start
}

/// Finds the end index after trimming ASCII whitespace from the end.
const fn find_trim_end(bytes: &[u8]) -> usize {
    let mut end = bytes.len();
    while end > 0 && is_ascii_whitespace(bytes[end - 1]) {
        end -= 1;
    }
    end
}

/// Trims ASCII whitespace from both ends of a string slice.
pub const fn trim_ascii_str(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let start = find_trim_start(bytes);
    let end = find_trim_end(bytes);

    // Ensure start <= end
    let end = if start > end { start } else { end };

    (start, end)
}

/// Trims ASCII whitespace from the start of a string slice.
pub const fn trim_ascii_start_str(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let start = find_trim_start(bytes);
    (start, bytes.len())
}

/// Trims ASCII whitespace from the end of a string slice.
pub const fn trim_ascii_end_str(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let end = find_trim_end(bytes);
    (0, end)
}

/// Creates a StrBuf from a substring defined by start and end indices.
const fn create_str_buf<const N: usize>(s: &str, start: usize, end: usize) -> StrBuf<N> {
    let bytes = s.as_bytes();
    assert!(start <= end);
    assert!(end <= bytes.len());
    assert!(end - start == N);

    let mut buf = [0; N];
    let mut i = 0;
    while i < N {
        buf[i] = bytes[start + i];
        i += 1;
    }

    unsafe { StrBuf::new_unchecked(buf) }
}

pub struct TrimAscii<T>(pub T);

impl TrimAscii<&str> {
    pub const fn output_len(&self) -> usize {
        let (start, end) = trim_ascii_str(self.0);
        end - start
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let (start, end) = trim_ascii_str(self.0);
        create_str_buf::<N>(self.0, start, end)
    }
}

pub struct TrimAsciiStart<T>(pub T);

impl TrimAsciiStart<&str> {
    pub const fn output_len(&self) -> usize {
        let (start, end) = trim_ascii_start_str(self.0);
        end - start
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let (start, end) = trim_ascii_start_str(self.0);
        create_str_buf::<N>(self.0, start, end)
    }
}

pub struct TrimAsciiEnd<T>(pub T);

impl TrimAsciiEnd<&str> {
    pub const fn output_len(&self) -> usize {
        let (start, end) = trim_ascii_end_str(self.0);
        end - start
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let (start, end) = trim_ascii_end_str(self.0);
        create_str_buf::<N>(self.0, start, end)
    }
}

/// Trims ASCII whitespace from both ends of a string.
///
/// ASCII whitespace characters are space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), and carriage return (0x0D).
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// const TRIMMED: &str = const_str::trim_ascii!("  hello world  ");
/// assert_eq!(TRIMMED, "hello world");
///
/// const MIXED: &str = const_str::trim_ascii!("\t\n  hello\tworld\n  \r");
/// assert_eq!(MIXED, "hello\tworld");
///
/// const EMPTY: &str = const_str::trim_ascii!("   ");
/// assert_eq!(EMPTY, "");
/// ```
#[macro_export]
macro_rules! trim_ascii {
    ($s: expr) => {{
        const INPUT: &str = $s;
        const N: usize = $crate::__ctfe::TrimAscii(INPUT).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<N> =
            $crate::__ctfe::TrimAscii(INPUT).const_eval::<N>();
        OUTPUT_BUF.as_str()
    }};
}

/// Trims ASCII whitespace from the start of a string.
///
/// ASCII whitespace characters are space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), and carriage return (0x0D).
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// const TRIMMED: &str = const_str::trim_ascii_start!("  hello world  ");
/// assert_eq!(TRIMMED, "hello world  ");
///
/// const MIXED: &str = const_str::trim_ascii_start!("\t\n  hello\tworld");
/// assert_eq!(MIXED, "hello\tworld");
///
/// const NO_WHITESPACE: &str = const_str::trim_ascii_start!("hello");
/// assert_eq!(NO_WHITESPACE, "hello");
/// ```
#[macro_export]
macro_rules! trim_ascii_start {
    ($s: expr) => {{
        const INPUT: &str = $s;
        const N: usize = $crate::__ctfe::TrimAsciiStart(INPUT).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<N> =
            $crate::__ctfe::TrimAsciiStart(INPUT).const_eval::<N>();
        OUTPUT_BUF.as_str()
    }};
}

/// Trims ASCII whitespace from the end of a string.
///
/// ASCII whitespace characters are space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), and carriage return (0x0D).
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// const TRIMMED: &str = const_str::trim_ascii_end!("  hello world  ");
/// assert_eq!(TRIMMED, "  hello world");
///
/// const MIXED: &str = const_str::trim_ascii_end!("hello\tworld\n  \r");
/// assert_eq!(MIXED, "hello\tworld");
///
/// const NO_WHITESPACE: &str = const_str::trim_ascii_end!("hello");
/// assert_eq!(NO_WHITESPACE, "hello");
/// ```
#[macro_export]
macro_rules! trim_ascii_end {
    ($s: expr) => {{
        const INPUT: &str = $s;
        const N: usize = $crate::__ctfe::TrimAsciiEnd(INPUT).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<N> =
            $crate::__ctfe::TrimAsciiEnd(INPUT).const_eval::<N>();
        OUTPUT_BUF.as_str()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ascii_whitespace() {
        assert!(is_ascii_whitespace(b' '));
        assert!(is_ascii_whitespace(b'\t'));
        assert!(is_ascii_whitespace(b'\n'));
        assert!(is_ascii_whitespace(b'\x0B')); // vertical tab
        assert!(is_ascii_whitespace(b'\x0C')); // form feed
        assert!(is_ascii_whitespace(b'\r'));

        assert!(!is_ascii_whitespace(b'a'));
        assert!(!is_ascii_whitespace(b'0'));
        assert!(!is_ascii_whitespace(b'!'));
        assert!(!is_ascii_whitespace(b'\x00'));
    }

    #[test]
    fn test_trim_ascii() {
        const S1: &str = trim_ascii!("  hello world  ");
        assert_eq!(S1, "hello world");

        const S2: &str = trim_ascii!("\t\n  hello\tworld\n  \r");
        assert_eq!(S2, "hello\tworld");

        const S3: &str = trim_ascii!("   ");
        assert_eq!(S3, "");

        const S4: &str = trim_ascii!("hello");
        assert_eq!(S4, "hello");

        const S5: &str = trim_ascii!("");
        assert_eq!(S5, "");
    }

    #[test]
    fn test_trim_ascii_start() {
        const S1: &str = trim_ascii_start!("  hello world  ");
        assert_eq!(S1, "hello world  ");

        const S2: &str = trim_ascii_start!("\t\n  hello\tworld");
        assert_eq!(S2, "hello\tworld");

        const S3: &str = trim_ascii_start!("hello");
        assert_eq!(S3, "hello");

        const S4: &str = trim_ascii_start!("");
        assert_eq!(S4, "");

        const S5: &str = trim_ascii_start!("   ");
        assert_eq!(S5, "");
    }

    #[test]
    fn test_trim_ascii_end() {
        const S1: &str = trim_ascii_end!("  hello world  ");
        assert_eq!(S1, "  hello world");

        const S2: &str = trim_ascii_end!("hello\tworld\n  \r");
        assert_eq!(S2, "hello\tworld");

        const S3: &str = trim_ascii_end!("hello");
        assert_eq!(S3, "hello");

        const S4: &str = trim_ascii_end!("");
        assert_eq!(S4, "");

        const S5: &str = trim_ascii_end!("   ");
        assert_eq!(S5, "");
    }

    #[test]
    fn test_edge_cases() {
        // Test with all types of ASCII whitespace
        const ALL_WS: &str = trim_ascii!(" \t\n\x0B\x0C\r");
        assert_eq!(ALL_WS, "");

        // Test with whitespace in the middle (should be preserved)
        const MIDDLE_WS: &str = trim_ascii!("  hello world  ");
        assert_eq!(MIDDLE_WS, "hello world");

        // Test with only start whitespace
        const START_WS: &str = trim_ascii!("  hello");
        assert_eq!(START_WS, "hello");

        // Test with only end whitespace
        const END_WS: &str = trim_ascii!("hello  ");
        assert_eq!(END_WS, "hello");
    }
}

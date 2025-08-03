pub struct TrimAscii<T>(pub T);

impl<'a> TrimAscii<&'a str> {
    pub const fn const_eval(&self) -> &'a str {
        crate::str::trim_ascii(self.0)
    }
}

pub struct TrimAsciiStart<T>(pub T);

impl<'a> TrimAsciiStart<&'a str> {
    pub const fn const_eval(&self) -> &'a str {
        crate::str::trim_ascii_start(self.0)
    }
}

pub struct TrimAsciiEnd<T>(pub T);

impl<'a> TrimAsciiEnd<&'a str> {
    pub const fn const_eval(&self) -> &'a str {
        crate::str::trim_ascii_end(self.0)
    }
}

/// Trims ASCII whitespace from both ends of a string.
///
/// ASCII whitespace characters are space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), and carriage return (0x0D).
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
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
///
/// // Works in const functions too
/// const fn process_string(s: &str) -> &str {
///     const_str::trim_ascii!(s)
/// }
/// assert_eq!(process_string("  test  "), "test");
/// ```
#[macro_export]
macro_rules! trim_ascii {
    ($s: expr) => {
        $crate::__ctfe::TrimAscii($s).const_eval()
    };
}

/// Trims ASCII whitespace from the start of a string.
///
/// ASCII whitespace characters are space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), and carriage return (0x0D).
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
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
///
/// // Works in const functions too
/// const fn process_string(s: &str) -> &str {
///     const_str::trim_ascii_start!(s)
/// }
/// assert_eq!(process_string("  test"), "test");
/// ```
#[macro_export]
macro_rules! trim_ascii_start {
    ($s: expr) => {
        $crate::__ctfe::TrimAsciiStart($s).const_eval()
    };
}

/// Trims ASCII whitespace from the end of a string.
///
/// ASCII whitespace characters are space (0x20), tab (0x09), newline (0x0A),
/// vertical tab (0x0B), form feed (0x0C), and carriage return (0x0D).
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
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
///
/// // Works in const functions too
/// const fn process_string(s: &str) -> &str {
///     const_str::trim_ascii_end!(s)
/// }
/// assert_eq!(process_string("test  "), "test");
/// ```
#[macro_export]
macro_rules! trim_ascii_end {
    ($s: expr) => {
        $crate::__ctfe::TrimAsciiEnd($s).const_eval()
    };
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_const_fn_compatibility() {
        const fn process_trim(s: &str) -> &str {
            trim_ascii!(s)
        }
        
        const fn process_trim_start(s: &str) -> &str {
            trim_ascii_start!(s)
        }
        
        const fn process_trim_end(s: &str) -> &str {
            trim_ascii_end!(s)
        }

        assert_eq!(process_trim("  test  "), "test");
        assert_eq!(process_trim_start("  test  "), "test  ");
        assert_eq!(process_trim_end("  test  "), "  test");
    }
}

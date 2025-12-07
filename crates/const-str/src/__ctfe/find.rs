use crate::utf8::CharEncodeUtf8;

pub struct Contains<'a, P>(pub &'a str, pub P);

impl Contains<'_, &str> {
    pub const fn const_eval(&self) -> bool {
        crate::str::contains(self.0, self.1)
    }
}

impl Contains<'_, char> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0;
        let ch = CharEncodeUtf8::new(self.1);
        let needle = ch.as_str();
        crate::str::contains(haystack, needle)
    }
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
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
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

impl StartsWith<'_, &str> {
    pub const fn const_eval(&self) -> bool {
        crate::str::starts_with(self.0, self.1)
    }
}

impl StartsWith<'_, char> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let ch = CharEncodeUtf8::new(self.1);
        let needle = ch.as_bytes();
        crate::bytes::starts_with(haystack, needle)
    }
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
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
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

impl EndsWith<'_, &str> {
    pub const fn const_eval(&self) -> bool {
        crate::str::ends_with(self.0, self.1)
    }
}

impl EndsWith<'_, char> {
    pub const fn const_eval(&self) -> bool {
        let haystack = self.0.as_bytes();
        let ch = CharEncodeUtf8::new(self.1);
        let needle = ch.as_bytes();
        crate::bytes::ends_with(haystack, needle)
    }
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
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
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

pub struct StripPrefix<'a, P>(pub &'a str, pub P);

impl<'a> StripPrefix<'a, &str> {
    pub const fn const_eval(&self) -> Option<&'a str> {
        crate::str::strip_prefix(self.0, self.1)
    }
}

pub struct StripSuffix<'a, P>(pub &'a str, pub P);

impl<'a> StripSuffix<'a, &str> {
    pub const fn const_eval(&self) -> Option<&'a str> {
        crate::str::strip_suffix(self.0, self.1)
    }
}

/// Returns a string slice with the prefix removed.
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
///
/// # Examples
///
/// ```
/// assert_eq!(const_str::strip_prefix!("foo:bar", "foo:"), Some("bar"));
/// assert_eq!(const_str::strip_prefix!("foo:bar", "bar"), None);
/// assert_eq!(const_str::strip_prefix!("foofoo", "foo"), Some("foo"));
///
/// const FOO_BAR: &str = "foo:bar";
/// const BAR: &str = const_str::unwrap!(const_str::strip_prefix!(FOO_BAR, "foo:"));
/// assert_eq!(BAR, "bar");
/// ```
///
#[macro_export]
macro_rules! strip_prefix {
    ($s: expr, $prefix: expr) => {{
        $crate::__ctfe::StripPrefix($s, $prefix).const_eval()
    }};
}

/// Returns a string slice with the suffix removed.
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
///
/// # Examples
///
/// ```
/// assert_eq!(const_str::strip_suffix!("bar:foo", ":foo"), Some("bar"));
/// assert_eq!(const_str::strip_suffix!("bar:foo", "bar"), None);
/// assert_eq!(const_str::strip_suffix!("foofoo", "foo"), Some("foo"));
///
/// const FOO_BAR: &str = "foo:bar";
/// const FOO: &str = const_str::unwrap!(const_str::strip_suffix!(FOO_BAR, ":bar"));
/// assert_eq!(FOO, "foo");
/// ```
///
#[macro_export]
macro_rules! strip_suffix {
    ($s: expr, $suffix: expr) => {{
        $crate::__ctfe::StripSuffix($s, $suffix).const_eval()
    }};
}

#[cfg(test)]
mod tests {
    use crate::unwrap;

    #[test]
    fn test_contains() {
        const BANANAS: &str = "bananas";
        const A: bool = contains!(BANANAS, "nana");
        const B: bool = contains!(BANANAS, "apples");
        const C: bool = contains!(BANANAS, 'c');
        const D: bool = contains!(BANANAS, 'a');

        assert_eq!([A, B, C, D], [true, false, false, true]);

        let f = contains!("hello", "");
        assert!(f);
    }

    #[test]
    fn test_starts_with() {
        const BANANAS: &str = "bananas";
        const A: bool = starts_with!(BANANAS, "bana");
        const B: bool = starts_with!(BANANAS, "nana");
        const C: bool = starts_with!(BANANAS, 'b');
        const D: bool = starts_with!(BANANAS, 'n');

        assert_eq!([A, B, C, D], [true, false, true, false]);

        let f = starts_with!("hello", "");
        assert!(f);
    }

    #[test]
    fn test_ends_with() {
        const BANANAS: &str = "bananas";
        const A: bool = ends_with!(BANANAS, "anas");
        const B: bool = ends_with!(BANANAS, "nana");
        const C: bool = ends_with!(BANANAS, 's');
        const D: bool = ends_with!(BANANAS, 'b');

        assert_eq!([A, B, C, D], [true, false, true, false]);

        let f = ends_with!("hello", "");
        assert!(f);
    }

    #[test]
    fn test_strip_prefix() {
        const R1: Option<&str> = strip_prefix!("foo:bar", "foo:");
        const R2: Option<&str> = strip_prefix!("foo:bar", "bar");
        const R3: Option<&str> = strip_prefix!("foofoo", "foo");
        const R4: Option<&str> = strip_prefix!("", "");

        assert_eq!(R1, Some("bar"));
        assert_eq!(R2, None);
        assert_eq!(R3, Some("foo"));
        assert_eq!(R4, Some(""));

        const FOO_BAR: &str = "foo:bar";
        const BAR: &str = unwrap!(strip_prefix!(FOO_BAR, "foo:"));
        assert_eq!(BAR, "bar");
    }

    #[test]
    fn test_strip_suffix() {
        const R1: Option<&str> = strip_suffix!("bar:foo", ":foo");
        const R2: Option<&str> = strip_suffix!("bar:foo", "bar");
        const R3: Option<&str> = strip_suffix!("foofoo", "foo");
        const R4: Option<&str> = strip_suffix!("", "");

        assert_eq!(R1, Some("bar"));
        assert_eq!(R2, None);
        assert_eq!(R3, Some("foo"));
        assert_eq!(R4, Some(""));

        const FOO_BAR: &str = "foo:bar";
        const FOO: &str = unwrap!(strip_suffix!(FOO_BAR, ":bar"));
        assert_eq!(FOO, "foo");
    }

    #[test]
    fn test_find_runtime() {
        use super::*;

        // Runtime tests for Contains
        let contains1 = Contains("hello world", "world");
        assert!(contains1.const_eval());

        let contains2 = Contains("hello", "x");
        assert!(!contains2.const_eval());

        // Runtime tests for StartsWith
        let starts1 = StartsWith("hello", "he");
        assert!(starts1.const_eval());

        let starts2 = StartsWith("hello", "lo");
        assert!(!starts2.const_eval());

        // Runtime tests for EndsWith
        let ends1 = EndsWith("hello", "lo");
        assert!(ends1.const_eval());

        let ends2 = EndsWith("hello", "he");
        assert!(!ends2.const_eval());

        // Runtime tests for StripPrefix
        let strip_pre = StripPrefix("hello world", "hello ");
        let result = strip_pre.const_eval();
        assert_eq!(result, Some("world"));

        let strip_pre_none = StripPrefix("hello", "world");
        let result_none = strip_pre_none.const_eval();
        assert_eq!(result_none, None);

        // Runtime tests for StripSuffix
        let strip_suf = StripSuffix("hello world", " world");
        let result_suf = strip_suf.const_eval();
        assert_eq!(result_suf, Some("hello"));

        let strip_suf_none = StripSuffix("hello", "world");
        let result_suf_none = strip_suf_none.const_eval();
        assert_eq!(result_suf_none, None);
    }
}

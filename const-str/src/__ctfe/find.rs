use crate::utf8::CharEncodeUtf8;

pub struct Contains<'a, P>(pub &'a str, pub P);

impl<'a, 'b> Contains<'a, &'b str> {
    pub const fn const_eval(&self) -> bool {
        crate::str::contains(self.0, self.1)
    }
}

impl<'a> Contains<'a, char> {
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
        crate::str::starts_with(self.0, self.1)
    }
}

impl<'a> StartsWith<'a, char> {
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
        crate::str::ends_with(self.0, self.1)
    }
}

impl<'a> EndsWith<'a, char> {
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

impl<'a, 'b> StripPrefix<'a, &'b str> {
    pub const fn const_eval(&self) -> Option<&'a str> {
        crate::str::strip_prefix(self.0, self.1)
    }
}

pub struct StripSuffix<'a, P>(pub &'a str, pub P);

impl<'a, 'b> StripSuffix<'a, &'b str> {
    pub const fn const_eval(&self) -> Option<&'a str> {
        crate::str::strip_suffix(self.0, self.1)
    }
}

/// Returns a string slice with the prefix removed.
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

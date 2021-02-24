/// Returns the lowercase equivalent of this string literal, as a new string literal.
///
/// See [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase).
///
/// # Examples
///
/// ```
/// assert_eq!("hello", const_str::to_lowercase!("HELLO"));
/// ```
///
#[macro_export]
macro_rules! to_lowercase {
    ($str:literal) => {
        $crate::__imp::to_lowercase!($str)
    };
}

/// Returns the uppercase equivalent of this string literal, as a new string literal.
///
/// See [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase).
///
/// # Examples
///
/// ```
/// assert_eq!("HELLO", const_str::to_uppercase!("hello"));
/// ```
///
#[macro_export]
macro_rules! to_uppercase {
    ($str:literal) => {
        $crate::__imp::to_uppercase!($str)
    };
}

/// Replaces all matches of a pattern with another string literal.
///
/// See [`str::replace`](https://doc.rust-lang.org/std/primitive.str.html#method.replace).
///
/// # Examples
///
/// ```
/// assert_eq!("this is new", const_str::replace!("this is old", "old", "new"));
/// ```
///
#[macro_export]
macro_rules! replace {
    ($str:literal, $from:literal, $to:literal) => {
        $crate::__imp::replace!($str, $from, $to)
    };
}

/// Converts a string literal to a byte string literal
///
/// # Examples
/// ```
/// let bytes: &'static [u8;4] = const_str::as_bytes!("file");
/// assert_eq!(bytes, b"file");
/// ```
///
#[macro_export]
macro_rules! as_bytes {
    ($str:literal) => {
        $crate::__imp::as_bytes!($str)
    };
}

/// Converts a byte string literal to a string literal
///
/// # Examples
/// ```
/// let name: &'static str = const_str::from_utf8!(b"file");
/// assert_eq!(name, "file");
/// ```
///
#[macro_export]
macro_rules! from_utf8 {
    ($str:literal) => {
        $crate::__imp::from_utf8!($str)
    };
}

/// Returns the length of a string literal or byte string literal
///
/// # Examples
/// ```
/// assert_eq!(const_str::len!("file"), 4_usize);
/// assert_eq!(const_str::len!(b"file\0"), 5_usize);
/// ```
///
#[macro_export]
macro_rules! len {
    ($str:literal) => {
        $crate::__imp::len!($str)
    };
}

/// Converts a string literal into an array of its characters.
///
/// # Examples
/// ```
/// let chars = to_char_array("Hello");
/// assert_eq!(chars[..], ['H', 'e', 'l', 'l', 'o']);
/// ```
///
#[macro_export]
macro_rules! to_char_array {
    ($str:literal) => {
        $crate::imp::to_char_array!($str)
    };
}

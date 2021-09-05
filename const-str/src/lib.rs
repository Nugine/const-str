//! compile-time string operations
//!

#![forbid(unsafe_code)]
#![deny(missing_docs, clippy::all, clippy::cargo)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]
#![no_std]

macro_rules! const_assert {
    ($e:expr) => {
        let _: () = [()][(!($e) as usize)];
    };
}

#[doc(hidden)]
pub mod __proc;

#[doc(hidden)]
pub mod __const;

// -----------------------------------------------------------------------------

/// Returns the length of a string slice or a byte string
///
/// # Examples
/// ```
/// const S: &str = "hello";
/// const B: &[u8; 6] = b"hello\0";
/// assert_eq!(const_str::len!(S), 5_usize);
/// assert_eq!(const_str::len!(B), 6_usize);
/// ```
///
#[macro_export]
macro_rules! len {
    ($s: expr) => {{
        $crate::__const::Len($s).const_eval()
    }};
}

// -----------------------------------------------------------------------------

/// Converts a string slice or a byte string to a byte array.
///
/// # Examples
/// ```
/// const S: &str = "hello";
/// const B: &[u8; 6] = b"hello\0";
/// assert_eq!(const_str::to_byte_array!(S), [b'h', b'e', b'l', b'l', b'o']);
/// assert_eq!(const_str::to_byte_array!(B), [b'h', b'e', b'l', b'l', b'o', b'\0']);
/// ```
///
#[macro_export]
macro_rules! to_byte_array {
    ($s: expr) => {{
        const OUTPUT_LEN: usize = $crate::len!($s);
        $crate::__const::ToByteArray($s).const_eval::<OUTPUT_LEN>()
    }};
}

// -----------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! __map_ascii_case {
    ($s: expr, $case: expr) => {{
        const INPUT: &str = $s;
        const OUTPUT_BYTES: [u8; INPUT.len()] =
            $crate::__const::MapAsciiCase(INPUT, $case).const_eval();
        unsafe { ::core::str::from_utf8_unchecked(&OUTPUT_BYTES) }
    }};
}

/// Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.
///
/// # Examples
/// ```
/// const S: &str = "Hello, World";
/// assert_eq!(const_str::to_ascii_lowercase!(S), "hello, world");
/// ```
///
#[macro_export]
macro_rules! to_ascii_lowercase {
    ($s: expr) => {{
        $crate::__map_ascii_case!($s, $crate::__const::AsciiCase::Lower)
    }};
}

/// Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.
///
/// # Examples
/// ```
/// const S: &str = "Hello, World";
/// assert_eq!(const_str::to_ascii_uppercase!(S), "HELLO, WORLD");
/// ```
///
#[macro_export]
macro_rules! to_ascii_uppercase {
    ($s: expr) => {{
        $crate::__map_ascii_case!($s, $crate::__const::AsciiCase::Upper)
    }};
}

// -----------------------------------------------------------------------------

/// Converts a string literal into an array of its characters.
///
/// # Examples
/// ```
/// let chars: [char; 5] = const_str::to_char_array!("Hello");
/// assert_eq!(chars, ['H', 'e', 'l', 'l', 'o']);
/// ```
///
#[macro_export]
macro_rules! to_char_array {
    ($s: literal) => {
        $crate::__proc::to_char_array!($s)
    };
}

// -----------------------------------------------------------------------------

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
    ($s: literal) => {
        $crate::__proc::from_utf8!($s)
    };
}

// -----------------------------------------------------------------------------

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
    ($s: literal) => {
        $crate::__proc::to_lowercase!($s)
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
    ($s: literal) => {
        $crate::__proc::to_uppercase!($s)
    };
}

// -----------------------------------------------------------------------------

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
    ($s: literal, $from: literal, $to: literal) => {
        $crate::__proc::replace!($s, $from, $to)
    };
}

// -----------------------------------------------------------------------------

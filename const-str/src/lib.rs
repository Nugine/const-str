//! Compile-time string operations
//!
//! MSRV: Rust 1.52.0
//!
//! ## Examples
//!
//! ```rust
//! assert_eq!(const_str::to_lowercase!("HELLO"), "hello");
//!
//! assert_eq!(const_str::to_uppercase!("hello"), "HELLO");
//!
//! assert_eq!(const_str::replace!("this is old", "old", "new"), "this is new");
//!
//! assert_eq!(const_str::to_str!(1_u8 + 1_u8), "2");
//!
//! const PROMPT: &str = "The answer is";
//! const ANSWER: usize = 42;
//! const MESSAGE: &str = const_str::concat!(PROMPT, " ", ANSWER);
//!
//! assert_eq!(MESSAGE, "The answer is 42");
//! ```
//!
//! feature `verify-regex`
//!
//! ```rust
//! use regex::Regex;
//! let re = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
//! assert!(Regex::new(re).is_ok());
//!
//! const_str::regex_assert_match!(r"^\d{4}-\d{2}-\d{2}$", "2014-01-01");
//! ```
//!
//! feature `verify-http`
//!
//! ```rust
//! use http::header::HeaderName;
//! let name = const_str::verified_header_name!("content-md5");
//! assert_eq!(HeaderName::from_static(name).as_str(), "content-md5");
//! ```
//!
#![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]
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

mod utils;

#[doc(hidden)]
pub mod __proc;

#[doc(hidden)]
pub mod __const;

#[cfg(feature = "verify-regex")]
mod verify_regex;

#[cfg(feature = "verify-http")]
mod verify_http;

#[cfg(feature = "case")]
mod case;

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
macro_rules! __transmute_bytes_to_str {
    ($b: expr) => {
        ::core::mem::transmute::<&[u8], &str>($b)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __map_ascii_case {
    ($s: expr, $case: expr) => {{
        const INPUT: &str = $s;
        const OUTPUT_BYTES: [u8; INPUT.len()] =
            $crate::__const::MapAsciiCase(INPUT, $case).const_eval();
        unsafe { $crate::__transmute_bytes_to_str!(&OUTPUT_BYTES) }
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

/// Replaces all matches of a pattern with another string slice.
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
    ($s: expr, $from: expr, $to: expr) => {{
        const OUTPUT_LEN: usize = $crate::__const::Replace($s, $from, $to).output_len();
        const OUTPUT_BYTES: [u8; OUTPUT_LEN] =
            $crate::__const::Replace($s, $from, $to).const_eval();
        unsafe { $crate::__transmute_bytes_to_str!(&OUTPUT_BYTES) }
    }};
}

// -----------------------------------------------------------------------------

/// Checks that two strings are equal.
///
/// # Examples
///
/// ```
/// const A: &str = "hello";
/// const B: &str = "world";
/// const C: &str = "hello";
/// const EQ_AB: bool = const_str::equal!(A, B);
/// const EQ_AC: bool = const_str::equal!(A, C);
/// assert_eq!([EQ_AB, EQ_AC], [false, true]);
///
#[macro_export]
macro_rules! equal {
    ($lhs: expr, $rhs: expr) => {
        $crate::__const::Equal($lhs, $rhs).const_eval()
    };
}

// -----------------------------------------------------------------------------

/// Creates a new string slice by repeating a string slice n times.
///
/// # Examples
///
/// ```
/// const S: &str = "abc";
/// const SSSS: &str = const_str::repeat!(S, 4);
/// assert_eq!(SSSS, "abcabcabcabc");
/// ```
///
#[macro_export]
macro_rules! repeat {
    ($s: expr, $n: expr) => {{
        const INPUT: &str = $s;
        const N: usize = $n;
        const OUTPUT_LEN: usize = INPUT.len() * N;
        const OUTPUT_BYTES: [u8; OUTPUT_LEN] = $crate::__const::Repeat(INPUT, N).const_eval();
        unsafe { $crate::__transmute_bytes_to_str!(&OUTPUT_BYTES) }
    }};
}

// -----------------------------------------------------------------------------

/// Converts a value to a string slice.
///
/// The input type must be one of
///
/// + `&str`
/// + `char`
/// + `bool`
/// + `u8`, `u16`, `u32`, `u128`, `usize`
/// + `i8`, `i16`, `i32`, `i128`, `isize`
///
/// # Examples
///
/// ```
/// const A: &str = const_str::to_str!("A");
/// assert_eq!(A, "A");
///
/// const B: &str = const_str::to_str!('我');
/// assert_eq!(B, "我");
///
/// const C: &str = const_str::to_str!(true);
/// assert_eq!(C, "true");
///
/// const D: &str = const_str::to_str!(1_u8 + 1);
/// assert_eq!(D, "2");
///
/// const E: &str = const_str::to_str!(-21_i32 * 2);
/// assert_eq!(E, "-42")
/// ```
///
#[macro_export]
macro_rules! to_str {
    ($x: expr) => {{
        const OUTPUT_LEN: usize = $crate::__const::ToStr($x).output_len();
        const OUTPUT_BYTES: [u8; OUTPUT_LEN] = $crate::__const::ToStr($x).const_eval();
        unsafe { $crate::__transmute_bytes_to_str!(&OUTPUT_BYTES) }
    }};
}

/// Concatenates values into a string slice.
///
/// The input type must be one of
///
/// + `&str`
/// + `char`
/// + `bool`
/// + `u8`, `u16`, `u32`, `u128`, `usize`
/// + `i8`, `i16`, `i32`, `i128`, `isize`
///
///
/// # Examples
///
/// ```
/// const PROMPT: &str = "The answer is";
/// const ANSWER: usize = 42;
/// const MESSAGE: &str = const_str::concat!(PROMPT, " ", ANSWER);
///
/// assert_eq!(MESSAGE, "The answer is 42");
/// ```
///
#[macro_export]
macro_rules! concat {
    ($($x: expr),+ $(,)?) => {{
        const STRS: &[&str] = &[$( $crate::to_str!($x) ),+];
        const OUTPUT_LEN: usize = $crate::__const::Concat(STRS).output_len();
        const OUTPUT_BYTES: [u8; OUTPUT_LEN] = $crate::__const::Concat(STRS).const_eval();
        unsafe { $crate::__transmute_bytes_to_str!(&OUTPUT_BYTES) }
    }}
}

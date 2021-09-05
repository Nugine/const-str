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

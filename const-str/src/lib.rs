//! compile-time string operations
//!
//! # Examples
//!
//! ```
//! assert_eq!(const_str::to_lowercase!("HELLO"), "hello");
//!
//! assert_eq!(const_str::to_uppercase!("hello"), "HELLO");
//!
//! assert_eq!(const_str::replace!("this is old", "old", "new"), "this is new");
//! ```
//!
//! feature `verify-regex`
//!
//! ```
//! use regex::Regex;
//! let re = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
//! assert!(Regex::new(re).is_ok());
//! ```
//!
//! feature `verify-http`
//!
//! ```
//! use http::header::HeaderName;
//! let name = const_str::verified_header_name!("content-md5");
//! assert_eq!(HeaderName::from_static(name).as_str(), "content-md5");
//! ```
//!

#![deny(
    anonymous_parameters,
    bare_trait_objects,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]
#![no_std]

#[doc(hidden)]
pub mod __imp {
    pub use const_str_proc_macro::{replace, to_lowercase, to_uppercase};

    #[cfg(feature = "verify-regex")]
    pub use const_str_proc_macro::verified_regex;

    #[cfg(feature = "verify-http")]
    pub use const_str_proc_macro::verified_header_name;
}

/// Returns the lowercase equivalent of this string literal, as a new string literal.
///
/// See [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)
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
/// See [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase)
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
/// See [`str::replace`](https://doc.rust-lang.org/std/primitive.str.html#method.replace)
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

/// Returns a compile-time verified regex string literal.
///
/// # Examples
///
/// ```
/// use regex::Regex;
/// let re = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
/// assert!(Regex::new(re).is_ok());
/// ```
///
#[cfg(feature = "verify-regex")]
#[macro_export]
macro_rules! verified_regex {
    ($re:literal) => {
        $crate::__imp::verified_regex!($re)
    };
}

/// Returns a compile-time verified header name string literal.
///
/// # Examples
///
/// ```
/// use http::header::HeaderName;
/// let name = const_str::verified_header_name!("content-md5");
/// assert_eq!(HeaderName::from_static(name).as_str(), "content-md5");
/// ```
///
#[cfg(feature = "verify-http")]
#[macro_export]
macro_rules! verified_header_name {
    ($name:literal) => {
        $crate::__imp::verified_header_name!($name)
    };
}

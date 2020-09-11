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
//!
//! const_str::regex_assert_match!(r"^\d{4}-\d{2}-\d{2}$", "2014-01-01");
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
    pub use const_str_proc_macro::{as_bytes, from_utf8, len, replace, to_lowercase, to_uppercase};

    #[cfg(feature = "verify-regex")]
    pub use const_str_proc_macro::{regex_assert_match, verified_regex};

    #[cfg(feature = "verify-http")]
    pub use const_str_proc_macro::verified_header_name;
}

mod str;

#[cfg(feature = "verify-regex")]
mod verify_regex;

#[cfg(feature = "verify-http")]
mod verify_http;

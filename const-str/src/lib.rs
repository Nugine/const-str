//! Compile-time string operations
//!
//! MSRV: Rust 1.64.0
//!
#![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    feature = "unstable", 
    feature(const_char_from_u32_unchecked), // https://github.com/rust-lang/rust/issues/89259
    feature(const_align_offset),            // https://github.com/rust-lang/rust/issues/90962
)]

#[allow(unused_macros)]
macro_rules! item_group {
    ($($item:item)*) => {
        $($item)*
    }
}

mod ascii;
mod bytes;
mod printable;
mod slice;
mod str;
mod utf16;
mod utf8;

#[doc(hidden)]
#[cfg(feature = "proc")]
pub mod __proc {
    mod case;
    pub use self::case::*;

    mod fmt;
    pub use self::fmt::*;

    #[cfg(feature = "http")]
    item_group! {
        mod http;
        pub use self::http::*;
    }

    #[cfg(feature = "regex")]
    item_group! {
        mod regex;
        pub use self::regex::*;
    }
}

#[doc(hidden)]
pub mod __ctfe {
    mod ascii_case;
    pub use self::ascii_case::*;

    mod compare;
    pub use self::compare::*;

    mod concat;
    pub use self::concat::*;

    mod concat_bytes;
    pub use self::concat_bytes::*;

    mod cstr;
    pub use self::cstr::*;

    mod encode;
    pub use self::encode::*;

    mod equal;
    pub use self::equal::*;

    mod find;
    pub use self::find::*;

    mod fmt;
    pub use self::fmt::*;

    mod hex;
    pub use self::hex::*;

    #[cfg(feature = "std")]
    item_group! {
        mod net;
        pub use self::net::*;
    }

    mod parse;
    pub use self::parse::*;

    mod repeat;
    pub use self::repeat::*;
    mod replace;
    pub use self::replace::*;

    mod str;
    pub use self::str::*;

    mod to_byte_array;
    pub use self::to_byte_array::*;

    mod to_char_array;
    pub use self::to_char_array::*;

    mod to_str;
    pub use self::to_str::*;

    mod sorted;
    pub use self::sorted::*;

    mod split;
    pub use self::split::*;

    mod squish;
    pub use self::squish::*;

    mod is_ascii;
    pub use self::is_ascii::*;

    mod eq_ignore_ascii_case;
    pub use self::eq_ignore_ascii_case::*;

    mod unwrap;
    pub use self::unwrap::*;
}

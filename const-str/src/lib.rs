//! Compile-time string operations
//!
//! MSRV: Rust 1.51.0
//!
#![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

macro_rules! constfn_assert {
    ($e:expr) => {{
        let _: () = [()][(!($e) as usize)];
    }};
}

macro_rules! constfn_panic {
    ($s: literal) => {{
        #[allow(unconditional_panic)]
        let _: &str = [$s][1];
        loop {}
    }};
}

macro_rules! constfn_unreachable {
    () => {
        constfn_panic!("unreachable")
    };
}

#[allow(unused_macros)]
macro_rules! item_group {
    ($($tt:tt)*) => {
        $($tt)*
    }
}

mod ascii;
mod bytes;
mod str;
mod utf16;
mod utf8;

#[doc(hidden)]
pub mod __proc {
    mod case;
    pub use self::case::*;

    mod fmt;
    pub use self::fmt::*;

    mod str;
    pub use self::str::*;

    #[cfg(feature = "verify-http")]
    item_group! {
        mod verify_http;
        pub use self::verify_http::*;
    }

    #[cfg(feature = "verify-regex")]
    item_group! {
        mod verify_regex;
        pub use self::verify_regex::*;
    }
}

#[doc(hidden)]
pub mod __ctfe {
    mod ascii_case;
    pub use self::ascii_case::*;

    mod concat;
    pub use self::concat::*;

    #[cfg(feature = "std")]
    item_group! {
        mod cstr;
        pub use self::cstr::*;
    }

    mod encode;
    pub use self::encode::*;

    mod equal;
    pub use self::equal::*;

    mod find;
    pub use self::find::*;

    mod fmt;
    pub use self::fmt::*;

    mod hex_bytes;
    pub use self::hex_bytes::*;

    mod len;
    pub use self::len::*;

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

    mod str_buf;
    pub use self::str_buf::*;

    mod to_byte_array;
    pub use self::to_byte_array::*;

    mod to_char_array;
    pub use self::to_char_array::*;

    mod to_str;
    pub use self::to_str::*;
}

//! Compile-time string operations.
//! See the [macro list](#macros) for what you need.
//!
//! MSRV: Rust 1.77.0
//!
//! ## Troubleshoot
//!
//! You don't have to care about this section
//! unless you come across some compile errors about const evaluation.
//!
//! ```txt
//! error[E0435]: attempt to use a non-constant value in a constant
//! ```
//!
//! There are mainly two kinds of macros in this crate,
//! which have different requirements for the arguments.
//! - [const-context only](#const-context-only)
//! - [const-fn compatible](#const-fn-compatible)
//!
//! ### const-context only
//!
//! These macros can only be used in [const contexts][const-context].
//! The expanded code is equivalent to compute new [constant items][const-item].
//! It implies that the *arguments* of these macros must be constant values,
//! similar to [`consteval`][consteval] in C++ world.
//!
//! The following examples will not work:
//! ```compile_fail
//! const fn foo(a: &str, b: &str) -> &str {
//!    const_str::concat!(a, b)
//! }
//! ```
//! ```compile_fail
//! const C: &str = {
//!     let a = "Hello";
//!     let b = "World";
//!     const_str::concat!(a, b);
//! };
//! ```
//!
//! Instead, this way will work:
//! ```
//! const A: &str = "Hello";
//! const B: &str = "World";
//! const C: &str = const_str::concat!(A, " ", B);
//! assert_eq!(C, "Hello World");
//! ```
//!
//! ### const-fn compatible
//!
//! These macros can be used in [const contexts][const-context] and [const functions][const-fn].
//! The expanded code is equivalent to calling const functions.
//! It implies that the *arguments* of these macros can be any expressions,
//! similar to [`constexpr`][constexpr] in C++ world.
//!
//! ```
//! const fn calc(y: &str, m: &str, d: &str) -> u64 {
//!     let y = const_str::parse!(y, u64);
//!     let m = const_str::parse!(m, u64);
//!     let d = const_str::parse!(d, u64);
//!     (y * 10000 + m * 100 + d)
//! }
//! const TIME: u64 = calc("2025", "01", "26");
//! assert_eq!(TIME, 20250126);
//! ```
//!
//! You can also use these macros in normal functions,
//! but they may be much slower than the runtime equivalents.
//! It's recommended to use them only if you need compile-time evaluation.
//!
//! [const-context]: https://doc.rust-lang.org/reference/const_eval.html#const-context
//! [const-fn]: https://doc.rust-lang.org/reference/const_eval.html#const-functions
//! [const-item]: https://doc.rust-lang.org/reference/items/constant-items.html
//! [consteval]: https://en.cppreference.com/w/cpp/language/consteval
//! [constexpr]: https://en.cppreference.com/w/cpp/language/constexpr
//!
#![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_macros)]
macro_rules! cfg_group {
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
    cfg_group! {
        mod http;
        pub use self::http::*;
    }

    #[cfg(feature = "regex")]
    cfg_group! {
        mod regex;
        pub use self::regex::*;
    }
}

#[doc(hidden)]
pub mod __ctfe {
    mod ascii_case;
    pub use self::ascii_case::*;

    mod chain;
    // pub use self::chain::*;

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

    mod net;
    pub use self::net::*;

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

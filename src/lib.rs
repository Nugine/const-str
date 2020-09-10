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
//! feature `regex`
//!
//! ```
//! use regex::Regex;
//! let re = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
//! assert!(Regex::new(re).is_ok());
//! ```
//!
//! feature `http`
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

use proc_macro::TokenStream;

use quote::ToTokens;

use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, LitStr, Token,
};

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
#[proc_macro]
pub fn to_lowercase(input: TokenStream) -> TokenStream {
    let src_token: LitStr = parse_macro_input!(input as LitStr);
    let dst = src_token.value().to_lowercase();
    LitStr::new(&dst, src_token.span())
        .into_token_stream()
        .into()
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
#[proc_macro]
pub fn to_uppercase(input: TokenStream) -> TokenStream {
    let src_token: LitStr = parse_macro_input!(input as LitStr);
    let dst = src_token.value().to_uppercase();
    LitStr::new(&dst, src_token.span())
        .into_token_stream()
        .into()
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
#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    struct Replace {
        src: LitStr,
        from: LitStr,
        to: LitStr,
    }

    impl Parse for Replace {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let src = input.parse::<LitStr>()?;
            let _ = input.parse::<Token![,]>()?;
            let from = input.parse::<LitStr>()?;
            let _ = input.parse::<Token![,]>()?;
            let to = input.parse::<LitStr>()?;
            Ok(Self { src, from, to })
        }
    }

    impl Replace {
        fn exec(&self) -> LitStr {
            let src = self.src.value();
            let from = self.from.value();
            let to = self.to.value();
            let dst = src.replace(&from, &to);
            LitStr::new(&dst, self.src.span())
        }
    }

    let f: Replace = parse_macro_input!(input as Replace);
    f.exec().into_token_stream().into()
}

#[allow(unused_macros)]
macro_rules! emit_error {
    ($token:expr, $msg: expr) => {
        return TokenStream::from(syn::Error::new($token.span(), $msg).to_compile_error());
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
#[cfg(feature = "regex")]
#[proc_macro]
pub fn verified_regex(input: TokenStream) -> TokenStream {
    let src_token: LitStr = parse_macro_input!(input as LitStr);

    if let Err(e) = regex::Regex::new(&src_token.value()) {
        emit_error!(src_token, format!("{}", e));
    }

    src_token.into_token_stream().into()
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
#[cfg(feature = "http")]
#[proc_macro]
pub fn verified_header_name(input: TokenStream) -> TokenStream {
    use http::header::HeaderName;

    let src_token: LitStr = parse_macro_input!(input as LitStr);

    if let Err(e) = HeaderName::from_lowercase(src_token.value().as_bytes()) {
        emit_error!(src_token, format!("{}", e));
    }

    src_token.into_token_stream().into()
}

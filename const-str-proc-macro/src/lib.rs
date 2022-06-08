//! const-str proc macros

#![forbid(unsafe_code)]
#![deny(missing_docs, clippy::all, clippy::cargo)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]

#[allow(unused_macros)]
macro_rules! emit_error {
    ($token:expr, $msg: expr) => {
        return TokenStream::from(syn::Error::new($token.span(), $msg).to_compile_error())
    };
}

mod case;
mod fmt;

#[cfg(feature = "regex")]
mod regex;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{parse_macro_input, LitStr};

#[allow(dead_code)]
fn direct_convert<T, E, F>(input: TokenStream, f: F) -> TokenStream
where
    T: Parse + Spanned,
    E: ToString,
    F: FnOnce(&T) -> Result<String, E>,
{
    let src_token: T = parse_macro_input!(input as T);
    let s = match f(&src_token) {
        Ok(s) => s,
        Err(e) => emit_error!(src_token, e.to_string()),
    };
    let dst_token = LitStr::new(&s, src_token.span());
    dst_token.into_token_stream().into()
}

#[doc(hidden)]
#[proc_macro]
pub fn format_parts(input: TokenStream) -> TokenStream {
    use crate::fmt::ConstFormat;
    let m = parse_macro_input!(input as ConstFormat);
    m.eval()
}

/// Converts a string literal to a specified case.
#[proc_macro]
pub fn convert_case(input: TokenStream) -> TokenStream {
    use crate::case::ConvertCase;
    let m = parse_macro_input!(input as ConvertCase);
    m.eval()
}

// -----------------------------------------------------------------------------

/// Returns a compile-time verified header name string literal.
#[cfg(feature = "http")]
#[proc_macro]
pub fn verified_header_name(input: TokenStream) -> TokenStream {
    use http::header::HeaderName;

    direct_convert(input, |s: &LitStr| {
        let s = s.value();
        HeaderName::from_lowercase(s.as_bytes()).map(|_| s)
    })
}

// -----------------------------------------------------------------------------

/// Returns a compile-time verified regex string literal.
#[cfg(feature = "regex")]
#[proc_macro]
pub fn verified_regex(input: TokenStream) -> TokenStream {
    direct_convert(input, |s: &LitStr| {
        let s = s.value();
        ::regex::Regex::new(&s).map(|_| s)
    })
}

/// Asserts that the string literal matches the pattern.
#[cfg(feature = "regex")]
#[proc_macro]
pub fn regex_assert_match(input: TokenStream) -> TokenStream {
    use crate::regex::RegexAssertMatch;
    let m = parse_macro_input!(input as RegexAssertMatch);
    m.eval()
}

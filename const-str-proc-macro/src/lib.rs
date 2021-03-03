//! const-str proc macros

#![forbid(unsafe_code)]
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
#![no_std]

#[allow(unused_extern_crates)]
extern crate alloc;

#[allow(unused_imports)]
use alloc::string::{String, ToString};
use core::convert::Infallible;

use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::ToTokens;

use syn::{
    parse::Parser,
    parse::{Parse, ParseStream},
    parse_macro_input,
    LitByteStr,
    LitInt,
    LitStr,
    Token,
};

#[allow(unused_macros)]
macro_rules! emit_error {
    ($token:expr, $msg: expr) => {
        return TokenStream::from(syn::Error::new($token.span(), $msg).to_compile_error());
    };
}

fn direct_convert<E: ToString, F>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(String) -> Result<String, E>,
{
    let src_token: LitStr = parse_macro_input!(input as LitStr);
    let s = match f(src_token.value()) {
        Ok(s) => s,
        Err(e) => emit_error!(src_token, e.to_string()),
    };
    let dst_token = LitStr::new(&s, src_token.span());
    dst_token.into_token_stream().into()
}

/// Returns the lowercase equivalent of this string literal, as a new string literal.
#[proc_macro]
pub fn to_lowercase(input: TokenStream) -> TokenStream {
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_lowercase()))
}

/// Returns the uppercase equivalent of this string literal, as a new string literal.
#[proc_macro]
pub fn to_uppercase(input: TokenStream) -> TokenStream {
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_uppercase()))
}

/// Replaces all matches of a pattern with another string literal.
#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    struct Replace {
        src: LitStr,
        from: LitStr,
        to: LitStr,
    }

    impl Parse for Replace {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
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

/// Converts a string literal to a byte string literal
#[proc_macro]
pub fn as_bytes(input: TokenStream) -> TokenStream {
    let src_token: LitStr = parse_macro_input!(input as LitStr);
    let dst_token = LitByteStr::new(src_token.value().as_bytes(), src_token.span());
    dst_token.into_token_stream().into()
}

/// Converts a byte string literal to a string literal
#[proc_macro]
pub fn from_utf8(input: TokenStream) -> TokenStream {
    let src_token: LitByteStr = parse_macro_input!(input as LitByteStr);
    let dst = match String::from_utf8(src_token.value()) {
        Err(_) => emit_error!(
            src_token,
            "the byte string literal is not a valid UTF-8 string"
        ),
        Ok(s) => s,
    };
    let dst_token = LitStr::new(&dst, src_token.span());
    dst_token.into_token_stream().into()
}

/// Returns the length of the string literal
#[proc_macro]
pub fn len(input: TokenStream) -> TokenStream {
    fn transform(input: ParseStream<'_>) -> syn::Result<LitInt> {
        let (len, span) = if input.peek(LitStr) {
            let token = input.parse::<LitStr>()?;
            (token.value().len(), token.span())
        } else if input.peek(LitByteStr) {
            let token = input.parse::<LitByteStr>()?;
            (token.value().len(), token.span())
        } else {
            return Err(syn::Error::new(
                Span::call_site(),
                "expected string literal or byte string literal",
            ));
        };
        let len_repr = alloc::format!("{}_usize", len);
        let dst_token = LitInt::new(&len_repr, span);
        Ok(dst_token)
    }
    match transform.parse(input) {
        Ok(token) => token.into_token_stream().into(),
        Err(e) => TokenStream::from(e.to_compile_error()),
    }
}

/// Converts a string literal into an array of its characters.
#[allow(clippy::integer_arithmetic)]
#[proc_macro]
pub fn to_char_array(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr).value();
    let iter = input_str.chars();
    (quote::quote! { [#(#iter),*] }).into()
}

/// Returns a compile-time verified regex string literal.
#[cfg(feature = "regex")]
#[proc_macro]
pub fn verified_regex(input: TokenStream) -> TokenStream {
    use regex::Regex;
    direct_convert(input, |s| Regex::new(&s).map(|_| s))
}

/// Asserts that the string literal matches the pattern.
#[cfg(feature = "regex")]
#[proc_macro]
pub fn regex_assert_match(input: TokenStream) -> TokenStream {
    struct RegexAssertMatch {
        re: LitStr,
        text: LitStr,
    }

    impl Parse for RegexAssertMatch {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let re = input.parse::<LitStr>()?;
            let _ = input.parse::<Token![,]>()?;
            let text = input.parse::<LitStr>()?;
            Ok(Self { re, text })
        }
    }

    use regex::Regex;

    let f: RegexAssertMatch = parse_macro_input!(input as RegexAssertMatch);

    let re: Regex = match Regex::new(&f.re.value()) {
        Ok(re) => re,
        Err(e) => emit_error!(f.re, e.to_string()),
    };

    let text = f.text.value();

    if !re.is_match(&text) {
        emit_error!(f.text, "the string literal does not match the pattern")
    }

    TokenStream::new()
}

/// Returns a compile-time verified header name string literal.
#[cfg(feature = "http")]
#[proc_macro]
pub fn verified_header_name(input: TokenStream) -> TokenStream {
    use http::header::HeaderName;

    direct_convert(input, |s| {
        HeaderName::from_lowercase(s.as_bytes()).map(|_| s)
    })
}

/// Converts a string literal to camel case.
#[cfg(feature = "heck")]
#[proc_macro]
pub fn to_camel_case(input: TokenStream) -> TokenStream {
    use heck::CamelCase;
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_camel_case()))
}

/// Converts a string literal to kebab case.
#[cfg(feature = "heck")]
#[proc_macro]
pub fn to_kebab_case(input: TokenStream) -> TokenStream {
    use heck::KebabCase;
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_kebab_case()))
}

/// Converts a string literal to snake case.
#[cfg(feature = "heck")]
#[proc_macro]
pub fn to_snake_case(input: TokenStream) -> TokenStream {
    use heck::SnakeCase;
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_snake_case()))
}

/// Converts a string literal to shouty snake case.
#[cfg(feature = "heck")]
#[proc_macro]
pub fn to_shouty_snake_case(input: TokenStream) -> TokenStream {
    use heck::ShoutySnakeCase;
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_shouty_snake_case()))
}

/// Converts a string literal to shouty kebab case.
#[cfg(feature = "heck")]
#[proc_macro]
pub fn to_shouty_kebab_case(input: TokenStream) -> TokenStream {
    use heck::ShoutySnakeCase;
    direct_convert::<Infallible, _>(input, |s| Ok(s.to_shouty_snake_case().replace("_", "-")))
}

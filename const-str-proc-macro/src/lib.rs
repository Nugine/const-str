//! const-str proc macros

#![forbid(unsafe_code)]
#![deny(missing_docs, clippy::all, clippy::cargo)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return
)]

mod fmt;

#[allow(unused_imports)]
use std::string::ToString;
use std::{collections::HashMap, convert::Infallible};

use fmt::FmtMethod;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, LitByteStr, LitStr, Token};

use self::fmt::FmtPart;

#[allow(unused_macros)]
macro_rules! emit_error {
    ($token:expr, $msg: expr) => {
        return TokenStream::from(syn::Error::new($token.span(), $msg).to_compile_error())
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

fn fmt_method(method: &FmtMethod) -> proc_macro2::TokenStream {
    match method {
        FmtMethod::Debug => quote! { __fmt_debug },
        FmtMethod::Display => quote! { __fmt_display },
        FmtMethod::LowerHex => quote! { __fmt_lowerhex },
        FmtMethod::UpperHex => quote! { __fmt_upperhex },
        FmtMethod::Binary => quote! { __fmt_binary },
    }
}

fn fmt_spec(part: &FmtPart) -> proc_macro2::TokenStream {
    let alternate = part.spec.alternate;
    quote! {{
        const_str::__ctfe::FmtSpec {
            alternate: #alternate
        }
    }}
}

/// Creates a string slice using interpolation of const expressions
#[proc_macro]
pub fn format(input: TokenStream) -> TokenStream {
    struct ConstFormat {
        fmt_string: LitStr,
        positional_args: Vec<syn::Expr>,
        named_args: HashMap<syn::Ident, syn::Expr>,
    }

    impl Parse for ConstFormat {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let fmt_string = input.parse::<LitStr>()?;
            let mut comma = input.parse::<Option<Token![,]>>()?;

            let mut positional_args = Vec::new();
            let mut named_args = HashMap::new();

            if input.is_empty() {
                return Ok(ConstFormat {
                    fmt_string,
                    positional_args,
                    named_args,
                });
            }

            while !input.is_empty() && !input.peek2(Token![=]) {
                if comma.is_none() {
                    return Err(input.error("expected comma"));
                }

                let arg = input.parse::<Expr>()?;
                comma = input.parse::<Option<Token![,]>>()?;
                positional_args.push(arg);
            }

            if input.is_empty() {
                return Ok(ConstFormat {
                    fmt_string,
                    positional_args,
                    named_args,
                });
            }

            while input.peek2(Token![=]) {
                if comma.is_none() {
                    return Err(input.error("expected comma"));
                }
                let name = input.parse::<Ident>()?;
                let _ = input.parse::<Token![=]>()?;
                let kwarg = input.parse::<Expr>()?;
                comma = input.parse::<Option<Token![,]>>()?;
                let prev = named_args.insert(name, kwarg);
                if prev.is_some() {
                    return Err(input.error("duplicate argument"));
                }
            }

            if input.is_empty() {
                Ok(ConstFormat {
                    fmt_string,
                    positional_args,
                    named_args,
                })
            } else {
                Err(input.error("unexpected tokens"))
            }
        }
    }

    let f = parse_macro_input!(input as ConstFormat);

    let parts = match self::fmt::parse_fmt_string(&f.fmt_string.value()) {
        Ok(p) => p,
        Err(err) => emit_error!(f.fmt_string, err.to_string()),
    };

    let mut eval_parts = Vec::new();

    for p in parts {
        eval_parts.push(loop {
            if let Some(ref s) = p.literal {
                break quote! {
                    {
                        const __FMT_PART: &str = #s;
                        __FMT_PART
                    },
                };
            }
            if let Some(pos) = p.pos {
                let method = p.method.as_ref().unwrap();
                match f.positional_args.get(pos) {
                    None => emit_error!(
                        f.fmt_string,
                        std::format!(
                            "invalid reference to positional argument {} (no arguments were given)",
                            pos
                        )
                    ),
                    Some(arg) => {
                        let method_ident = fmt_method(method);
                        let spec = fmt_spec(&p);
                        break quote! {
                            {
                                const __FMT_PART: &str = ::const_str::#method_ident!(#arg, #spec);
                                __FMT_PART
                            },
                        };
                    }
                }
            }
            if let Some(ref name) = p.name {
                let method_ident = fmt_method(p.method.as_ref().unwrap());
                let spec = fmt_spec(&p);

                break match f.named_args.get(name) {
                    None => quote! {
                        {
                            const __FMT_PART: &str = ::const_str::#method_ident!(#name, #spec);
                            __FMT_PART
                        },
                    },
                    Some(kwarg) => quote! {
                        {
                            const __FMT_PART: &str = ::const_str::#method_ident!(#kwarg, #spec);
                            __FMT_PART
                        },
                    },
                };
            }
            unreachable!()
        })
    }

    let tt = quote! {
        {
            use core::primitive::{str, usize};
            const STRS: &[&str] = &[
                #(#eval_parts)*
            ];
            const OUTPUT_LEN: usize = const_str::__ctfe::Concat(STRS).output_len();
            const OUTPUT_BUF: const_str::__ctfe::StrBuf<OUTPUT_LEN> = const_str::__ctfe::Concat(STRS).const_eval();
            const_str::__strbuf_as_str!(&OUTPUT_BUF)
        }
    };

    tt.into()
}

use std::collections::HashMap;
use std::{fmt, mem};

use std::string::String;
use std::vec::Vec;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitStr, Token};

#[derive(Debug)]
struct FmtPart {
    pub literal: Option<String>,
    pub pos: Option<usize>,
    pub name: Option<Ident>,
    pub method: Option<FmtMethod>,
    pub spec: FmtSpec,
}

#[derive(Debug)]
struct FmtSpec {
    pub alternate: bool,
}

#[derive(Debug)]
enum FmtMethod {
    Debug,
    Display,
    LowerHex,
    UpperHex,
    Binary,
}

impl FmtSpec {
    fn empty() -> Self {
        Self { alternate: false }
    }

    fn alternate() -> Self {
        Self { alternate: true }
    }
}

impl FmtPart {
    #[cfg(test)]
    fn literal_str(&self) -> Option<&str> {
        self.literal.as_deref()
    }

    #[cfg(test)]
    fn named_ident(&self) -> Option<&Ident> {
        self.name.as_ref()
    }

    fn from_literal(lit: String) -> Self {
        Self {
            literal: Some(lit),
            pos: None,
            name: None,
            method: None,
            spec: FmtSpec::empty(),
        }
    }

    fn from_positional(pos: usize, method: FmtMethod, spec: FmtSpec) -> Self {
        Self {
            literal: None,
            pos: Some(pos),
            name: None,
            method: Some(method),
            spec,
        }
    }

    fn from_named(name: Ident, method: FmtMethod, spec: FmtSpec) -> Self {
        Self {
            literal: None,
            pos: None,
            name: Some(name),
            method: Some(method),
            spec,
        }
    }
}

#[derive(Debug)]
struct ParseError {
    _priv: (),
}

impl ParseError {
    fn new() -> Self {
        Self { _priv: () }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unsupported format string")
    }
}

fn parse_fmt_string(s: &str) -> Result<Vec<FmtPart>, ParseError> {
    let mut ans = Vec::new();
    let mut iter = s.chars().peekable();

    let mut literal: String = String::new();
    let mut fmt_spec: String = String::new();
    let mut pos_iter = 0;

    loop {
        match iter.next() {
            None => {
                if !literal.is_empty() {
                    ans.push(FmtPart::from_literal(mem::take(&mut literal)));
                }
                break;
            }
            Some('{') => match iter.next() {
                None => {
                    return Err(ParseError::new());
                }
                Some('{') => {
                    literal.push('{');
                    continue;
                }
                Some(mut ch) => {
                    if !literal.is_empty() {
                        ans.push(FmtPart::from_literal(mem::take(&mut literal)));
                    }
                    while ch != '}' {
                        fmt_spec.push(ch);
                        match iter.next() {
                            Some(c) => ch = c,
                            None => return Err(ParseError::new()),
                        }
                    }
                    ans.push(parse_fmt_spec(&fmt_spec, &mut pos_iter)?);
                    fmt_spec.clear();
                }
            },
            Some('}') => match iter.next() {
                Some('}') => {
                    literal.push('}');
                    continue;
                }
                _ => return Err(ParseError::new()),
            },
            Some(ch) => literal.push(ch),
        }
    }

    Ok(ans)
}

fn parse_fmt_spec(s: &str, pos_iter: &mut usize) -> Result<FmtPart, ParseError> {
    let pieces = s.split(':').collect::<Vec<_>>();
    if pieces.len() > 2 {
        return Err(ParseError::new());
    }

    let (method, spec) = match pieces.get(1).copied() {
        Some("?") => (FmtMethod::Debug, FmtSpec::empty()),
        Some("#?") => (FmtMethod::Debug, FmtSpec::alternate()),
        Some("") | None => (FmtMethod::Display, FmtSpec::empty()),
        Some("x") => (FmtMethod::LowerHex, FmtSpec::empty()),
        Some("#x") => (FmtMethod::LowerHex, FmtSpec::alternate()),
        Some("X") => (FmtMethod::UpperHex, FmtSpec::empty()),
        Some("#X") => (FmtMethod::UpperHex, FmtSpec::alternate()),
        Some("b") => (FmtMethod::Binary, FmtSpec::empty()),
        Some("#b") => (FmtMethod::Binary, FmtSpec::alternate()),
        _ => return Err(ParseError::new()),
    };

    let argument = pieces[0];
    if argument.is_empty() {
        let pos = *pos_iter;
        *pos_iter += 1;
        return Ok(FmtPart::from_positional(pos, method, spec));
    }

    if let Ok(pos) = argument.parse::<usize>() {
        return Ok(FmtPart::from_positional(pos, method, spec));
    }

    if let Ok(name) = syn::parse_str::<Ident>(argument) {
        return Ok(FmtPart::from_named(name, method, spec));
    }

    Err(ParseError::new())
}

#[test]
fn test_parse_fmt() {
    {
        let s = "";
        assert!(parse_fmt_string(s).unwrap().is_empty());
    }

    {
        let s = "{}";
        let parts = parse_fmt_string(s).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(parts[0].pos, Some(0)))
    }

    {
        let s = "{1} {} {0} {}";
        let parts = parse_fmt_string(s).unwrap();
        assert_eq!(parts.len(), 7);
        assert!(matches!(parts[0].pos, Some(1)));
        assert!(matches!(parts[1].literal_str().unwrap(), " "));
        assert!(matches!(parts[2].pos, Some(0)));
        assert!(matches!(parts[3].literal_str().unwrap(), " "));
        assert!(matches!(parts[4].pos, Some(0)));
        assert!(matches!(parts[5].literal_str().unwrap(), " "));
        assert!(matches!(parts[6].pos, Some(1)));
    }

    {
        let s = "{argument}";
        let parts = parse_fmt_string(s).unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].named_ident().unwrap(), "argument");
    }

    {
        let s = "{name} {}";
        let parts = parse_fmt_string(s).unwrap();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].named_ident().unwrap(), "name");
        assert!(matches!(parts[1].literal_str().unwrap(), " "));
        assert!(matches!(parts[2].pos, Some(0)));
    }

    {
        let s = "{a} {c} {b}";
        let parts = parse_fmt_string(s).unwrap();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].named_ident().unwrap(), "a");
        assert!(matches!(parts[1].literal_str().unwrap(), " "));
        assert_eq!(parts[2].named_ident().unwrap(), "c");
        assert!(matches!(parts[3].literal_str().unwrap(), " "));
        assert_eq!(parts[4].named_ident().unwrap(), "b");
    }

    {
        let s = "{{}}";
        let parts = parse_fmt_string(s).unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].literal_str().unwrap(), "{}");
    }
}

pub struct ConstFormat {
    fmt_string: LitStr,
    positional_args: Vec<Expr>,
    named_args: HashMap<Ident, Expr>,
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

impl ConstFormat {
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

    pub fn eval(&self) -> TokenStream {
        let parts = match parse_fmt_string(&self.fmt_string.value()) {
            Ok(p) => p,
            Err(err) => emit_error!(self.fmt_string, err.to_string()),
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
                match self.positional_args.get(pos) {
                    None => emit_error!(
                        self.fmt_string,
                        std::format!(
                            "invalid reference to positional argument {} (no arguments were given)",
                            pos
                        )
                    ),
                    Some(arg) => {
                        let method_ident = Self::fmt_method(method);
                        let spec = Self::fmt_spec(&p);
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
                let method_ident = Self::fmt_method(p.method.as_ref().unwrap());
                let spec = Self::fmt_spec(&p);

                break match self.named_args.get(name) {
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
}

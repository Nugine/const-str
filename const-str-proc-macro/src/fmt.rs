use std::{fmt, mem};

use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
pub struct FmtPart {
    pub literal: Option<String>,
    pub pos: Option<usize>,
    pub name: Option<syn::Ident>,
    pub method: Option<FmtMethod>,
    pub spec: FmtSpec,
}

#[derive(Debug)]
pub struct FmtSpec {
    pub alternate: bool,
}

#[derive(Debug)]
pub enum FmtMethod {
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
    fn named_ident(&self) -> Option<&syn::Ident> {
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

    fn from_named(name: syn::Ident, method: FmtMethod, spec: FmtSpec) -> Self {
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
pub struct ParseError {
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

pub fn parse_fmt_string(s: &str) -> Result<Vec<FmtPart>, ParseError> {
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

    if let Ok(name) = syn::parse_str::<syn::Ident>(argument) {
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

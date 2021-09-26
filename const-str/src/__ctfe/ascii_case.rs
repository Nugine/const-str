#![allow(unsafe_code)]

use core::ops::Range;

use crate::__ctfe::StrBuf;

#[derive(Clone, Copy)]
#[repr(u8)]
enum TokenKind {
    NonAscii = 1,
    Lower = 2,
    Upper = 3,
    Digit = 4,
    Dot = 5,
    Other = 6,
}

impl TokenKind {
    const fn new(b: u8) -> Self {
        if !b.is_ascii() {
            return TokenKind::NonAscii;
        }
        if b.is_ascii_lowercase() {
            return TokenKind::Lower;
        }
        if b.is_ascii_uppercase() {
            return TokenKind::Upper;
        }
        if b.is_ascii_digit() {
            return TokenKind::Digit;
        }
        if b == b'.' {
            return TokenKind::Dot;
        }
        TokenKind::Other
    }

    const fn is_boundary_word(s: &[u8]) -> bool {
        let mut i = 0;
        while i < s.len() {
            let kind = Self::new(s[i]);
            match kind {
                TokenKind::Other | TokenKind::Dot => {}
                _ => return false,
            }
            i += 1;
        }
        true
    }
}

#[derive(Debug)]
struct Boundaries<const N: usize> {
    buf: [usize; N],
    len: usize,
}

impl<const N: usize> Boundaries<N> {
    const fn new(src: &str) -> Self {
        let s = src.as_bytes();
        constfn_assert!(s.len() + 1 == N);

        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x: expr) => {{
                buf[pos] = $x;
                pos += 1;
            }};
        }

        let mut k2: Option<TokenKind> = None;
        let mut k1: Option<TokenKind> = None;

        let mut i = 0;
        while i < s.len() {
            let b = s[i];
            let k0 = TokenKind::new(b);

            use TokenKind::*;

            match (k1, k0) {
                (None, _) => push!(i),
                (Some(k1), k0) => {
                    if k1 as u8 != k0 as u8 {
                        match (k1, k0) {
                            (Upper, Lower) => push!(i - 1),
                            (NonAscii, Digit) => push!(i),
                            (Lower, Digit) | (Upper, Digit) => {} // or-pattens stable since 1.53
                            (Digit, Lower) | (Digit, Upper) | (Digit, NonAscii) => {}
                            (_, Dot) => {}
                            (Dot, _) => match (k2, k0) {
                                (None, _) => push!(i),
                                (Some(k2), k0) => match (k2, k0) {
                                    (Digit, Digit) => {}
                                    _ => {
                                        push!(i - 1);
                                        push!(i);
                                    }
                                },
                            },
                            _ => push!(i),
                        }
                    }
                }
            }

            k2 = k1;
            k1 = Some(k0);
            i += 1;
        }
        push!(i);

        Self { buf, len: pos }
    }

    const fn words_count(&self) -> usize {
        self.len - 1
    }

    const fn word_range(&self, idx: usize) -> Range<usize> {
        self.buf[idx]..self.buf[idx + 1]
    }
}

pub enum AsciiCase {
    Lower,
    Upper,
    Camel,
    Snake,
    Kebab,
    ShoutySnake,
    ShoutyKebab,
}

impl AsciiCase {
    const fn get_seperator(&self) -> Option<u8> {
        match self {
            Self::Snake | Self::ShoutySnake => Some(b'_'),
            Self::Kebab | Self::ShoutyKebab => Some(b'-'),
            _ => None,
        }
    }
}

pub struct ConvAsciiCase<T>(pub T, pub AsciiCase);

impl ConvAsciiCase<&str> {
    pub const fn output_len<const M: usize>(&self) -> usize {
        constfn_assert!(self.0.len() + 1 == M);

        use AsciiCase::*;
        match self.1 {
            Lower | Upper => self.0.len(),
            Camel | Snake | Kebab | ShoutySnake | ShoutyKebab => {
                let mut ans = 0;

                let has_sep = self.1.get_seperator().is_some();

                let boundaries = Boundaries::<M>::new(self.0);
                let words_count = boundaries.words_count();

                let mut i = 0;
                let mut is_starting_boundary: bool = true;

                while i < words_count {
                    let rng = boundaries.word_range(i);
                    let word = crate::bytes::subslice(self.0.as_bytes(), rng);

                    if !TokenKind::is_boundary_word(word) {
                        if has_sep && !is_starting_boundary {
                            ans += 1;
                        }
                        ans += word.len();
                        is_starting_boundary = false;
                    }

                    i += 1;
                }
                ans
            }
        }
    }

    pub const fn const_eval<const M: usize, const N: usize>(&self) -> StrBuf<N> {
        constfn_assert!(self.0.len() + 1 == M);

        let mut buf = [0; N];
        let mut pos = 0;
        let s = self.0.as_bytes();

        macro_rules! push {
            ($x: expr) => {{
                buf[pos] = $x;
                pos += 1;
            }};
        }

        use AsciiCase::*;
        match self.1 {
            Lower => {
                while pos < s.len() {
                    let b = crate::ascii::to_lowercase(s[pos]);
                    push!(b);
                }
            }
            Upper => {
                while pos < s.len() {
                    let b = crate::ascii::to_uppercase(s[pos]);
                    push!(b);
                }
            }
            Camel | Snake | Kebab | ShoutySnake | ShoutyKebab => {
                let sep = self.1.get_seperator();

                let boundaries = Boundaries::<M>::new(self.0);
                let words_count = boundaries.words_count();

                let mut i = 0;
                let mut is_starting_boundary = true;

                while i < words_count {
                    let rng = boundaries.word_range(i);
                    let word = crate::bytes::subslice(self.0.as_bytes(), rng);

                    if !TokenKind::is_boundary_word(word) {
                        if let (Some(sep), false) = (sep, is_starting_boundary) {
                            push!(sep)
                        }
                        let mut j = 0;
                        while j < word.len() {
                            let b = match self.1 {
                                Snake | Kebab => crate::ascii::to_lowercase(word[j]),
                                ShoutySnake | ShoutyKebab => crate::ascii::to_uppercase(word[j]),
                                Camel if j == 0 => crate::ascii::to_uppercase(word[j]),
                                Camel if j > 0 => crate::ascii::to_lowercase(word[j]),
                                _ => constfn_unreachable!(),
                            };
                            push!(b);
                            j += 1;
                        }
                        is_starting_boundary = false;
                    }

                    i += 1;
                }
            }
        }

        constfn_assert!(pos == N);

        unsafe { StrBuf::new_unchecked(buf) }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __conv_ascii_case {
    ($s: expr, $case: expr) => {{
        const INPUT: &str = $s;
        const M: usize = INPUT.len() + 1;
        const N: usize = $crate::__ctfe::ConvAsciiCase(INPUT, $case).output_len::<M>();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<N> =
            $crate::__ctfe::ConvAsciiCase(INPUT, $case).const_eval::<M, N>();
        $crate::__strbuf_as_str!(&OUTPUT_BUF)
    }};
}

/// Converts a string slice to a specified case. Non-ascii characters are not affected.
///
/// # Examples
///
/// ```
/// use const_str::convert_ascii_case;
///
/// const S1: &str = convert_ascii_case!(lower, "Lower Case");
/// const S2: &str = convert_ascii_case!(upper, "Upper Case");
/// const S3: &str = convert_ascii_case!(camel, "camel case");
/// const S4: &str = convert_ascii_case!(snake, "snake case");
/// const S5: &str = convert_ascii_case!(kebab, "kebab case");
/// const S6: &str = convert_ascii_case!(shouty_snake, "shouty snake case");
/// const S7: &str = convert_ascii_case!(shouty_kebab, "shouty kebab case");
///
/// assert_eq!(S1, "lower case");
/// assert_eq!(S2, "UPPER CASE");
/// assert_eq!(S3, "CamelCase");
/// assert_eq!(S4, "snake_case");
/// assert_eq!(S5, "kebab-case");
/// assert_eq!(S6, "SHOUTY_SNAKE_CASE");
/// assert_eq!(S7, "SHOUTY-KEBAB-CASE");
/// ```
#[macro_export]
macro_rules! convert_ascii_case {
    (lower, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Lower)
    };
    (upper, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Upper)
    };
    (camel, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Camel)
    };
    (snake, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Snake)
    };
    (kebab, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Kebab)
    };
    (shouty_snake, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::ShoutySnake)
    };
    (shouty_kebab, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::ShoutyKebab)
    };
}

#[test]
fn test_conv_ascii_case() {
    macro_rules! test_conv_ascii_case {
        ($v: tt, $a: expr, $b: expr $(,)?) => {{
            const A: &str = $a;
            const B: &str = convert_ascii_case!($v, A);
            assert_eq!(B, $b);
            test_conv_ascii_case!(heck, $v, $a, $b);
        }};
        (heck, camel, $a: expr, $b: expr) => {{
            use heck::CamelCase;
            let c: String = $a.to_camel_case();
            assert_eq!(c.as_str(), $b, "heck");
        }};
        (heck, snake, $a: expr, $b: expr) => {{
            use heck::SnakeCase;
            let c: String = $a.to_snake_case();
            assert_eq!(c.as_str(), $b, "heck");
        }};
        (heck, kebab, $a: expr, $b: expr) => {{
            use heck::KebabCase;
            let c: String = $a.to_kebab_case();
            assert_eq!(c.as_str(), $b, "heck");
        }};
        (heck, shouty_snake, $a: expr, $b: expr) => {{
            use heck::ShoutySnakeCase;
            let c: String = $a.to_shouty_snake_case();
            assert_eq!(c.as_str(), $b, "heck");
        }};
        (heck, shouty_kebab, $a: expr, $b: expr) => {{
            use heck::ShoutyKebabCase;
            let c: String = $a.to_shouty_kebab_case();
            assert_eq!(c.as_str(), $b, "heck");
        }};
    }

    {
        const S: &str = "b.8";
        test_conv_ascii_case!(camel, S, "B8");
        test_conv_ascii_case!(snake, S, "b_8");
        test_conv_ascii_case!(kebab, S, "b-8");
        test_conv_ascii_case!(shouty_snake, S, "B_8");
        test_conv_ascii_case!(shouty_kebab, S, "B-8");
    }

    {
        const S: &str = "Hello World123!XMLHttp我4t5.c6.7b.8";
        test_conv_ascii_case!(camel, S, "HelloWorld123XmlHttp我4t5C6.7b8");
        test_conv_ascii_case!(snake, S, "hello_world123_xml_http_我_4t5_c6.7b_8");
        test_conv_ascii_case!(kebab, S, "hello-world123-xml-http-我-4t5-c6.7b-8");
        test_conv_ascii_case!(shouty_snake, S, "HELLO_WORLD123_XML_HTTP_我_4T5_C6.7B_8");
        test_conv_ascii_case!(shouty_kebab, S, "HELLO-WORLD123-XML-HTTP-我-4T5-C6.7B-8");
    }
    {
        const S: &str = "XMLHttpRequest";
        test_conv_ascii_case!(camel, S, "XmlHttpRequest");
        test_conv_ascii_case!(snake, S, "xml_http_request");
        test_conv_ascii_case!(kebab, S, "xml-http-request");
        test_conv_ascii_case!(shouty_snake, S, "XML_HTTP_REQUEST");
        test_conv_ascii_case!(shouty_kebab, S, "XML-HTTP-REQUEST");
    }
    {
        const S: &str = "  hello world  ";
        test_conv_ascii_case!(camel, S, "HelloWorld");
        test_conv_ascii_case!(snake, S, "hello_world");
        test_conv_ascii_case!(kebab, S, "hello-world");
        test_conv_ascii_case!(shouty_snake, S, "HELLO_WORLD");
        test_conv_ascii_case!(shouty_kebab, S, "HELLO-WORLD");
    }
    {
        const S: &str = "";
        test_conv_ascii_case!(camel, S, "");
        test_conv_ascii_case!(snake, S, "");
        test_conv_ascii_case!(kebab, S, "");
        test_conv_ascii_case!(shouty_snake, S, "");
        test_conv_ascii_case!(shouty_kebab, S, "");
    }
    {
        const S: &str = "_";
        test_conv_ascii_case!(camel, S, "");
        test_conv_ascii_case!(snake, S, "");
        test_conv_ascii_case!(kebab, S, "");
        test_conv_ascii_case!(shouty_snake, S, "");
        test_conv_ascii_case!(shouty_kebab, S, "");
    }
    {
        const S: &str = "1.2E3";
        test_conv_ascii_case!(camel, S, "1.2e3");
        test_conv_ascii_case!(snake, S, "1.2e3");
        test_conv_ascii_case!(kebab, S, "1.2e3");
        test_conv_ascii_case!(shouty_snake, S, "1.2E3");
        test_conv_ascii_case!(shouty_kebab, S, "1.2E3");
    }
    {
        const S: &str = "__a__b-c__d__";
        test_conv_ascii_case!(camel, S, "ABCD");
        test_conv_ascii_case!(snake, S, "a_b_c_d");
        test_conv_ascii_case!(kebab, S, "a-b-c-d");
        test_conv_ascii_case!(shouty_snake, S, "A_B_C_D");
        test_conv_ascii_case!(shouty_kebab, S, "A-B-C-D");
    }
    {
        const S: &str = "futures-core123";
        test_conv_ascii_case!(camel, S, "FuturesCore123");
        test_conv_ascii_case!(snake, S, "futures_core123");
        test_conv_ascii_case!(kebab, S, "futures-core123");
        test_conv_ascii_case!(shouty_snake, S, "FUTURES_CORE123");
        test_conv_ascii_case!(shouty_kebab, S, "FUTURES-CORE123");
    }
}

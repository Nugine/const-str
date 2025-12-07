#![allow(unsafe_code)]

use core::ops::Range;

use crate::__ctfe::StrBuf;
use crate::slice::subslice;

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
        assert!(s.len() + 1 == N);

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
                            (NonAscii, Digit) => {} // Don't create boundary between NonAscii and Digit
                            (NonAscii, Lower | Upper) => {} // Don't create boundary between NonAscii and alphabetic
                            (Lower | Upper, Digit) => {}    // or-pattens stable since 1.53
                            (Digit, Lower | Upper | NonAscii) => {}
                            (Lower | Upper, NonAscii) => {} // Don't create boundary between alphabetic and NonAscii
                            (_, Dot) => {}
                            (Dot, _) => match (k2, k0) {
                                (None, _) => push!(i),
                                (Some(_), _) => {
                                    push!(i - 1);
                                    push!(i);
                                }
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
    LowerCamel,
    UpperCamel,
    Title,
    Train,
    Snake,
    Kebab,
    ShoutySnake,
    ShoutyKebab,
}

impl AsciiCase {
    const fn get_seperator(&self) -> Option<u8> {
        match self {
            Self::Title => Some(b' '),
            Self::Snake | Self::ShoutySnake => Some(b'_'),
            Self::Train | Self::Kebab | Self::ShoutyKebab => Some(b'-'),
            _ => None,
        }
    }
}

pub struct ConvAsciiCase<T>(pub T, pub AsciiCase);

impl ConvAsciiCase<&str> {
    pub const fn output_len<const M: usize>(&self) -> usize {
        assert!(self.0.len() + 1 == M);

        use AsciiCase::*;
        match self.1 {
            Lower | Upper => self.0.len(),
            LowerCamel | UpperCamel | Title | Train | Snake | Kebab | ShoutySnake | ShoutyKebab => {
                let mut ans = 0;

                let has_sep = self.1.get_seperator().is_some();

                let boundaries = Boundaries::<M>::new(self.0);
                let words_count = boundaries.words_count();

                let mut i = 0;
                let mut is_starting_boundary: bool = true;

                while i < words_count {
                    let rng = boundaries.word_range(i);
                    let word = subslice(self.0.as_bytes(), rng);

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
        assert!(self.0.len() + 1 == M);

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
                    push!(s[pos].to_ascii_lowercase());
                }
            }
            Upper => {
                while pos < s.len() {
                    push!(s[pos].to_ascii_uppercase());
                }
            }
            LowerCamel | UpperCamel | Title | Train | Snake | Kebab | ShoutySnake | ShoutyKebab => {
                let sep = self.1.get_seperator();

                let boundaries = Boundaries::<M>::new(self.0);
                let words_count = boundaries.words_count();

                let mut i = 0;
                let mut is_starting_boundary = true;

                while i < words_count {
                    let rng = boundaries.word_range(i);
                    let word = subslice(self.0.as_bytes(), rng);

                    if !TokenKind::is_boundary_word(word) {
                        if let (Some(sep), false) = (sep, is_starting_boundary) {
                            push!(sep)
                        }
                        let mut j = 0;
                        while j < word.len() {
                            let b = match self.1 {
                                Snake | Kebab => word[j].to_ascii_lowercase(),
                                ShoutySnake | ShoutyKebab => word[j].to_ascii_uppercase(),
                                LowerCamel | UpperCamel | Title | Train => {
                                    let is_upper = match self.1 {
                                        LowerCamel => !is_starting_boundary && j == 0,
                                        UpperCamel | Title | Train => j == 0,
                                        _ => unreachable!(),
                                    };
                                    if is_upper {
                                        word[j].to_ascii_uppercase()
                                    } else {
                                        word[j].to_ascii_lowercase()
                                    }
                                }
                                _ => unreachable!(),
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

        assert!(pos == N);

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
        OUTPUT_BUF.as_str()
    }};
}

/// Converts a string slice to a specified case. Non-ascii characters are not affected.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// use const_str::convert_ascii_case;
///
/// const S1: &str = convert_ascii_case!(lower, "Lower Case");
/// const S2: &str = convert_ascii_case!(upper, "Upper Case");
/// const S3: &str = convert_ascii_case!(lower_camel, "lower camel case");
/// const S4: &str = convert_ascii_case!(upper_camel, "upper camel case");
/// const S5: &str = convert_ascii_case!(title, "title case");
/// const S6: &str = convert_ascii_case!(train, "train case");
/// const S7: &str = convert_ascii_case!(snake, "snake case");
/// const S8: &str = convert_ascii_case!(kebab, "kebab case");
/// const S9: &str = convert_ascii_case!(shouty_snake, "shouty snake case");
/// const S10: &str = convert_ascii_case!(shouty_kebab, "shouty kebab case");
///
/// assert_eq!(S1, "lower case");
/// assert_eq!(S2, "UPPER CASE");
/// assert_eq!(S3, "lowerCamelCase");
/// assert_eq!(S4, "UpperCamelCase");
/// assert_eq!(S5, "Title Case");
/// assert_eq!(S6, "Train-Case");
/// assert_eq!(S7, "snake_case");
/// assert_eq!(S8, "kebab-case");
/// assert_eq!(S9, "SHOUTY_SNAKE_CASE");
/// assert_eq!(S10, "SHOUTY-KEBAB-CASE");
/// ```
#[macro_export]
macro_rules! convert_ascii_case {
    (lower, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Lower)
    };
    (upper, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Upper)
    };
    (lower_camel, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::LowerCamel)
    };
    (upper_camel, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::UpperCamel)
    };
    (title, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Title)
    };
    (train, $s: expr) => {
        $crate::__conv_ascii_case!($s, $crate::__ctfe::AsciiCase::Train)
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_conv_ascii_case() {
        macro_rules! test_conv_ascii_case {
            ($v: tt, $a: expr, $b: expr $(,)?) => {{
                const A: &str = $a;
                const B: &str = convert_ascii_case!($v, A);
                assert_eq!(B, $b);
                test_conv_ascii_case!(heck, $v, $a, $b);
            }};
            (heck, assert_eq, $c: expr, $b: expr) => {{
                if $c != $b {
                    println!("heck mismatch:\nheck:     {:?}\nexpected: {:?}\n", $c, $b);
                }
            }};
            (heck, lower_camel, $a: expr, $b: expr) => {{
                use heck::ToLowerCamelCase;
                let c: String = $a.to_lower_camel_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, upper_camel, $a: expr, $b: expr) => {{
                use heck::ToUpperCamelCase;
                let c: String = $a.to_upper_camel_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, title, $a: expr, $b: expr) => {{
                use heck::ToTitleCase;
                let c: String = $a.to_title_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, train, $a: expr, $b: expr) => {{
                use heck::ToTrainCase;
                let c: String = $a.to_train_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, snake, $a: expr, $b: expr) => {{
                use heck::ToSnakeCase;
                let c: String = $a.to_snake_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, kebab, $a: expr, $b: expr) => {{
                use heck::ToKebabCase;
                let c: String = $a.to_kebab_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, shouty_snake, $a: expr, $b: expr) => {{
                use heck::ToShoutySnakeCase;
                let c: String = $a.to_shouty_snake_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
            (heck, shouty_kebab, $a: expr, $b: expr) => {{
                use heck::ToShoutyKebabCase;
                let c: String = $a.to_shouty_kebab_case();
                test_conv_ascii_case!(heck, assert_eq, c.as_str(), $b);
            }};
        }

        {
            const S: &str = "b.8";
            test_conv_ascii_case!(lower_camel, S, "b8");
            test_conv_ascii_case!(upper_camel, S, "B8");
            test_conv_ascii_case!(title, S, "B 8");
            test_conv_ascii_case!(train, S, "B-8");
            test_conv_ascii_case!(snake, S, "b_8");
            test_conv_ascii_case!(kebab, S, "b-8");
            test_conv_ascii_case!(shouty_snake, S, "B_8");
            test_conv_ascii_case!(shouty_kebab, S, "B-8");
        }

        {
            const S: &str = "Hello World123!XMLHttp我4t5.c6.7b.8";
            test_conv_ascii_case!(lower_camel, S, "helloWorld123XmlHttp我4t5C67b8");
            test_conv_ascii_case!(upper_camel, S, "HelloWorld123XmlHttp我4t5C67b8");
            test_conv_ascii_case!(title, S, "Hello World123 Xml Http我4t5 C6 7b 8");
            test_conv_ascii_case!(train, S, "Hello-World123-Xml-Http我4t5-C6-7b-8");
            test_conv_ascii_case!(snake, S, "hello_world123_xml_http我4t5_c6_7b_8");
            test_conv_ascii_case!(kebab, S, "hello-world123-xml-http我4t5-c6-7b-8");
            test_conv_ascii_case!(shouty_snake, S, "HELLO_WORLD123_XML_HTTP我4T5_C6_7B_8");
            test_conv_ascii_case!(shouty_kebab, S, "HELLO-WORLD123-XML-HTTP我4T5-C6-7B-8");
        }
        {
            const S: &str = "XMLHttpRequest";
            test_conv_ascii_case!(lower_camel, S, "xmlHttpRequest");
            test_conv_ascii_case!(upper_camel, S, "XmlHttpRequest");
            test_conv_ascii_case!(title, S, "Xml Http Request");
            test_conv_ascii_case!(train, S, "Xml-Http-Request");
            test_conv_ascii_case!(snake, S, "xml_http_request");
            test_conv_ascii_case!(kebab, S, "xml-http-request");
            test_conv_ascii_case!(shouty_snake, S, "XML_HTTP_REQUEST");
            test_conv_ascii_case!(shouty_kebab, S, "XML-HTTP-REQUEST");
        }
        {
            const S: &str = "  hello world  ";
            test_conv_ascii_case!(lower_camel, S, "helloWorld");
            test_conv_ascii_case!(upper_camel, S, "HelloWorld");
            test_conv_ascii_case!(title, S, "Hello World");
            test_conv_ascii_case!(train, S, "Hello-World");
            test_conv_ascii_case!(snake, S, "hello_world");
            test_conv_ascii_case!(kebab, S, "hello-world");
            test_conv_ascii_case!(shouty_snake, S, "HELLO_WORLD");
            test_conv_ascii_case!(shouty_kebab, S, "HELLO-WORLD");
        }
        {
            const S: &str = "";
            test_conv_ascii_case!(lower_camel, S, "");
            test_conv_ascii_case!(upper_camel, S, "");
            test_conv_ascii_case!(title, S, "");
            test_conv_ascii_case!(train, S, "");
            test_conv_ascii_case!(snake, S, "");
            test_conv_ascii_case!(kebab, S, "");
            test_conv_ascii_case!(shouty_snake, S, "");
            test_conv_ascii_case!(shouty_kebab, S, "");
        }
        {
            const S: &str = "_";
            test_conv_ascii_case!(lower_camel, S, "");
            test_conv_ascii_case!(upper_camel, S, "");
            test_conv_ascii_case!(title, S, "");
            test_conv_ascii_case!(train, S, "");
            test_conv_ascii_case!(snake, S, "");
            test_conv_ascii_case!(kebab, S, "");
            test_conv_ascii_case!(shouty_snake, S, "");
            test_conv_ascii_case!(shouty_kebab, S, "");
        }
        {
            const S: &str = "1.2E3";
            test_conv_ascii_case!(lower_camel, S, "12e3");
            test_conv_ascii_case!(upper_camel, S, "12e3");
            test_conv_ascii_case!(title, S, "1 2e3");
            test_conv_ascii_case!(train, S, "1-2e3");
            test_conv_ascii_case!(snake, S, "1_2e3");
            test_conv_ascii_case!(kebab, S, "1-2e3");
            test_conv_ascii_case!(shouty_snake, S, "1_2E3");
            test_conv_ascii_case!(shouty_kebab, S, "1-2E3");
        }
        {
            const S: &str = "__a__b-c__d__";
            test_conv_ascii_case!(lower_camel, S, "aBCD");
            test_conv_ascii_case!(upper_camel, S, "ABCD");
            test_conv_ascii_case!(title, S, "A B C D");
            test_conv_ascii_case!(train, S, "A-B-C-D");
            test_conv_ascii_case!(snake, S, "a_b_c_d");
            test_conv_ascii_case!(kebab, S, "a-b-c-d");
            test_conv_ascii_case!(shouty_snake, S, "A_B_C_D");
            test_conv_ascii_case!(shouty_kebab, S, "A-B-C-D");
        }
        {
            const S: &str = "futures-core123";
            test_conv_ascii_case!(lower_camel, S, "futuresCore123");
            test_conv_ascii_case!(upper_camel, S, "FuturesCore123");
            test_conv_ascii_case!(title, S, "Futures Core123");
            test_conv_ascii_case!(train, S, "Futures-Core123");
            test_conv_ascii_case!(snake, S, "futures_core123");
            test_conv_ascii_case!(kebab, S, "futures-core123");
            test_conv_ascii_case!(shouty_snake, S, "FUTURES_CORE123");
            test_conv_ascii_case!(shouty_kebab, S, "FUTURES-CORE123");
        }
    }

    #[test]
    fn test_conv_ascii_case_runtime() {
        use super::*;

        // Test Lower case
        let conv_lower = ConvAsciiCase("HELLO", AsciiCase::Lower);
        let len_lower = conv_lower.output_len::<6>();
        assert_eq!(len_lower, 5);
        let result_lower: StrBuf<5> = conv_lower.const_eval::<6, 5>();
        assert_eq!(result_lower.as_str(), "hello");

        // Test Upper case
        let conv_upper = ConvAsciiCase("hello", AsciiCase::Upper);
        let len_upper = conv_upper.output_len::<6>();
        assert_eq!(len_upper, 5);
        let result_upper: StrBuf<5> = conv_upper.const_eval::<6, 5>();
        assert_eq!(result_upper.as_str(), "HELLO");

        // Test LowerCamel case
        let conv_camel = ConvAsciiCase("hello_world", AsciiCase::LowerCamel);
        let _len_camel = conv_camel.output_len::<12>();
        let result_camel: StrBuf<10> = conv_camel.const_eval::<12, 10>();
        assert_eq!(result_camel.as_str(), "helloWorld");

        // Test UpperCamel case
        let conv_upper_camel = ConvAsciiCase("hello_world", AsciiCase::UpperCamel);
        let _len_upper_camel = conv_upper_camel.output_len::<12>();
        let result_upper_camel: StrBuf<10> = conv_upper_camel.const_eval::<12, 10>();
        assert_eq!(result_upper_camel.as_str(), "HelloWorld");

        // Test Title case
        let conv_title = ConvAsciiCase("hello_world", AsciiCase::Title);
        let _len_title = conv_title.output_len::<12>();
        let result_title: StrBuf<11> = conv_title.const_eval::<12, 11>();
        assert_eq!(result_title.as_str(), "Hello World");

        // Test Train case
        let conv_train = ConvAsciiCase("hello_world", AsciiCase::Train);
        let _len_train = conv_train.output_len::<12>();
        let result_train: StrBuf<11> = conv_train.const_eval::<12, 11>();
        assert_eq!(result_train.as_str(), "Hello-World");

        // Test Snake case
        let conv_snake = ConvAsciiCase("HelloWorld", AsciiCase::Snake);
        let _len_snake = conv_snake.output_len::<11>();
        let result_snake: StrBuf<11> = conv_snake.const_eval::<11, 11>();
        assert_eq!(result_snake.as_str(), "hello_world");

        // Test Kebab case
        let conv_kebab = ConvAsciiCase("HelloWorld", AsciiCase::Kebab);
        let _len_kebab = conv_kebab.output_len::<11>();
        let result_kebab: StrBuf<11> = conv_kebab.const_eval::<11, 11>();
        assert_eq!(result_kebab.as_str(), "hello-world");

        // Test ShoutySnake case
        let conv_shouty_snake = ConvAsciiCase("helloWorld", AsciiCase::ShoutySnake);
        let _len_shouty_snake = conv_shouty_snake.output_len::<11>();
        let result_shouty_snake: StrBuf<11> = conv_shouty_snake.const_eval::<11, 11>();
        assert_eq!(result_shouty_snake.as_str(), "HELLO_WORLD");

        // Test ShoutyKebab case
        let conv_shouty_kebab = ConvAsciiCase("helloWorld", AsciiCase::ShoutyKebab);
        let _len_shouty_kebab = conv_shouty_kebab.output_len::<11>();
        let result_shouty_kebab: StrBuf<11> = conv_shouty_kebab.const_eval::<11, 11>();
        assert_eq!(result_shouty_kebab.as_str(), "HELLO-WORLD");

        // Test edge cases with numbers and dots
        let conv_edge = ConvAsciiCase("1.2E3", AsciiCase::LowerCamel);
        let _len_edge = conv_edge.output_len::<6>();
        let result_edge: StrBuf<4> = conv_edge.const_eval::<6, 4>();
        assert_eq!(result_edge.as_str(), "12e3");

        // Test empty-ish strings
        let conv_empty = ConvAsciiCase("___", AsciiCase::LowerCamel);
        let _len_empty = conv_empty.output_len::<4>();
        let result_empty: StrBuf<0> = conv_empty.const_eval::<4, 0>();
        assert_eq!(result_empty.as_str(), "");
    }
}

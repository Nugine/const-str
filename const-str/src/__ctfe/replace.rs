#![allow(unsafe_code)]

use crate::slice::advance;
use crate::utf8::CharEncodeUtf8;

use super::str::StrBuf;

pub struct Replace<I, P, O>(pub I, pub P, pub O);

impl<'input, 'from, 'to> Replace<&'input str, &'from str, &'to str> {
    pub const fn output_len(&self) -> usize {
        let Self(mut input, replace_from, replace_to) = *self;

        if replace_from.is_empty() {
            let input_chars = crate::utf8::str_count_chars(self.0);
            input.len() + (input_chars + 1) * replace_to.len()
        } else {
            let mut ans = 0;
            while let Some((pos, remain)) = crate::str::next_match(input, replace_from) {
                ans += pos + replace_to.len();
                input = remain;
            }
            ans += input.len();
            ans
        }
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let Self(input, replace_from, replace_to) = *self;

        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x: expr) => {{
                buf[pos] = $x;
                pos += 1;
            }};
        }

        if replace_from.is_empty() {
            let mut input = input.as_bytes();
            let replace_to = replace_to.as_bytes();
            loop {
                let mut k = 0;
                while k < replace_to.len() {
                    push!(replace_to[k]);
                    k += 1;
                }

                let count = match crate::utf8::next_char(input) {
                    Some((_, count)) => count,
                    None => break,
                };

                let mut i = 0;
                while i < count {
                    push!(input[i]);
                    i += 1;
                }

                input = advance(input, count);
            }
        } else {
            let mut input = input;
            let replace_to = replace_to.as_bytes();

            while let Some((pos, remain)) = crate::str::next_match(input, replace_from) {
                let mut i = 0;
                while i < pos {
                    push!(input.as_bytes()[i]);
                    i += 1;
                }
                let mut k = 0;
                while k < replace_to.len() {
                    push!(replace_to[k]);
                    k += 1;
                }
                input = remain;
            }

            let input = input.as_bytes();
            let mut i = 0;
            while i < input.len() {
                push!(input[i]);
                i += 1;
            }
        }

        assert!(pos == N);
        unsafe { StrBuf::new_unchecked(buf) }
    }
}

impl<'input, 'to> Replace<&'input str, char, &'to str> {
    pub const fn output_len(&self) -> usize {
        let ch = CharEncodeUtf8::new(self.1);
        Replace(self.0, ch.as_str(), self.2).output_len()
    }
    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let ch = CharEncodeUtf8::new(self.1);
        Replace(self.0, ch.as_str(), self.2).const_eval()
    }
}

/// Replaces all matches of a pattern with another string slice.
///
/// See [`str::replace`](https://doc.rust-lang.org/std/primitive.str.html#method.replace).
///
/// The pattern type must be one of
///
/// + [`&str`](str)
/// + [`char`]
///
/// # Examples
///
/// ```
/// assert_eq!("this is new", const_str::replace!("this is old", "old", "new"));
/// ```
///
#[macro_export]
macro_rules! replace {
    ($s: expr, $from: expr, $to: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::Replace($s, $from, $to).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::Replace($s, $from, $to).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        macro_rules! testcase {
            ($input: expr, $from: expr, $to: expr) => {{
                const OUTPUT_LEN: usize = Replace($input, $from, $to).output_len();
                const OUTPUT_BUF: StrBuf<OUTPUT_LEN> = Replace($input, $from, $to).const_eval();
                const OUTPUT: &str = OUTPUT_BUF.as_str();

                let ans = $input.replace($from, $to);
                assert_eq!(OUTPUT, &*ans, "ans = {:?}", ans);
                assert_eq!(OUTPUT_LEN, ans.len());
            }};
        }

        testcase!("", "", "");
        testcase!("", "", "a");
        testcase!("", "a", "");
        testcase!("", "a", "b");
        testcase!("a", "", "b");
        testcase!("asd", "", "b");
        testcase!("aba", "a", "c");
        testcase!("this is old", "old", "new");
        testcase!("我", "", "1");
        testcase!("我", "", "我");
        testcase!("我", "我", "");
        testcase!("aaaa", "aa", "bb");
        testcase!("run / v4", " ", "");
        testcase!("token", " ", "");
        testcase!("v4 / udp", " ", "");
        testcase!("v4 / upnp", "p", "");

        testcase!("", 'a', "");
        testcase!("", 'a', "b");
        testcase!("aba", 'a', "c");
        testcase!("run / v4", ' ', "");
        testcase!("token", ' ', "");
        testcase!("v4 / udp", ' ', "");
        testcase!("我", '我', "");
    }
}

#![allow(unsafe_code)]

use crate::slice::advance;
use crate::utf8::CharEncodeUtf8;

use super::str::StrBuf;

pub struct Replace<I, P, O>(pub I, pub P, pub O);

impl Replace<&str, &str, &str> {
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

impl Replace<&str, char, &str> {
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
/// This macro is [const-context only](./index.html#const-context-only).
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

    #[test]
    fn test_replace_runtime() {
        // Runtime tests for Replace
        let replace1 = Replace("hello world", "world", "rust");
        assert_eq!(replace1.output_len(), 10);
        let buf1: StrBuf<10> = replace1.const_eval();
        assert_eq!(buf1.as_str(), "hello rust");

        let replace2 = Replace("aaa", "a", "bb");
        assert_eq!(replace2.output_len(), 6);
        let buf2: StrBuf<6> = replace2.const_eval();
        assert_eq!(buf2.as_str(), "bbbbbb");

        let replace3 = Replace("test", "x", "y");
        assert_eq!(replace3.output_len(), 4);
        let buf3: StrBuf<4> = replace3.const_eval();
        assert_eq!(buf3.as_str(), "test");

        let replace_empty = Replace("", "a", "b");
        let len_empty = replace_empty.output_len();
        assert_eq!(len_empty, 0);

        // Test with char pattern
        let replace_char = Replace("hello", 'l', "L");
        assert_eq!(replace_char.output_len(), 5);
        let buf_char: StrBuf<5> = replace_char.const_eval();
        assert_eq!(buf_char.as_str(), "heLLo");
    }

    #[test]
    fn test_replace_empty_pattern() {
        // Test replacing with empty "from" pattern
        // This inserts "to" between every character
        
        // Empty string with empty pattern
        let r1 = Replace("", "", "");
        assert_eq!(r1.output_len(), 0);
        let buf1: StrBuf<0> = r1.const_eval();
        assert_eq!(buf1.as_str(), "");
        
        // Empty string, empty pattern, non-empty replacement
        let r2 = Replace("", "", "x");
        assert_eq!(r2.output_len(), 1);
        let buf2: StrBuf<1> = r2.const_eval();
        assert_eq!(buf2.as_str(), "x");
        
        // Single char with empty pattern
        let r3 = Replace("a", "", "x");
        assert_eq!(r3.output_len(), 3);
        let buf3: StrBuf<3> = r3.const_eval();
        assert_eq!(buf3.as_str(), "xax");
        
        // Multiple chars with empty pattern
        let r4 = Replace("ab", "", "x");
        assert_eq!(r4.output_len(), 5);
        let buf4: StrBuf<5> = r4.const_eval();
        assert_eq!(buf4.as_str(), "xaxbx");
        
        // Multi-byte UTF-8 character with empty pattern
        let r5 = Replace("我", "", "x");
        assert_eq!(r5.output_len(), 5);
        let buf5: StrBuf<5> = r5.const_eval();
        assert_eq!(buf5.as_str(), "x我x");
        
        // Multiple multi-byte characters with empty pattern
        let r6 = Replace("我好", "", "x");
        assert_eq!(r6.output_len(), 9);
        let buf6: StrBuf<9> = r6.const_eval();
        assert_eq!(buf6.as_str(), "x我x好x");
    }
}

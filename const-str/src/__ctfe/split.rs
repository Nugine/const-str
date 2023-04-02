use crate::slice::advance;
use crate::slice::subslice;
use crate::utf8::CharEncodeUtf8;

use core::str;

pub struct Split<T, P>(pub T, pub P);

impl<'input, 'pat> Split<&'input str, &'pat str> {
    pub const fn output_len(&self) -> usize {
        let Self(mut input, pat) = *self;

        if pat.is_empty() {
            crate::utf8::str_count_chars(input) + 2
        } else {
            let mut ans = 1;
            while let Some((_, remain)) = crate::str::next_match(input, pat) {
                ans += 1;
                input = remain
            }
            ans
        }
    }

    #[allow(unsafe_code)]
    pub const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        let Self(mut input, pat) = *self;

        let mut buf: [&str; N] = [""; N];
        let mut pos = 0;

        if pat.is_empty() {
            let mut input = input.as_bytes();

            {
                buf[pos] = unsafe { str::from_utf8_unchecked(subslice(input, 0..0)) };
                pos += 1;
            }

            while let Some((_, count)) = crate::utf8::next_char(input) {
                buf[pos] = unsafe { str::from_utf8_unchecked(subslice(input, 0..count)) };
                pos += 1;
                input = advance(input, count);
            }

            {
                buf[pos] = unsafe { str::from_utf8_unchecked(subslice(input, 0..0)) };
                pos += 1;
            }
        } else {
            while let Some((m, remain)) = crate::str::next_match(input, pat) {
                let substr = subslice(input.as_bytes(), 0..m);
                buf[pos] = unsafe { str::from_utf8_unchecked(substr) };
                pos += 1;
                input = remain;
            }
            buf[pos] = input;
            pos += 1;
        }
        assert!(pos == N);
        buf
    }
}

impl<'input> Split<&'input str, char> {
    pub const fn output_len(&self) -> usize {
        let ch = CharEncodeUtf8::new(self.1);
        Split(self.0, ch.as_str()).output_len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        let ch = CharEncodeUtf8::new(self.1);
        Split(self.0, ch.as_str()).const_eval()
    }
}

/// Returns an array of substrings of a string slice, separated by characters matched by a pattern.
///
/// See [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split).
///
/// The pattern type must be one of
///
/// + [`&str`](prim@str)
/// + [`char`]
///
/// # Examples
///
/// ```
/// const SEPARATOR: &str = ", ";
/// const TEXT: &str = "lion, tiger, leopard";
///
/// const ANIMALS_ARRAY: [&str;3] = const_str::split!(TEXT, SEPARATOR);
/// const ANIMALS_SLICE: &[&str] = &const_str::split!(TEXT, SEPARATOR);
///
/// assert_eq!(ANIMALS_ARRAY, ANIMALS_SLICE);
/// assert_eq!(ANIMALS_SLICE, &["lion", "tiger", "leopard"]);
/// ```
#[macro_export]
macro_rules! split {
    ($s: expr, $pat: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::Split($s, $pat).output_len();
        const OUTPUT_BUF: [&str; OUTPUT_LEN] = $crate::__ctfe::Split($s, $pat).const_eval();
        OUTPUT_BUF
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        macro_rules! testcase {
            ($input: expr, $pat: expr) => {{
                const OUTPUT_LEN: usize = Split($input, $pat).output_len();
                const OUTPUT: &[&str] = &Split($input, $pat).const_eval::<OUTPUT_LEN>();

                let ans = $input.split($pat).collect::<Vec<_>>();
                assert_eq!(OUTPUT, &*ans, "ans = {:?}", ans);
                assert_eq!(OUTPUT_LEN, ans.len());
            }};
        }

        testcase!("", "");
        testcase!("a中1😂1!", "");
        testcase!("a中1😂1!", "a");
        testcase!("a中1😂1!", "中");
        testcase!("a中1😂1!", "1");
        testcase!("a中1😂1!", "😂");
        testcase!("a中1😂1!", "!");
        testcase!("11111", "1");
        testcase!("222", "22");
        testcase!("啊哈哈哈", "哈哈");
        testcase!("some string:another string", ":");

        testcase!("11111", '1');
        testcase!("a中1😂1!", 'a');
        testcase!("a中1😂1!", '中');
        testcase!("a中1😂1!", '1');
        testcase!("a中1😂1!", '😂');
        testcase!("a中1😂1!", '!');
    }
}

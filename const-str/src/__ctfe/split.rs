use crate::slice::advance;
use crate::slice::subslice;
use crate::utf8::CharEncodeUtf8;

use core::str;

struct SplitImpl<'input, 'pat> {
    input: &'input str,
    pattern: &'pat str,
    inclusive: bool,
}

impl<'input> SplitImpl<'input, '_> {
    const fn output_len(&self) -> usize {
        let mut input = self.input;
        let pat = self.pattern;

        if pat.is_empty() {
            crate::utf8::str_count_chars(input) + 2
        } else {
            let mut ans = 0;
            while let Some((_, remain)) = crate::str::next_match(input, pat) {
                ans += 1;
                input = remain
            }
            if self.inclusive {
                if !input.is_empty() {
                    ans += 1;
                }
            } else {
                ans += 1;
            }
            ans
        }
    }

    #[allow(unsafe_code)]
    const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        let mut input = self.input;
        let pat = self.pattern;

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
                let substr = if self.inclusive {
                    subslice(input.as_bytes(), 0..m + pat.len())
                } else {
                    subslice(input.as_bytes(), 0..m)
                };
                buf[pos] = unsafe { str::from_utf8_unchecked(substr) };
                pos += 1;
                input = remain;
            }
            if self.inclusive {
                if !input.is_empty() {
                    buf[pos] = input;
                    pos += 1;
                }
            } else {
                buf[pos] = input;
                pos += 1;
            }
        }
        assert!(pos == N);
        buf
    }
}

pub struct Split<T, P>(pub T, pub P);

impl<'input, 'pat> Split<&'input str, &'pat str> {
    const fn to_impl(&self) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: self.1,
            inclusive: false,
        }
    }

    pub const fn output_len(&self) -> usize {
        self.to_impl().output_len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        self.to_impl().const_eval()
    }
}

impl<'input> Split<&'input str, char> {
    const fn to_impl<'pat>(&self, ch: &'pat CharEncodeUtf8) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: ch.as_str(),
            inclusive: false,
        }
    }

    pub const fn output_len(&self) -> usize {
        let ch = CharEncodeUtf8::new(self.1);
        self.to_impl(&ch).output_len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        let ch = CharEncodeUtf8::new(self.1);
        self.to_impl(&ch).const_eval()
    }
}

/// Returns an array of substrings of a string slice, separated by characters matched by a pattern.
///
/// The pattern type must be one of
///
/// + [`&str`](prim@str)
/// + [`char`]
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split).
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
        const INPUT: &str = $s;
        const OUTPUT_LEN: usize = $crate::__ctfe::Split(INPUT, $pat).output_len();
        const OUTPUT_BUF: [&str; OUTPUT_LEN] = $crate::__ctfe::Split(INPUT, $pat).const_eval();
        OUTPUT_BUF
    }};
}

pub struct SplitInclusive<T, P>(pub T, pub P);

impl<'input, 'pat> SplitInclusive<&'input str, &'pat str> {
    const fn to_impl(&self) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: self.1,
            inclusive: true,
        }
    }

    pub const fn output_len(&self) -> usize {
        self.to_impl().output_len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        self.to_impl().const_eval()
    }
}

impl<'input> SplitInclusive<&'input str, char> {
    const fn to_impl<'pat>(&self, ch: &'pat CharEncodeUtf8) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: ch.as_str(),
            inclusive: true,
        }
    }

    pub const fn output_len(&self) -> usize {
        let ch = CharEncodeUtf8::new(self.1);
        self.to_impl(&ch).output_len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        let ch = CharEncodeUtf8::new(self.1);
        self.to_impl(&ch).const_eval()
    }
}

/// Returns an array of substrings of a string slice, separated by characters matched by a pattern.
///
/// Differs from the array produced by [`split!`] in that
/// [`split_inclusive!`](crate::split_inclusive) leaves the matched part as the terminator of the substring.
///
/// If the last element of the string is matched,
/// that element will be considered the terminator of the preceding substring.
/// That substring will be the last item returned by the iterator.
///
/// The pattern type must be one of
///
/// + [`&str`](prim@str)
/// + [`char`]
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`str::split_inclusive`](https://doc.rust-lang.org/std/primitive.str.html#method.split_inclusive).
///
/// # Examples
/// ```
/// const TEXT: &str = "Mary had a little lamb\nlittle lamb\nlittle lamb.";
/// const ANSWER:&[&str] = &const_str::split_inclusive!(TEXT, "\n");
/// assert_eq!(ANSWER, &["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
/// ```
/// ```
/// const TEXT: &str = "\nA\nB\nC\n";
/// const ANSWER:&[&str] = &const_str::split_inclusive!(TEXT, "\n");
/// assert_eq!(ANSWER, &["\n", "A\n", "B\n", "C\n"]);
/// ```
#[macro_export]
macro_rules! split_inclusive {
    ($s: expr, $pat: expr) => {{
        const INPUT: &str = $s;
        const OUTPUT_LEN: usize = $crate::__ctfe::SplitInclusive(INPUT, $pat).output_len();
        const OUTPUT_BUF: [&str; OUTPUT_LEN] =
            $crate::__ctfe::SplitInclusive(INPUT, $pat).const_eval();
        OUTPUT_BUF
    }};
}

pub const fn map_lines<const N: usize>(mut lines: [&str; N]) -> [&str; N] {
    let mut i = 0;
    while i < N {
        let s = lines[i];
        match crate::str::strip_suffix(s, "\r\n") {
            Some(s) => lines[i] = s,
            None => match crate::str::strip_suffix(s, "\n") {
                Some(s) => lines[i] = s,
                None => lines[i] = s,
            },
        }
        i += 1;
    }
    lines
}

/// Returns an array of the lines in a string.
///
/// Lines are split by LF (`\n`) or CRLF (`\r\n`).
///
/// Line terminators are not included in the returned array.
///
/// The final line ending is optional.
/// A string that ends with a final line ending will return the same lines
/// as an otherwise identical string without a final line ending.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
///
/// # Examples
/// ```rust
/// const TEXT: &str = "foo\r\nbar\n\nbaz\r";
/// const LINES_ARRAY: [&str;4] = const_str::split_lines!(TEXT);
/// const LINES_SLICE: &[&str] = &const_str::split_lines!(TEXT);
///
/// assert_eq!(LINES_ARRAY, LINES_SLICE);
/// assert_eq!(LINES_SLICE, &["foo", "bar", "", "baz\r"]);
/// ```
/// ```rust
/// const TEXT1: &str = "1\r\n2\r\n3\r\n";
/// const TEXT2: &str = "1\n2\n3\n";
/// const TEXT3: &str = "1\n2\n3";
/// const LINES1: &[&str] = &const_str::split_lines!(TEXT1);
/// const LINES2: &[&str] = &const_str::split_lines!(TEXT2);
/// const LINES3: &[&str] = &const_str::split_lines!(TEXT3);
/// assert_eq!(LINES1, LINES2);
/// assert_eq!(LINES2, LINES3);
/// ```
#[macro_export]
macro_rules! split_lines {
    ($s: expr) => {{
        $crate::__ctfe::map_lines($crate::split_inclusive!($s, "\n"))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        macro_rules! testcase {
            ($input: expr, $pat: expr) => {{
                const OUTPUT: &[&str] = &$crate::split!($input, $pat);

                let ans = $input.split($pat).collect::<Vec<_>>();
                assert_eq!(OUTPUT.len(), ans.len());
                assert_eq!(OUTPUT, &*ans, "ans = {:?}", ans);
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

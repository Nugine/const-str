use crate::slice::advance;
use crate::slice::subslice;
use crate::utf8::CharEncodeUtf8;

use core::str;

enum Pattern<'pat> {
    Str(&'pat str),
    CharSlice(&'pat [char]),
}

struct SplitImpl<'input, 'pat> {
    input: &'input str,
    pattern: Pattern<'pat>,
    inclusive: bool,
}

impl<'input> SplitImpl<'input, '_> {
    const fn output_len(&self) -> usize {
        let mut input = self.input;

        match self.pattern {
            Pattern::Str(pat) => {
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
            Pattern::CharSlice(chars) => {
                if chars.is_empty() {
                    1  // If no chars to split on, return the whole string
                } else {
                    let mut ans = 0;
                    while let Some((_, remain)) = crate::str::next_match_char_slice(input, chars) {
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
        }
    }

    #[allow(unsafe_code)]
    const fn const_eval<const N: usize>(&self) -> [&'input str; N] {
        let mut input = self.input;

        let mut buf: [&str; N] = [""; N];
        let mut pos = 0;

        match self.pattern {
            Pattern::Str(pat) => {
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
            }
            Pattern::CharSlice(chars) => {
                if chars.is_empty() {
                    buf[pos] = input;
                    pos += 1;
                } else {
                    while let Some((m, remain)) = crate::str::next_match_char_slice(input, chars) {
                        let substr = if self.inclusive {
                            // For char slice, we need to determine how many bytes the matched char takes
                            let original_len = input.len();
                            let remain_len = remain.len();
                            let matched_char_len = original_len - remain_len - m;
                            subslice(input.as_bytes(), 0..m + matched_char_len)
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
            pattern: Pattern::Str(self.1),
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
            pattern: Pattern::Str(ch.as_str()),
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

impl<'input, 'pat> Split<&'input str, &'pat [char]> {
    const fn to_impl(&self) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: Pattern::CharSlice(self.1),
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

impl<'input, 'pat, const LEN: usize> Split<&'input str, &'pat [char; LEN]> {
    const fn to_impl(&self) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: Pattern::CharSlice(self.1.as_slice()),
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

/// Returns an array of substrings of a string slice, separated by characters matched by a pattern.
///
/// The pattern type must be one of
///
/// + [`&str`](prim@str)
/// + [`char`]
/// + [`&[char]`][slice]
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
///
/// Split by any character in a slice:
/// ```
/// const TEXT: &str = "hello,world;foo:bar";
/// const DELIMITERS: &[char] = &[',', ';', ':'];
/// const PARTS: &[&str] = &const_str::split!(TEXT, DELIMITERS);
/// 
/// assert_eq!(PARTS, &["hello", "world", "foo", "bar"]);
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
            pattern: Pattern::Str(self.1),
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
            pattern: Pattern::Str(ch.as_str()),
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

impl<'input, 'pat> SplitInclusive<&'input str, &'pat [char]> {
    const fn to_impl(&self) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: Pattern::CharSlice(self.1),
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

impl<'input, 'pat, const LEN: usize> SplitInclusive<&'input str, &'pat [char; LEN]> {
    const fn to_impl(&self) -> SplitImpl<'input, 'pat> {
        SplitImpl {
            input: self.0,
            pattern: Pattern::CharSlice(self.1.as_slice()),
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
/// + [`&[char]`][slice]
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
/// 
/// Split inclusive by any character in a slice:
/// ```
/// const TEXT: &str = "a,b;c:d";
/// const DELIMITERS: &[char] = &[',', ';', ':'];
/// const PARTS: &[&str] = &const_str::split_inclusive!(TEXT, DELIMITERS);
/// 
/// assert_eq!(PARTS, &["a,", "b;", "c:", "d"]);
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

pub struct SplitAsciiWhitespace<T>(pub T);

impl SplitAsciiWhitespace<&'_ str> {
    pub const fn output_len(&self) -> usize {
        let bytes = self.0.as_bytes();
        let mut count = 0;
        let mut i = 0;
        let mut in_word = false;

        while i < bytes.len() {
            if bytes[i].is_ascii_whitespace() {
                if in_word {
                    count += 1;
                    in_word = false;
                }
            } else {
                in_word = true;
            }
            i += 1;
        }

        if in_word {
            count += 1;
        }

        count
    }

    #[allow(unsafe_code)]
    pub const fn const_eval<const N: usize>(&self) -> [&'_ str; N] {
        let bytes = self.0.as_bytes();
        let mut buf: [&str; N] = [""; N];
        let mut pos = 0;
        let mut i = 0;

        while i < bytes.len() {
            // Skip leading whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }

            if i >= bytes.len() {
                break;
            }

            // Mark start of word
            let start = i;

            // Find end of word
            while i < bytes.len() && !bytes[i].is_ascii_whitespace() {
                i += 1;
            }

            // Extract word
            let word_bytes = subslice(bytes, start..i);
            buf[pos] = unsafe { core::str::from_utf8_unchecked(word_bytes) };
            pos += 1;
        }

        assert!(pos == N);
        buf
    }
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

/// Returns an array of substrings of a string slice, separated by ASCII whitespace.
///
/// ASCII whitespace characters are: space (` `), tab (`\t`), newline (`\n`),
/// carriage return (`\r`), and form feed (`\f`).
///
/// Consecutive whitespace characters are treated as a single separator.
/// Leading and trailing whitespace is ignored.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`str::split_ascii_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_ascii_whitespace).
///
/// # Examples
///
/// ```
/// const TEXT: &str = "  hello   world  ";
/// const WORDS_ARRAY: [&str; 2] = const_str::split_ascii_whitespace!(TEXT);
/// const WORDS_SLICE: &[&str] = &const_str::split_ascii_whitespace!(TEXT);
///
/// assert_eq!(WORDS_ARRAY, WORDS_SLICE);
/// assert_eq!(WORDS_SLICE, &["hello", "world"]);
/// ```
///
/// ```
/// const TEXT: &str = "word1\t\tword2\n\nword3";
/// const WORDS: &[&str] = &const_str::split_ascii_whitespace!(TEXT);
/// assert_eq!(WORDS, &["word1", "word2", "word3"]);
/// ```
#[macro_export]
macro_rules! split_ascii_whitespace {
    ($s: expr) => {{
        const INPUT: &str = $s;
        const OUTPUT_LEN: usize = $crate::__ctfe::SplitAsciiWhitespace(INPUT).output_len();
        const OUTPUT_BUF: [&str; OUTPUT_LEN] =
            $crate::__ctfe::SplitAsciiWhitespace(INPUT).const_eval();
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
        
        // Test with char slice
        macro_rules! testcase_char_slice {
            ($input: expr, $pat: expr) => {{
                const OUTPUT: &[&str] = &$crate::split!($input, $pat);

                let ans = $input.split($pat as &[char]).collect::<Vec<_>>();
                assert_eq!(OUTPUT.len(), ans.len());
                assert_eq!(OUTPUT, &*ans, "ans = {:?}", ans);
            }};
        }

        testcase_char_slice!("hello,world;test", &[',', ';']);
        testcase_char_slice!("a中1😂1!", &['a', '1']);
        testcase_char_slice!("a中1😂1!", &['中', '😂']);
        testcase_char_slice!("no match", &['x', 'y']);
        testcase_char_slice!("", &['a', 'b']);
        testcase_char_slice!("abc", &[]);
        testcase_char_slice!("aaa", &['a']);
        testcase_char_slice!("a,b;c:d", &[',', ';', ':']);
    }
    
    #[test]
    fn test_split_inclusive_char_slice() {
        macro_rules! testcase {
            ($input: expr, $pat: expr) => {{
                const OUTPUT: &[&str] = &$crate::split_inclusive!($input, $pat);

                let ans = $input.split_inclusive($pat as &[char]).collect::<Vec<_>>();
                assert_eq!(OUTPUT.len(), ans.len());
                assert_eq!(OUTPUT, &*ans, "ans = {:?}", ans);
            }};
        }

        testcase!("hello,world;test", &[',', ';']);
        testcase!("a中1😂1!", &['a', '1']);
        testcase!("a中1😂1!", &['中', '😂']);
        testcase!("no match", &['x', 'y']);
        testcase!("", &['a', 'b']);
        testcase!("abc", &[]);
        testcase!("aaa", &['a']);
        testcase!("a,b;c:d", &[',', ';', ':']);
    }

    #[test]
    fn test_split_ascii_whitespace() {
        macro_rules! testcase {
            ($input: expr) => {{
                const OUTPUT: &[&str] = &$crate::split_ascii_whitespace!($input);

                let ans = $input.split_ascii_whitespace().collect::<Vec<_>>();
                assert_eq!(
                    OUTPUT.len(),
                    ans.len(),
                    "Length mismatch for input: {:?}",
                    $input
                );
                assert_eq!(
                    OUTPUT, &*ans,
                    "Content mismatch for input: {:?}, expected: {:?}",
                    $input, ans
                );
            }};
        }

        // Basic cases
        testcase!("");
        testcase!(" ");
        testcase!("  ");
        testcase!("hello");
        testcase!(" hello ");
        testcase!("  hello  ");
        testcase!("hello world");
        testcase!(" hello world ");
        testcase!("  hello   world  ");

        // Different whitespace types
        testcase!("a\tb\nc\rd\x0Cf");
        testcase!(" \t\n\r\x0C ");
        testcase!("word1\t\t\tword2\n\n\nword3");

        // Mixed content
        testcase!("foo bar baz");
        testcase!("\tfoo\nbar\rbaz\x0C");
        testcase!("   a   b   c   ");
        testcase!("\t\n\r\x0C");

        // Edge cases
        testcase!("single");
        testcase!("a");
        testcase!("a b");
        testcase!("  a  b  ");
    }
}

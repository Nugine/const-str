#![allow(unsafe_code)]

use super::StrBuf;

pub struct Concat<'a>(pub &'a [&'a str]);

impl Concat<'_> {
    pub const fn output_len(&self) -> usize {
        let mut ans = 0;
        let mut iter = self.0;
        while let [x, xs @ ..] = iter {
            ans += x.len();
            iter = xs;
        }
        ans
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let mut buf = [0; N];
        let mut pos = 0;

        let mut iter = self.0;
        while let [x, xs @ ..] = iter {
            let x = x.as_bytes();
            let mut i = 0;
            while i < x.len() {
                buf[pos] = x[i];
                pos += 1;
                i += 1;
            }
            iter = xs;
        }
        assert!(pos == N);

        unsafe { StrBuf::new_unchecked(buf) }
    }
}

/// Concatenates values into a string slice.
///
/// The input type must be one of
///
/// + [`&str`]
/// + [`char`]
/// + [`bool`]
/// + [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`]
/// + [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`]
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// const PROMPT: &str = "The answer is";
/// const ANSWER: usize = 42;
/// const MESSAGE: &str = const_str::concat!(PROMPT, " ", ANSWER);
///
/// assert_eq!(MESSAGE, "The answer is 42");
/// ```
///
#[macro_export]
macro_rules! concat {
    ($($x: expr),+ $(,)?) => {{
        const STRS: &[&str] = &[$( $crate::to_str!($x) ),+];
        const OUTPUT_LEN: usize = $crate::__ctfe::Concat(STRS).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> = $crate::__ctfe::Concat(STRS).const_eval();
        OUTPUT_BUF.as_str()
    }}
}

pub struct Join<'a>(pub &'a [&'a str], pub &'a str);

impl Join<'_> {
    pub const fn output_len(&self) -> usize {
        let mut ans = 0;
        let mut i = 0;
        while i < self.0.len() {
            ans += self.0[i].len();
            if i < self.0.len() - 1 {
                ans += self.1.len();
            }
            i += 1;
        }
        ans
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let mut buf = [0; N];
        let mut pos = 0;

        let mut i = 0;
        while i < self.0.len() {
            let x = self.0[i].as_bytes();
            let mut j = 0;
            while j < x.len() {
                buf[pos] = x[j];
                pos += 1;
                j += 1;
            }
            if i < self.0.len() - 1 {
                let sep = self.1.as_bytes();
                let mut j = 0;
                while j < sep.len() {
                    buf[pos] = sep[j];
                    pos += 1;
                    j += 1;
                }
            }
            i += 1;
        }

        unsafe { StrBuf::new_unchecked(buf) }
    }
}

/// Concatenates string slices into a string slice, separated by a given separator.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// const WORDS: &[&str] = &["hello", "world"];
/// const MESSAGE1: &str = const_str::join!(WORDS, " ");
/// assert_eq!(MESSAGE1, "hello world");
///
/// const NUMS: &[&str] = &["1", "2", "3"];
/// const MESSAGE2: &str = const_str::join!(NUMS, ", ");
/// assert_eq!(MESSAGE2, "1, 2, 3");
///
/// const EMPTY: &[&str] = &[];
/// const MESSAGE3: &str = const_str::join!(EMPTY, ", ");
/// assert_eq!(MESSAGE3, "");
/// ```
#[macro_export]
macro_rules! join {
    ($strs: expr, $sep: expr) => {{
        const STRS: &[&str] = $strs;
        const SEP: &str = $sep;
        const OUTPUT_LEN: usize = $crate::__ctfe::Join(STRS, SEP).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::Join(STRS, SEP).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        const PROMPT: &str = "The answer is";
        const ANSWER: usize = 42;
        const MESSAGE: &str = concat!(PROMPT, " ", ANSWER);
        assert_eq!(MESSAGE, "The answer is 42");

        const S1: &str = concat!("hello", " ", "world");
        assert_eq!(S1, "hello world");

        const S2: &str = concat!("a", "b", "c");
        assert_eq!(S2, "abc");

        const S3: &str = concat!("single");
        assert_eq!(S3, "single");

        const S4: &str = concat!("");
        assert_eq!(S4, "");

        const S5: &str = concat!(true, " ", false);
        assert_eq!(S5, "true false");

        const S6: &str = concat!('a', 'b', 'c');
        assert_eq!(S6, "abc");
    }

    #[test]
    fn test_concat_runtime() {
        // Runtime tests to improve coverage
        let strs = &["hello", "world"];
        let concat = Concat(strs);
        assert_eq!(concat.output_len(), 10);

        let buf: StrBuf<10> = concat.const_eval();
        assert_eq!(buf.as_str(), "helloworld");

        let empty: &[&str] = &[];
        let concat_empty = Concat(empty);
        assert_eq!(concat_empty.output_len(), 0);

        let single = &["test"];
        let concat_single = Concat(single);
        assert_eq!(concat_single.output_len(), 4);
    }

    #[test]
    fn test_join() {
        const WORDS: &[&str] = &["hello", "world"];
        const MESSAGE1: &str = join!(WORDS, " ");
        assert_eq!(MESSAGE1, "hello world");

        const NUMS: &[&str] = &["1", "2", "3"];
        const MESSAGE2: &str = join!(NUMS, ", ");
        assert_eq!(MESSAGE2, "1, 2, 3");

        const EMPTY: &[&str] = &[];
        const MESSAGE3: &str = join!(EMPTY, ", ");
        assert_eq!(MESSAGE3, "");

        const SINGLE: &[&str] = &["alone"];
        const MESSAGE4: &str = join!(SINGLE, ", ");
        assert_eq!(MESSAGE4, "alone");

        const MULTI: &[&str] = &["a", "b", "c", "d"];
        const MESSAGE5: &str = join!(MULTI, "-");
        assert_eq!(MESSAGE5, "a-b-c-d");
    }

    #[test]
    fn test_join_runtime() {
        // Runtime tests to improve coverage
        let strs = &["hello", "world"];
        let join = Join(strs, " ");
        assert_eq!(join.output_len(), 11);

        let buf: StrBuf<11> = join.const_eval();
        assert_eq!(buf.as_str(), "hello world");

        let empty: &[&str] = &[];
        let join_empty = Join(empty, ", ");
        assert_eq!(join_empty.output_len(), 0);

        let single = &["test"];
        let join_single = Join(single, ", ");
        assert_eq!(join_single.output_len(), 4);

        let multi = &["a", "b", "c"];
        let join_multi = Join(multi, "-");
        assert_eq!(join_multi.output_len(), 5); // "a-b-c"
    }
}

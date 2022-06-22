#![allow(unsafe_code)]

use super::str_buf::StrBuf;

pub struct Replace<'input, 'to, P>(pub &'input str, pub P, pub &'to str);

impl<'input, 'from, 'to> Replace<'input, 'to, &'from str> {
    pub const fn output_len(&self) -> usize {
        let input = self.0.as_bytes();
        let replace_from = self.1.as_bytes();
        let replace_to = self.2.as_bytes();

        let input_len = input.len();
        let replace_from_len = replace_from.len();
        let replace_to_len = replace_to.len();

        if input_len == 0 {
            if replace_from_len == 0 {
                return replace_to_len;
            } else {
                return 0;
            }
        }

        if replace_from_len == 0 {
            let input_chars_count = crate::utf8::str_count_chars(self.0);
            return input_len + (input_chars_count + 1) * replace_to_len;
        }

        let mut ans = 0;

        let mut i = 0;
        while i < input_len {
            let mut j = 0;
            while j < replace_from_len && i + j < input_len {
                if input[i + j] == replace_from[j] {
                    j += 1;
                } else {
                    break;
                }
            }
            if j == replace_from_len {
                ans += replace_to_len;
                i += j;
            } else {
                ans += 1;
                i += 1;
            }
        }
        ans
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let input = self.0.as_bytes();
        let replace_from = self.1.as_bytes();
        let replace_to = self.2.as_bytes();

        let input_len = input.len();
        let replace_from_len = replace_from.len();
        let replace_to_len = replace_to.len();

        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x: expr) => {{
                buf[pos] = $x;
                pos += 1;
            }};
        }

        if input_len == 0 {
            if replace_from_len == 0 {
                let mut k = 0;
                while k < replace_to_len {
                    push!(replace_to[k]);
                    k += 1;
                }
            }
            constfn_assert!(pos == N);
            return unsafe { StrBuf::new_unchecked(buf) };
        }

        if replace_from_len == 0 {
            let mut s = input;
            loop {
                let mut k = 0;
                while k < replace_to_len {
                    push!(replace_to[k]);
                    k += 1;
                }
                match crate::utf8::next_code_point(s) {
                    Some((_, count)) => {
                        let mut i = 0;
                        while i < count {
                            push!(s[i]);
                            i += 1;
                        }
                        s = crate::bytes::advance(s, count);
                    }
                    None => break,
                }
            }
            constfn_assert!(pos == N);
            return unsafe { StrBuf::new_unchecked(buf) };
        }

        let mut i = 0;
        while i < input_len {
            let mut j = 0;
            while j < replace_from_len && i + j < input_len {
                if input[i + j] == replace_from[j] {
                    j += 1;
                } else {
                    break;
                }
            }
            if j == replace_from_len {
                let mut k = 0;
                while k < replace_to_len {
                    push!(replace_to[k]);
                    k += 1;
                }
                i += j;
            } else {
                push!(input[i]);
                i += 1;
            }
        }
        constfn_assert!(pos == N);
        unsafe { StrBuf::new_unchecked(buf) }
    }
}

#[test]
fn test_replace() {
    macro_rules! test_replace_str {
        ($input: expr, $replace_from: expr, $replace_to: expr) => {{
            const INPUT: &str = $input;
            const REPLACE_FROM: &str = $replace_from;
            const REPLACE_TO: &str = $replace_to;

            const CONSTFN: Replace<'static, 'static, &str> =
                Replace(INPUT, REPLACE_FROM, REPLACE_TO);
            const OUTPUT_LEN: usize = CONSTFN.output_len();

            let ans = INPUT.replace(REPLACE_FROM, REPLACE_TO);
            assert_eq!(OUTPUT_LEN, ans.len());

            let output_buf = CONSTFN.const_eval::<OUTPUT_LEN>();
            let output = output_buf.as_str();
            assert_eq!(output, ans);
        }};
    }

    test_replace_str!("", "", "");
    test_replace_str!("", "", "a");
    test_replace_str!("", "a", "");
    test_replace_str!("", "a", "b");
    test_replace_str!("a", "", "b");
    test_replace_str!("asd", "", "b");
    test_replace_str!("aba", "a", "c");
    test_replace_str!("this is old", "old", "new");
    test_replace_str!("我", "", "1");
    test_replace_str!("我", "", "我");
}

/// Replaces all matches of a pattern with another string slice.
///
/// See [`str::replace`](https://doc.rust-lang.org/std/primitive.str.html#method.replace).
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
        $crate::__strbuf_as_str!(&OUTPUT_BUF)
    }};
}

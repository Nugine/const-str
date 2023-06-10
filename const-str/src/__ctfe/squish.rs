#![allow(unsafe_code)]

use crate::__ctfe::StrBuf;

pub struct Squish<T>(pub T);

impl Squish<&'_ str> {
    pub const fn output_len(&self) -> usize {
        let mut len = 0;

        macro_rules! push {
            ($x: expr) => {
                len += 1;
            };
        }

        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let x = bytes[i];

            if x.is_ascii_whitespace() {
                let mut j = i + 1;
                while j < bytes.len() {
                    if bytes[j].is_ascii_whitespace() {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if !(i == 0 || j == bytes.len()) {
                    push!(b' ');
                }
                i = j;
                continue;
            }

            push!(x);
            i += 1;
        }

        len
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x: expr) => {
                buf[pos] = $x;
                pos += 1;
            };
        }

        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let x = bytes[i];

            if x.is_ascii_whitespace() {
                let mut j = i + 1;
                while j < bytes.len() {
                    if bytes[j].is_ascii_whitespace() {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if !(i == 0 || j == bytes.len()) {
                    push!(b' ');
                }
                i = j;
                continue;
            }

            push!(x);
            i += 1;
        }

        assert!(pos == N);
        unsafe { StrBuf::new_unchecked(buf) }
    }
}

/// Splits the string by ASCII whitespaces, and then joins the parts with a single space.
///
/// # Examples
///
/// ```rust
/// use const_str::squish;
///
/// assert_eq!(squish!("   SQUISH  \t THAT  \t CAT!    "), "SQUISH THAT CAT!");
///
/// const SQL: &str = squish!(
///     "SELECT
///         name,
///         created_at,
///         updated_at
///     FROM users
///     WHERE id = ?"
/// );
/// assert_eq!(SQL, "SELECT name, created_at, updated_at FROM users WHERE id = ?");
///
///
/// ```
///
#[macro_export]
macro_rules! squish {
    ($s:expr) => {{
        const INPUT: &str = $s;
        const N: usize = $crate::__ctfe::Squish(INPUT).output_len();
        const OUTPUT: $crate::__ctfe::StrBuf<N> = $crate::__ctfe::Squish(INPUT).const_eval();
        OUTPUT.as_str()
    }};
}

#[cfg(test)]
mod tessts {
    fn join<'a>(iter: impl IntoIterator<Item = &'a str>, sep: &str) -> String {
        let mut ans = String::new();
        let mut iter = iter.into_iter();
        match iter.next() {
            None => return ans,
            Some(first) => ans.push_str(first),
        }
        for part in iter {
            ans.push_str(sep);
            ans.push_str(part);
        }
        ans
    }

    fn std_squish(input: &str) -> String {
        join(input.split_ascii_whitespace(), " ")
    }

    #[test]
    fn test_squish() {
        macro_rules! testcase {
            ($s:expr) => {{
                const OUTPUT: &str = squish!($s);
                let expected = std_squish($s);
                assert_eq!(OUTPUT, expected);
            }};
        }

        testcase!("");
        testcase!(" ");
        testcase!(" t");
        testcase!("t ");
        testcase!(" t ");
        testcase!(" t t");

        testcase!(" SQUISH \t THAT \t CAT ");

        testcase!(
            "
                All you need to know is to \t 
                SQUISH THAT CAT! \
            "
        );

        testcase!(concat!("We\n", "always\n", "SQUISH\n", "THAT\n", "CAT."));

        testcase!(
            "SELECT 
                name, 
                created_at, 
                updated_at 
            FROM users 
            WHERE id = ?"
        );
    }
}

use crate::utils;

pub struct Len<T>(pub T);

impl Len<&str> {
    pub const fn const_eval(&self) -> usize {
        self.0.len()
    }
}

impl<const L: usize> Len<&[u8; L]> {
    pub const fn const_eval(&self) -> usize {
        L
    }
}

pub struct ToByteArray<T>(pub T);

impl ToByteArray<&str> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(self.0.len() == N);
        let mut buf = [0; N];
        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            buf[i] = bytes[i];
            i += 1;
        }
        buf
    }
}

impl<const L: usize> ToByteArray<&[u8; L]> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(L == N);
        let mut buf = [0; N];
        let bytes: &[u8] = self.0;
        let mut i = 0;
        while i < bytes.len() {
            buf[i] = bytes[i];
            i += 1;
        }
        buf
    }
}

pub enum AsciiCase {
    Lower,
    Upper,
}

pub struct MapAsciiCase<T>(pub T, pub AsciiCase);

impl MapAsciiCase<&str> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(self.0.len() == N);
        let mut buf = ToByteArray(self.0).const_eval::<N>();

        let mut i = 0;
        while i < buf.len() {
            buf[i] = match self.1 {
                AsciiCase::Lower => buf[i].to_ascii_lowercase(),
                AsciiCase::Upper => buf[i].to_ascii_uppercase(),
            };
            i += 1;
        }

        buf
    }
}

pub struct Equal<T1, T2>(pub T1, pub T2);

impl Equal<&[u8], &[u8]> {
    pub const fn const_eval(&self) -> bool {
        let lhs: &[u8] = self.0;
        let rhs: &[u8] = self.1;
        if lhs.len() != rhs.len() {
            return false;
        }
        let mut i = 0;
        while i < lhs.len() {
            if lhs[i] != rhs[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl<const L1: usize, const L2: usize> Equal<&[u8; L1], &[u8; L2]> {
    pub const fn const_eval(&self) -> bool {
        let eq: Equal<&[u8], &[u8]> = Equal(self.0, self.1);
        eq.const_eval()
    }
}

impl Equal<&str, &str> {
    pub const fn const_eval(&self) -> bool {
        Equal(self.0.as_bytes(), self.1.as_bytes()).const_eval()
    }
}

pub struct Repeat<T>(pub T, pub usize);

impl Repeat<&str> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(self.0.len().checked_mul(self.1).is_some());
        const_assert!(self.0.len() * self.1 == N);
        let mut buf = [0; N];
        let bytes = self.0.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < self.1 {
            let mut k = 0;
            while k < bytes.len() {
                buf[j] = bytes[k];
                j += 1;
                k += 1;
            }
            i += 1;
        }
        buf
    }
}

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
            return input_len + (input_len + 1) * replace_to_len;
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

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let input = self.0.as_bytes();
        let replace_from = self.1.as_bytes();
        let replace_to = self.2.as_bytes();

        let input_len = input.len();
        let replace_from_len = replace_from.len();
        let replace_to_len = replace_to.len();

        let mut buf = [0; N];
        let mut pos = 0;

        if input_len == 0 {
            if replace_from_len == 0 {
                let mut k = 0;
                while k < replace_to_len {
                    buf[pos] = replace_to[k];
                    pos += 1;
                    k += 1;
                }
            }
            return buf;
        }

        if replace_from_len == 0 {
            let mut i = 0;
            loop {
                let mut k = 0;
                while k < replace_to_len {
                    buf[pos] = replace_to[k];
                    pos += 1;
                    k += 1;
                }
                if i < input_len {
                    buf[pos] = input[i];
                    pos += 1;
                    i += 1;
                } else {
                    break;
                }
            }
            return buf;
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
                    buf[pos] = replace_to[k];
                    pos += 1;
                    k += 1;
                }
                i += j;
            } else {
                buf[pos] = input[i];
                pos += 1;
                i += 1;
            }
        }
        buf
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

            let output_bytes = CONSTFN.const_eval::<OUTPUT_LEN>();
            let output = core::str::from_utf8(&output_bytes).unwrap();
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
}

pub struct ToStr<T>(pub T);

impl ToStr<&str> {
    pub const fn output_len(&self) -> usize {
        self.0.len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        ToByteArray(self.0).const_eval()
    }
}

impl ToStr<bool> {
    pub const fn output_len(&self) -> usize {
        if self.0 {
            4
        } else {
            5
        }
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let buf = [0; N];
        let bytes: &[u8] = if self.0 { b"true" } else { b"false" };
        utils::merge_bytes(buf, bytes, bytes.len())
    }
}

impl ToStr<char> {
    pub const fn output_len(&self) -> usize {
        self.0.len_utf8()
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let (utf8_buf, len) = utils::encode_utf8(self.0);
        utils::merge_bytes([0; N], &utf8_buf, len)
    }
}

macro_rules! impl_integer_to_str {
    ($unsigned: ty, $signed: ty) => {
        impl ToStr<$unsigned> {
            pub const fn output_len(&self) -> usize {
                let mut x = self.0;
                let mut ans = 1;
                while x > 9 {
                    ans += 1;
                    x /= 10;
                }
                ans
            }

            pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
                let mut buf = [0; N];
                let mut pos = 0;
                let mut x = self.0;
                loop {
                    buf[pos] = b'0' + (x % 10) as u8;
                    pos += 1;
                    x /= 10;
                    if x == 0 {
                        break;
                    }
                }
                const_assert!(pos == N);
                utils::reversed_bytes(buf)
            }
        }

        impl ToStr<$signed> {
            pub const fn output_len(&self) -> usize {
                let x = self.0;
                let abs_len = ToStr(x.unsigned_abs()).output_len();
                abs_len + (x < 0) as usize
            }

            pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
                let mut buf = [0; N];
                let mut pos = 0;

                let mut x = self.0.unsigned_abs();

                loop {
                    buf[pos] = b'0' + (x % 10) as u8;
                    pos += 1;
                    x /= 10;
                    if x == 0 {
                        break;
                    }
                }

                if self.0 < 0 {
                    buf[pos] = b'-';
                    pos += 1;
                }

                const_assert!(pos == N);
                utils::reversed_bytes(buf)
            }
        }
    };
}

impl_integer_to_str!(u8, i8);
impl_integer_to_str!(u16, i16);
impl_integer_to_str!(u32, i32);
impl_integer_to_str!(u64, i64);
impl_integer_to_str!(u128, i128);
impl_integer_to_str!(usize, isize);

#[test]
fn test_to_str() {
    extern crate alloc;
    use alloc::string::ToString;

    macro_rules! test_to_str {
        ($ty: ty, $x: expr) => {{
            const X: $ty = $x;
            const OUTPUT_LEN: usize = ToStr(X).output_len();
            const OUTPUT_BYTES: [u8; OUTPUT_LEN] = ToStr(X).const_eval();

            let output = core::str::from_utf8(&OUTPUT_BYTES).unwrap();
            let ans = X.to_string();
            assert_eq!(OUTPUT_LEN, ans.len());
            assert_eq!(output, ans);
        }};
    }

    test_to_str!(&str, "lovelive superstar");

    test_to_str!(bool, true);
    test_to_str!(bool, false);

    test_to_str!(char, '鲤');
    test_to_str!(char, '鱼');

    test_to_str!(u8, 0);
    test_to_str!(u16, 0);
    test_to_str!(u32, 0);
    test_to_str!(u64, 0);
    test_to_str!(u128, 0);

    test_to_str!(u8, 10);
    test_to_str!(u8, 128);
    test_to_str!(u8, u8::MAX);

    test_to_str!(u64, 1);
    test_to_str!(u64, 10);
    test_to_str!(u64, 42);
    test_to_str!(u64, u64::MAX);

    test_to_str!(u128, u128::MAX);

    test_to_str!(i8, 0);
    test_to_str!(i16, 0);
    test_to_str!(i32, 0);
    test_to_str!(i64, 0);
    test_to_str!(i128, 0);

    test_to_str!(i8, -10);
    test_to_str!(i8, -42);
    test_to_str!(i8, i8::MAX);
    test_to_str!(i8, i8::MIN);

    test_to_str!(i64, 1);
    test_to_str!(i64, 10);
    test_to_str!(i64, -42);
    test_to_str!(i64, i64::MAX);
    test_to_str!(i64, i64::MIN);

    test_to_str!(i128, i128::MAX);
    test_to_str!(i128, i128::MIN);
}

pub struct Concat<'a>(pub &'a [&'a str]);

impl<'a> Concat<'a> {
    pub const fn output_len(&self) -> usize {
        let mut ans = 0;
        let mut iter = self.0;
        while let [x, xs @ ..] = iter {
            ans += x.len();
            iter = xs;
        }
        ans
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
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
        const_assert!(pos == N);

        buf
    }
}

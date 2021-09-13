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

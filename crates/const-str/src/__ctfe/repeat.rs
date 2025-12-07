#![allow(unsafe_code)]

use super::StrBuf;

pub struct Repeat<T>(pub T, pub usize);

impl Repeat<&str> {
    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let buf = bytes_repeat(self.0.as_bytes(), self.1);
        unsafe { StrBuf::new_unchecked(buf) }
    }
}

const fn bytes_repeat<const N: usize>(bytes: &[u8], n: usize) -> [u8; N] {
    assert!(bytes.len().checked_mul(n).is_some());
    assert!(bytes.len() * n == N);
    let mut buf = [0; N];
    let mut i = 0;
    let mut j = 0;
    while i < n {
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

/// Creates a new string slice by repeating a string slice n times.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
///
/// ```
/// const S: &str = "abc";
/// const SSSS: &str = const_str::repeat!(S, 4);
/// assert_eq!(SSSS, "abcabcabcabc");
/// ```
///
#[macro_export]
macro_rules! repeat {
    ($s: expr, $n: expr) => {{
        const INPUT: &str = $s;
        const N: usize = $n;
        const OUTPUT_LEN: usize = INPUT.len() * N;
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::Repeat(INPUT, N).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        const S1: &str = "abc";
        const R1: &str = repeat!(S1, 4);
        assert_eq!(R1, "abcabcabcabc");

        const S2: &str = "x";
        const R2: &str = repeat!(S2, 5);
        assert_eq!(R2, "xxxxx");

        const S3: &str = "hello";
        const R3: &str = repeat!(S3, 2);
        assert_eq!(R3, "hellohello");

        const S4: &str = "test";
        const R4: &str = repeat!(S4, 0);
        assert_eq!(R4, "");

        const S5: &str = "test";
        const R5: &str = repeat!(S5, 1);
        assert_eq!(R5, "test");
    }

    #[test]
    fn test_repeat_runtime() {
        // Runtime tests for Repeat
        let repeat = Repeat("abc", 3);
        let buf: StrBuf<9> = repeat.const_eval();
        assert_eq!(buf.as_str(), "abcabcabc");

        let repeat_single = Repeat("x", 10);
        let buf2: StrBuf<10> = repeat_single.const_eval();
        assert_eq!(buf2.as_str(), "xxxxxxxxxx");

        let repeat_zero = Repeat("test", 0);
        let buf3: StrBuf<0> = repeat_zero.const_eval();
        assert_eq!(buf3.as_str(), "");

        // Test bytes_repeat directly
        let bytes = b"hi";
        let result: [u8; 6] = bytes_repeat(bytes, 3);
        assert_eq!(&result, b"hihihi");

        let result2: [u8; 0] = bytes_repeat(b"x", 0);
        assert_eq!(&result2, b"");
    }
}

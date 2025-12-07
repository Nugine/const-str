use crate::slice::advance;
use crate::utf16::CharEncodeUtf16;

pub struct Utf8Encoder {
    pub nul_terminated: bool,
}

pub struct Utf16Encoder {
    pub nul_terminated: bool,
}

pub struct Encode<'a, T>(pub &'a str, pub T);

impl Encode<'_, Utf8Encoder> {
    pub const fn output_len(&self) -> usize {
        if self.1.nul_terminated {
            self.0.len() + 1
        } else {
            self.0.len()
        }
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let bytes = self.0.as_bytes();
        if self.1.nul_terminated {
            let mut buf = [0; N];
            let mut i = 0;
            while i < bytes.len() {
                let b = bytes[i];
                assert!(b != 0);
                buf[i] = b;
                i += 1;
            }
            assert!(i + 1 == N);
            buf
        } else {
            crate::bytes::clone(bytes)
        }
    }
}

impl Encode<'_, Utf16Encoder> {
    pub const fn output_len(&self) -> usize {
        crate::utf16::str_len_utf16(self.0) + (self.1.nul_terminated as usize)
    }

    pub const fn const_eval<const N: usize>(&self) -> [u16; N] {
        let mut s = self.0.as_bytes();

        let mut buf = [0; N];
        let mut pos = 0;

        while let Some((code, count)) = crate::utf8::next_char(s) {
            s = advance(s, count);
            let e = CharEncodeUtf16::new(code);

            buf[pos] = e.first();
            pos += 1;

            if e.has_second() {
                buf[pos] = e.second();
                pos += 1;
            }

            if self.1.nul_terminated {
                assert!(buf[pos - 1] != 0);
                if e.has_second() {
                    assert!(buf[pos - 2] != 0);
                }
            }
        }

        if self.1.nul_terminated {
            pos += 1;
        }

        assert!(pos == N);

        buf
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __encoder {
    (utf8) => {{
        $crate::__ctfe::Utf8Encoder {
            nul_terminated: false,
        }
    }};
    (utf8_z) => {{
        $crate::__ctfe::Utf8Encoder {
            nul_terminated: true,
        }
    }};
    (utf16) => {{
        $crate::__ctfe::Utf16Encoder {
            nul_terminated: false,
        }
    }};
    (utf16_z) => {{
        $crate::__ctfe::Utf16Encoder {
            nul_terminated: true,
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __encode {
    ($e: tt, $s: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::Encode($s, $crate::__encoder!($e)).output_len();
        &{ $crate::__ctfe::Encode($s, $crate::__encoder!($e)).const_eval::<OUTPUT_LEN>() }
    }};
}

/// Encode a string slice with a specified encoding.
///
/// Supported encodings:
///
/// + utf8
/// + utf16
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
/// ``` rust
/// use const_str::encode;
///
/// const S: &str = "hello你好";
///
/// const S_UTF8: &[u8] = encode!(utf8, S);
/// assert_eq!(S_UTF8, [104, 101, 108, 108, 111, 228, 189, 160, 229, 165, 189]);
///
/// const S_UTF16: &[u16] = encode!(utf16, S);
/// assert_eq!(S_UTF16, [104, 101, 108, 108, 111, 20320, 22909]);
/// ```
///
#[macro_export]
macro_rules! encode {
    (utf8, $s: expr) => {
        $crate::__encode!(utf8, $s)
    };
    (utf16, $s: expr) => {
        $crate::__encode!(utf16, $s)
    };
}

/// Encode a string slice with a specified encoding and append a nul character.
///
/// The provided data should not contain any nul bytes in it.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`const_str::encode!`][crate::encode]
///
/// # Examples
/// ``` rust
/// use const_str::encode_z;
///
/// const S: &str = "hello你好";
///
/// const S_UTF8_Z: &[u8] = encode_z!(utf8, S);
/// assert_eq!(S_UTF8_Z, [104, 101, 108, 108, 111, 228, 189, 160, 229, 165, 189, 0]);
///
/// const S_UTF16_Z: &[u16] = encode_z!(utf16, S);
/// assert_eq!(S_UTF16_Z, [104, 101, 108, 108, 111, 20320, 22909, 0]);
/// ```
///
#[macro_export]
macro_rules! encode_z {
    (utf8, $s: expr) => {
        $crate::__encode!(utf8_z, $s)
    };
    (utf16, $s: expr) => {
        $crate::__encode!(utf16_z, $s)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        {
            const S: &str = "abc你好";
            const B1: &[u8; 9] = encode!(utf8, S);
            const B2: &[u8] = encode!(utf8, S);
            const B3: &[u8; 10] = encode_z!(utf8, S);
            let mut ans = S.as_bytes().to_owned();
            assert_eq!(B1, ans.as_slice());
            assert_eq!(B2, B1);
            ans.push(0);
            assert_eq!(B3, ans.as_slice());
        }
        {
            const S: &str = "abc你好𤭢";
            const B1: &[u16; 7] = encode!(utf16, S);
            const B2: &[u16; 8] = encode_z!(utf16, S);
            let mut ans = S.encode_utf16().collect::<Vec<_>>();
            assert_eq!(B1, ans.as_slice());
            ans.push(0);
            assert_eq!(B2, ans.as_slice());
        }
    }

    #[test]
    fn test_encode_runtime() {
        // Runtime tests for Utf8Encoder
        let encoder_utf8 = Encode(
            "test",
            Utf8Encoder {
                nul_terminated: false,
            },
        );
        assert_eq!(encoder_utf8.output_len(), 4);
        let buf: [u8; 4] = encoder_utf8.const_eval();
        assert_eq!(&buf, b"test");

        let encoder_utf8_z = Encode(
            "hello",
            Utf8Encoder {
                nul_terminated: true,
            },
        );
        assert_eq!(encoder_utf8_z.output_len(), 6);
        let buf2: [u8; 6] = encoder_utf8_z.const_eval();
        assert_eq!(&buf2, b"hello\0");

        // Runtime tests for Utf16Encoder
        let encoder_utf16 = Encode(
            "abc",
            Utf16Encoder {
                nul_terminated: false,
            },
        );
        assert_eq!(encoder_utf16.output_len(), 3);
        let buf3: [u16; 3] = encoder_utf16.const_eval();
        assert_eq!(buf3, [b'a' as u16, b'b' as u16, b'c' as u16]);

        let encoder_utf16_z = Encode(
            "hi",
            Utf16Encoder {
                nul_terminated: true,
            },
        );
        assert_eq!(encoder_utf16_z.output_len(), 3);
        let buf4: [u16; 3] = encoder_utf16_z.const_eval();
        assert_eq!(buf4, [b'h' as u16, b'i' as u16, 0]);

        // Test with unicode
        let encoder_unicode = Encode(
            "你好",
            Utf16Encoder {
                nul_terminated: false,
            },
        );
        let len = encoder_unicode.output_len();
        assert_eq!(len, 2);

        // Test empty string
        let encoder_empty = Encode(
            "",
            Utf8Encoder {
                nul_terminated: false,
            },
        );
        assert_eq!(encoder_empty.output_len(), 0);
    }
}

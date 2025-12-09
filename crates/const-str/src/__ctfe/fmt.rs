#![allow(unsafe_code)]

use super::StrBuf;
use super::ToStr;

use crate::slice::advance;
use crate::utf8::CharEscapeDebug;
use crate::utf8::CharEscapeDebugArgs;

#[derive(Clone, Copy)]
pub struct FmtSpec {
    pub alternate: bool,
}

pub struct Display<T>(pub T, pub FmtSpec);

macro_rules! delegate_display {
    ($($ty: ty,)+) => {
        $(
            impl Display<$ty> {
                pub const fn output_len(&self) -> usize {
                    ToStr(self.0).output_len()
                }

                pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                    ToStr(self.0).const_eval()
                }
            }
        )+
    };
}

delegate_display!(&str, char, bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize,);

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_display {
    ($x: expr, $spec: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::Display($x, $spec).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::Display($x, $spec).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

pub struct Debug<T>(pub T, pub FmtSpec);

macro_rules! delegate_debug {
    ($($ty: ty,)+) => {
        $(
            impl Debug<$ty> {
                pub const fn output_len(&self) -> usize {
                    ToStr(self.0).output_len()
                }

                pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                    ToStr(self.0).const_eval()
                }
            }
        )+
    };
}

delegate_debug!(bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize,);

impl Debug<char> {
    pub const fn output_len(&self) -> usize {
        let escape = CharEscapeDebug::new(
            self.0,
            CharEscapeDebugArgs {
                escape_single_quote: true,
                escape_double_quote: false,
            },
        );

        escape.as_bytes().len() + 2
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x: expr) => {{
                buf[pos] = $x;
                pos += 1;
            }};
        }

        push!(b'\'');
        {
            let e = CharEscapeDebug::new(
                self.0,
                CharEscapeDebugArgs {
                    escape_single_quote: true,
                    escape_double_quote: false,
                },
            );
            let bytes = e.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                push!(bytes[i]);
                i += 1;
            }
        }
        push!(b'\'');

        assert!(pos == N);

        unsafe { StrBuf::new_unchecked(buf) }
    }
}

impl Debug<&str> {
    pub const fn output_len(&self) -> usize {
        let mut s = self.0.as_bytes();
        let mut ans = 2;
        while let Some((ch, count)) = crate::utf8::next_char(s) {
            s = advance(s, count);
            let e = CharEscapeDebug::new(
                ch,
                CharEscapeDebugArgs {
                    escape_single_quote: false,
                    escape_double_quote: true,
                },
            );
            ans += e.as_bytes().len()
        }
        ans
    }

    pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x: expr) => {{
                buf[pos] = $x;
                pos += 1;
            }};
        }

        push!(b'"');

        let mut s = self.0.as_bytes();
        while let Some((ch, count)) = crate::utf8::next_char(s) {
            s = advance(s, count);
            let e = CharEscapeDebug::new(
                ch,
                CharEscapeDebugArgs {
                    escape_single_quote: false,
                    escape_double_quote: true,
                },
            );
            let bytes = e.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                push!(bytes[i]);
                i += 1;
            }
        }

        push!(b'"');

        assert!(pos == N);

        unsafe { StrBuf::new_unchecked(buf) }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_debug {
    ($x: expr, $spec: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::Debug($x, $spec).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::Debug($x, $spec).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

struct Hex<T>(T, FmtSpec, bool);

pub struct LowerHex<T>(pub T, pub FmtSpec);
pub struct UpperHex<T>(pub T, pub FmtSpec);

macro_rules! impl_integer_hex {
    ($unsigned: ty, $signed: ty) => {
        impl Hex<$unsigned> {
            const fn output_len(&self) -> usize {
                let mut x = self.0;
                let mut ans = 0;
                loop {
                    ans += 1;
                    x /= 16;
                    if x == 0 {
                        break;
                    }
                }
                if self.1.alternate {
                    ans += 2;
                }
                ans
            }

            const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let mut buf = [0; N];
                let mut pos = 0;
                let mut x = self.0;
                loop {
                    let d = crate::ascii::num_to_hex_digit((x % 16) as u8);
                    buf[pos] = if self.2 { d.to_ascii_uppercase() } else { d };
                    pos += 1;
                    x /= 16;
                    if x == 0 {
                        break;
                    }
                }
                if self.1.alternate {
                    buf[pos] = b'x';
                    pos += 1;
                    buf[pos] = b'0';
                    pos += 1;
                }
                assert!(pos == N);
                let buf = crate::bytes::reversed(buf);
                unsafe { StrBuf::new_unchecked(buf) }
            }
        }

        impl LowerHex<$unsigned> {
            pub const fn output_len(&self) -> usize {
                let h = Hex(self.0, self.1, false);
                h.output_len()
            }

            pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let h = Hex(self.0, self.1, false);
                h.const_eval()
            }
        }

        impl UpperHex<$unsigned> {
            pub const fn output_len(&self) -> usize {
                let h = Hex(self.0, self.1, true);
                h.output_len()
            }

            pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let h = Hex(self.0, self.1, true);
                h.const_eval()
            }
        }

        impl LowerHex<$signed> {
            pub const fn output_len(&self) -> usize {
                let h = Hex(self.0 as $unsigned, self.1, false);
                h.output_len()
            }

            pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let h = Hex(self.0 as $unsigned, self.1, false);
                h.const_eval()
            }
        }

        impl UpperHex<$signed> {
            pub const fn output_len(&self) -> usize {
                let h = Hex(self.0 as $unsigned, self.1, true);
                h.output_len()
            }

            pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let h = Hex(self.0 as $unsigned, self.1, true);
                h.const_eval()
            }
        }
    };
}

impl_integer_hex!(u8, i8);
impl_integer_hex!(u16, i16);
impl_integer_hex!(u32, i32);
impl_integer_hex!(u64, i64);
impl_integer_hex!(u128, i128);
impl_integer_hex!(usize, isize);

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_lowerhex {
    ($x: expr, $spec: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::LowerHex($x, $spec).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::LowerHex($x, $spec).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_upperhex {
    ($x: expr, $spec: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::UpperHex($x, $spec).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::UpperHex($x, $spec).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

pub struct Binary<T>(pub T, pub FmtSpec);

macro_rules! impl_integer_binary {
    ($unsigned: ty, $signed: ty) => {
        impl Binary<$unsigned> {
            pub const fn output_len(&self) -> usize {
                let mut x = self.0;
                let mut ans = 0;
                loop {
                    ans += 1;
                    x /= 2;
                    if x == 0 {
                        break;
                    }
                }
                if self.1.alternate {
                    ans += 2;
                }
                ans
            }

            pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let mut buf = [0; N];
                let mut pos = 0;
                let mut x = self.0;
                loop {
                    buf[pos] = b'0' + (x % 2) as u8;
                    pos += 1;
                    x /= 2;
                    if x == 0 {
                        break;
                    }
                }
                if self.1.alternate {
                    buf[pos] = b'b';
                    pos += 1;
                    buf[pos] = b'0';
                    pos += 1;
                }
                assert!(pos == N);
                let buf = crate::bytes::reversed(buf);
                unsafe { StrBuf::new_unchecked(buf) }
            }
        }

        impl Binary<$signed> {
            pub const fn output_len(&self) -> usize {
                let b = Binary(self.0 as $unsigned, self.1);
                b.output_len()
            }

            pub const fn const_eval<const N: usize>(&self) -> StrBuf<N> {
                let b = Binary(self.0 as $unsigned, self.1);
                b.const_eval()
            }
        }
    };
}

impl_integer_binary!(u8, i8);
impl_integer_binary!(u16, i16);
impl_integer_binary!(u32, i32);
impl_integer_binary!(u64, i64);
impl_integer_binary!(u128, i128);
impl_integer_binary!(usize, isize);

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_binary {
    ($x: expr, $spec: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::Binary($x, $spec).output_len();
        const OUTPUT_BUF: $crate::__ctfe::StrBuf<OUTPUT_LEN> =
            $crate::__ctfe::Binary($x, $spec).const_eval();
        OUTPUT_BUF.as_str()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_runtime() {
        let spec = FmtSpec { alternate: false };

        // Test Display for various types
        let display_str = Display("hello", spec);
        assert_eq!(display_str.output_len(), 5);
        let buf: StrBuf<5> = display_str.const_eval();
        assert_eq!(buf.as_str(), "hello");

        let display_char = Display('A', spec);
        assert_eq!(display_char.output_len(), 1);
        let buf_char: StrBuf<1> = display_char.const_eval();
        assert_eq!(buf_char.as_str(), "A");

        let display_bool = Display(true, spec);
        assert_eq!(display_bool.output_len(), 4);
        let buf_bool: StrBuf<4> = display_bool.const_eval();
        assert_eq!(buf_bool.as_str(), "true");

        let display_u8 = Display(42u8, spec);
        assert_eq!(display_u8.output_len(), 2);
        let buf_u8: StrBuf<2> = display_u8.const_eval();
        assert_eq!(buf_u8.as_str(), "42");

        let display_i32 = Display(-123i32, spec);
        assert_eq!(display_i32.output_len(), 4);
        let buf_i32: StrBuf<4> = display_i32.const_eval();
        assert_eq!(buf_i32.as_str(), "-123");

        // Test more integer types
        let display_u16 = Display(999u16, spec);
        let buf_u16: StrBuf<3> = display_u16.const_eval();
        assert_eq!(buf_u16.as_str(), "999");

        let display_i8 = Display(-99i8, spec);
        let buf_i8: StrBuf<3> = display_i8.const_eval();
        assert_eq!(buf_i8.as_str(), "-99");

        let display_u64 = Display(123456u64, spec);
        let buf_u64: StrBuf<6> = display_u64.const_eval();
        assert_eq!(buf_u64.as_str(), "123456");

        let display_i64 = Display(-999i64, spec);
        let buf_i64: StrBuf<4> = display_i64.const_eval();
        assert_eq!(buf_i64.as_str(), "-999");
    }

    #[test]
    fn test_debug_runtime() {
        let spec = FmtSpec { alternate: false };

        // Test Debug for str
        let debug_str = Debug("test", spec);
        assert_eq!(debug_str.output_len(), 6); // "test" with quotes
        let buf: StrBuf<6> = debug_str.const_eval();
        assert_eq!(buf.as_str(), "\"test\"");

        // Test Debug for char
        let debug_char = Debug('a', spec);
        assert_eq!(debug_char.output_len(), 3); // 'a' with quotes
        let buf2: StrBuf<3> = debug_char.const_eval();
        assert_eq!(buf2.as_str(), "'a'");

        // Test Debug for special chars
        let debug_newline = Debug('\n', spec);
        assert!(debug_newline.output_len() > 2);

        // Test Debug for numeric types
        let debug_u8 = Debug(42u8, spec);
        assert_eq!(debug_u8.output_len(), 2);
        let buf_u8: StrBuf<2> = debug_u8.const_eval();
        assert_eq!(buf_u8.as_str(), "42");

        // Test Debug for more integer types
        let debug_i32 = Debug(-5i32, spec);
        assert_eq!(debug_i32.output_len(), 2);
        let buf_i32: StrBuf<2> = debug_i32.const_eval();
        assert_eq!(buf_i32.as_str(), "-5");
        
        let debug_bool = Debug(true, spec);
        assert_eq!(debug_bool.output_len(), 4);
        let buf_bool: StrBuf<4> = debug_bool.const_eval();
        assert_eq!(buf_bool.as_str(), "true");

        // Test Debug with alternate formatting
        let spec_alt = FmtSpec { alternate: true };
        let debug_alt = Debug(42u8, spec_alt);
        assert_eq!(debug_alt.output_len(), 2);
        let buf_alt: StrBuf<2> = debug_alt.const_eval();
        assert_eq!(buf_alt.as_str(), "42");
    }

    #[test]
    fn test_lower_hex_runtime() {
        let spec = FmtSpec { alternate: false };
        let spec_alt = FmtSpec { alternate: true };

        // Test LowerHex for unsigned - with output_len
        let hex_u8 = LowerHex(255u8, spec);
        assert_eq!(hex_u8.output_len(), 2);
        let buf: StrBuf<2> = hex_u8.const_eval();
        assert_eq!(buf.as_str(), "ff");

        let hex_u8_alt = LowerHex(255u8, spec_alt);
        assert_eq!(hex_u8_alt.output_len(), 4);
        let buf_alt: StrBuf<4> = hex_u8_alt.const_eval();
        assert_eq!(buf_alt.as_str(), "0xff");

        // Test LowerHex for signed - now with const_eval
        let hex_i32 = LowerHex(-1i32, spec);
        assert_eq!(hex_i32.output_len(), 8);
        let buf_i32: StrBuf<8> = hex_i32.const_eval();
        assert_eq!(buf_i32.as_str(), "ffffffff");
        
        // Test LowerHex for signed with alternate
        let hex_i32_alt = LowerHex(-1i32, spec_alt);
        assert_eq!(hex_i32_alt.output_len(), 10);
        let buf_i32_alt: StrBuf<10> = hex_i32_alt.const_eval();
        assert_eq!(buf_i32_alt.as_str(), "0xffffffff");
    }

    #[test]
    fn test_upper_hex_runtime() {
        let spec = FmtSpec { alternate: false };
        let spec_alt = FmtSpec { alternate: true };

        // Test UpperHex for unsigned
        let hex_u8 = UpperHex(255u8, spec);
        let buf: StrBuf<2> = hex_u8.const_eval();
        assert_eq!(buf.as_str(), "FF");

        let hex_u8_alt = UpperHex(255u8, spec_alt);
        let buf_alt: StrBuf<4> = hex_u8_alt.const_eval();
        assert_eq!(buf_alt.as_str(), "0xFF");

        // Test UpperHex for signed - now with const_eval
        let hex_i32 = UpperHex(-1i32, spec);
        assert_eq!(hex_i32.output_len(), 8);
        let buf_i32: StrBuf<8> = hex_i32.const_eval();
        assert_eq!(buf_i32.as_str(), "FFFFFFFF");

        // Test more integer types
        let hex_u16 = UpperHex(0xABCDu16, spec);
        let buf_u16: StrBuf<4> = hex_u16.const_eval();
        assert_eq!(buf_u16.as_str(), "ABCD");

        let hex_u64 = UpperHex(0x123456u64, spec);
        assert!(hex_u64.output_len() > 0);
    }

    #[test]
    fn test_binary_runtime() {
        let spec = FmtSpec { alternate: false };
        let spec_alt = FmtSpec { alternate: true };

        // Test Binary for unsigned
        let bin_u8 = Binary(5u8, spec);
        let buf: StrBuf<3> = bin_u8.const_eval();
        assert_eq!(buf.as_str(), "101");

        let bin_u8_alt = Binary(5u8, spec_alt);
        let buf_alt: StrBuf<5> = bin_u8_alt.const_eval();
        assert_eq!(buf_alt.as_str(), "0b101");

        // Test Binary for signed - now with const_eval
        let bin_i32 = Binary(-1i32, spec);
        assert_eq!(bin_i32.output_len(), 32);
        let buf_i32: StrBuf<32> = bin_i32.const_eval();
        // -1 in binary is all 1s
        assert_eq!(buf_i32.as_str(), "11111111111111111111111111111111");

        // Test more types
        let bin_u16 = Binary(7u16, spec);
        let buf_u16: StrBuf<3> = bin_u16.const_eval();
        assert_eq!(buf_u16.as_str(), "111");

        let bin_u64 = Binary(15u64, spec);
        let buf_u64: StrBuf<4> = bin_u64.const_eval();
        assert_eq!(buf_u64.as_str(), "1111");

        let bin_u128 = Binary(3u128, spec_alt);
        assert!(bin_u128.output_len() > 0);
    }
}

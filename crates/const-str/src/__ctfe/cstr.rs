#![allow(unsafe_code)]

pub struct ToCStr<T>(pub T);

impl ToCStr<&str> {
    const fn check_nul(&self) {
        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            assert!(bytes[i] != 0);
            i += 1;
        }
    }

    pub const fn output_len(&self) -> usize {
        self.check_nul();
        self.0.len() + 1
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0; N];
        let mut pos = 0;
        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            assert!(bytes[i] != 0);
            buf[pos] = bytes[i];
            pos += 1;
            i += 1;
        }
        pos += 1;
        assert!(pos == N);
        buf
    }
}

/// Converts a string slice to [`*const c_char`](core::ffi::c_char).
///
/// The C-style string is guaranteed to be terminated by a nul byte.
/// This trailing nul byte will be appended by this macro.
/// The provided data should not contain any nul bytes in it.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`cstr!`](crate::cstr)
///
/// # Examples
///
/// ```
/// use core::ffi::c_char;
/// const PRINTF_FMT: *const c_char = const_str::raw_cstr!("%d\n");
/// ```
#[macro_export]
macro_rules! raw_cstr {
    ($s: expr) => {
        $crate::cstr!($s).as_ptr()
    };
}

/// Converts a string slice to [`&CStr`](core::ffi::CStr).
///
/// The C-style string is guaranteed to be terminated by a nul byte.
/// This trailing nul byte will be appended by this macro.
/// The provided data should not contain any nul bytes in it.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// See also [`raw_cstr!`](crate::raw_cstr).
///
/// Note that Rust has supported [C string literals][c-str-literal] since [1.77.0][rust-1-77-0].
///
/// [c-str-literal]: https://doc.rust-lang.org/reference/tokens.html#c-string-and-raw-c-string-literals
/// [rust-1-77-0]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#c-string-literals
///
/// # Examples
///
/// ```
/// use core::ffi::CStr;;
/// const PRINTF_FMT: &CStr = const_str::cstr!("%d\n");
/// ```
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        const OUTPUT_LEN: ::core::primitive::usize = $crate::__ctfe::ToCStr($s).output_len();
        const OUTPUT_BUF: [u8; OUTPUT_LEN] = $crate::__ctfe::ToCStr($s).const_eval();
        const OUTPUT: &::core::ffi::CStr =
            unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(&OUTPUT_BUF) };
        OUTPUT
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_cstr() {
        const FMT: &str = "%d\n";
        let fmt = raw_cstr!(FMT);
        let len = FMT.len() + 1;
        let bytes: &[u8] = unsafe { core::slice::from_raw_parts(fmt.cast(), len) };
        assert_eq!(bytes, b"%d\n\0");
    }

    #[test]
    fn test_cstr_runtime() {
        // Runtime tests for ToCStr
        let to_cstr = ToCStr("hello");
        assert_eq!(to_cstr.output_len(), 6); // "hello" + '\0'

        let buf: [u8; 6] = to_cstr.const_eval();
        assert_eq!(&buf, b"hello\0");

        // Test empty string
        let to_cstr_empty = ToCStr("");
        assert_eq!(to_cstr_empty.output_len(), 1);
        let buf2: [u8; 1] = to_cstr_empty.const_eval();
        assert_eq!(&buf2, b"\0");

        // Test longer string
        let to_cstr_long = ToCStr("test string");
        assert_eq!(to_cstr_long.output_len(), 12);
        let buf3: [u8; 12] = to_cstr_long.const_eval();
        assert_eq!(&buf3, b"test string\0");
    }
}
